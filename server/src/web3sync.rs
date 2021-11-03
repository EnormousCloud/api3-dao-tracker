use jsonrpc_core::types::params::Params;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use web3::types::{Block, Filter, Log, H256, U256};

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
}
