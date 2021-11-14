use chrono::NaiveDateTime;
use client::fees::TxFee;
use jsonrpc_core::types::params::Params;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use web3::types::TransactionReceipt as Receipt;
use web3::types::{Block, Filter, Log, Transaction, H256, U256};

pub struct EthClient {
    rpc_addr: String,
    agent: ureq::Agent,
}

#[derive(Debug, Clone, Deserialize)]
struct EvmNumericResult {
    pub result: U256,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcSingleResponse<T> {
    pub id: serde_json::Value,
    pub result: T,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcId {
    pub id: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
struct RpcErrorResponse {
    pub id: serde_json::Value,
    pub error: RpcError,
}

#[derive(Debug, Clone, Serialize)]
struct RpcSingleRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Params,
}

type RpcBatchRequest = Vec<RpcSingleRequest>;
type RpcBatchResponse = Vec<Value>;

pub fn batch_fragment<T>(response: &RpcBatchResponse, id_match: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    for v in response {
        let id = v.as_object().unwrap().get("id").unwrap().as_str();
        if let Some(id_val) = id {
            if id_val == id_match {
                let s = serde_json::to_string(&v).unwrap();
                if let Ok(err) = serde_json::from_str::<RpcErrorResponse>(&s) {
                    return Err(anyhow::Error::msg(err.error.message));
                }
                let out: RpcSingleResponse<T> = serde_json::from_str(&s).unwrap();
                return Ok(out.result);
            }
        }
    }
    Err(anyhow::Error::msg("result not found in the batch"))
}

impl EthClient {
    pub fn new(rpc_addr: &str) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(60))
            .timeout_write(Duration::from_secs(5))
            .build();
        EthClient {
            agent,
            rpc_addr: rpc_addr.to_string(),
        }
    }

    pub fn execute_str<T>(&self, payload: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let rq = self
            .agent
            .post(&self.rpc_addr)
            .set("Content-Type", "application/json");
        tracing::debug!("JSONRPC request={}", payload);
        let response: String = match rq.send_string(&payload) {
            Ok(x) => x.into_string().unwrap(),
            Err(e) => return Err(anyhow::Error::new(e)),
        };
        if let Ok(err) = serde_json::from_str::<RpcErrorResponse>(&response) {
            return Err(anyhow::Error::msg(err.error.message));
        }

        tracing::debug!("JSONRPC response={}", response);
        Ok(serde_json::from_str::<T>(&response).unwrap())
    }

    pub fn execute<T>(&self, method: &str, params: Params) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let payload = serde_json::to_string(&RpcSingleRequest {
            jsonrpc: "2.0".to_owned(),
            id: "1".to_owned(),
            method: method.to_string(),
            params: params.clone(),
        })
        .unwrap();
        self.execute_str(&payload)
    }

    pub fn new_filter(&self, filter: &Filter) -> anyhow::Result<U256> {
        let filter_str = serde_json::to_string(filter).expect("filter serialize failure");
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_newFilter\",\"params\":[{}],\"id\":\"1\"}}",
            filter_str
        );
        let res: RpcSingleResponse<U256> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn filter_changes(&self, filter_id: U256) -> anyhow::Result<Vec<Log>> {
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getFilterChanges\",\"params\":[\"0x{:x}\"],\"id\":\"1\"}}",
            filter_id
        );
        let res: RpcSingleResponse<Vec<Log>> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn block(&self, block_hash: H256) -> anyhow::Result<Block<H256>> {
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByHash\",\"params\":[\"{:?}\",false],\"id\":\"1\"}}",
            block_hash
        );
        let res: RpcSingleResponse<Block<H256>> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn transaction(&self, tx_hash: H256) -> anyhow::Result<Transaction> {
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionByHash\",\"params\":[\"{:?}\"],\"id\":\"1\"}}",
            tx_hash
        );
        let res: RpcSingleResponse<Transaction> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn receipt(&self, tx_hash: H256) -> anyhow::Result<Receipt> {
        let payload = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionReceipt\",\"params\":[\"{:?}\"],\"id\":\"1\"}}",
            tx_hash
        );
        let res: RpcSingleResponse<Receipt> = self.execute_str(&payload)?;
        Ok(res.result)
    }

    pub fn fees(&self, tx_hash: H256, dt: NaiveDateTime) -> anyhow::Result<TxFee> {
        let txh = Value::from(format!("{:?}", tx_hash));
        let rq1 = RpcSingleRequest {
            jsonrpc: "2.0".to_owned(),
            id: "hash".to_owned(),
            method: "eth_getTransactionByHash".to_owned(),
            params: Params::Array(vec![txh.clone()]),
        };
        let rq2 = RpcSingleRequest {
            jsonrpc: "2.0".to_owned(),
            id: "receipt".to_owned(),
            method: "eth_getTransactionReceipt".to_owned(),
            params: Params::Array(vec![txh.clone()]),
        };
        let batch: RpcBatchRequest = vec![rq1, rq2];
        let payload = serde_json::to_string(&batch).unwrap();
        let response: RpcBatchResponse = self.execute_str(&payload)?;
        let tx: Transaction = batch_fragment(&response, "hash").unwrap();
        let receipt: Receipt = batch_fragment(&response, "receipt").unwrap();
        Ok(TxFee::new(&tx, &receipt, dt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use hex_literal::hex;

    #[test]
    pub fn it_works() {
        let rpc_addr = "http://localhost:8545";
        let client = EthClient::new(rpc_addr);

        let time =
            NaiveDateTime::parse_from_str("2021-11-18 12:10:51", "%Y-%m-%d %H:%M:%S").unwrap();
        let tx_hash: H256 =
            hex!("38407b1df1d03632a9874c6ca304dbb55eeebdbe3af8d6478e7c07e405cecb41").into();
        let fees = client.fees(tx_hash, time).unwrap();
        println!("fees {:?}", fees);
    }
}
