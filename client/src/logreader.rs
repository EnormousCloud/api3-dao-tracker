use std::str::FromStr;
use thiserror::Error;
use web3::types::{H160, H256, U256};

#[derive(Error, Debug, Clone)]
pub enum EventParseError {
    #[error("no topics")]
    NoTopics,
    #[error("{0} topics found, {1} expected")]
    InvalidTopics(usize, usize),
    #[error("{0} data length, {1} bytes expected")]
    InvalidDataSize(usize, usize),
}

pub struct LogReader {
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
    // mutable iterator for topics
    pub current_topic: usize,
    // mutable data offset
    pub data_offset: usize,
}

impl LogReader {
    pub fn new(
        log: &web3::types::Log,
        expected_topics: usize,
        expected_data_size: Option<usize>,
    ) -> Result<Self, EventParseError> {
        if log.topics.len() == 0 {
            return Err(EventParseError::NoTopics);
        }
        if log.topics.len() - 1 != expected_topics {
            return Err(EventParseError::InvalidTopics(
                log.topics.len() - 1,
                expected_topics,
            ));
        }
        let data: &Vec<u8> = &log.data.0;
        if let Some(sz) = expected_data_size {
            if data.len() != sz * 32 {
                return Err(EventParseError::InvalidDataSize(data.len(), sz));
            }
        }
        Ok(Self {
            current_topic: 1,
            data_offset: 0,
            topics: log.topics.clone(),
            data: data.clone(),
        })
    }

    fn has_topics(&self) -> bool {
        self.topics.len() > self.current_topic
    }

    fn has_data(&self) -> bool {
        self.data.len() > self.data_offset
    }

    fn next32(&mut self) -> String {
        if self.has_topics() {
            let hex_str = format!("{:?}", self.topics.get(self.current_topic).unwrap()); // string of 64
                                                                                         // println!("next topic={}", hex_str);
            self.current_topic += 1;
            hex_str[2..].to_owned()
        } else {
            let hex_str = hex::encode(&self.data);
            let offs: usize = 2 * self.data_offset;
            let res: String = hex_str.chars().skip(offs).take(64).collect();
            self.data_offset += 32;
            res
        }
    }

    // pop meta data as text
    pub fn text(&mut self) -> String {
        let hex_size = self.next32();
        let bytes: usize = usize::from_str_radix(&hex_size[48..], 16).unwrap();
        let mut s = String::with_capacity(bytes);
        let mut params: usize = bytes / 8;
        while params > 0 {
            let bts: Vec<u8> = hex::decode(self.next32()).unwrap();
            bts.iter().filter(|ch| **ch != 0).for_each(|ch| {
                if *ch == 0x1F {
                    s.push('|');
                } else {
                    s.push(*ch as char);
                }
            });
            params -= 1;
        }
        // println!("s={:?}", s);
        s
    }

    // pop address from the latest topic
    pub fn address(&mut self) -> H160 {
        let hex_str = self.next32();
        // println!("address={}", hex_str);
        H160::from_str(&hex_str[24..]).unwrap()
    }

    // pop array of addresses
    pub fn addresses(&mut self) -> Vec<H160> {
        let mut res = vec![];
        while self.has_data() {
            res.push(self.address());
        }
        res
    }

    // pop value from the latest topic or data
    pub fn value(&mut self) -> U256 {
        // let offs = self.data_offset;
        let hex_str = self.next32();
        // println!("offset={} data={}", offs, hex_str);
        U256::from_str(hex_str.as_str()).unwrap()
    }

    // pop bool value
    pub fn bool(&mut self) -> bool {
        let val = self.value().as_u64();
        val > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use web3::types::{Address, Log};

    #[test]
    pub fn test_it_reads() {
        let log = Log {
            address: Address::from_low_u64_be(1),
            topics: vec![
                hex!("06fbd2297e6f6f7701a9cf99685a6af911cab275ec5c75ac7aaaf13b5cf3d61f").into(),
                hex!("000000000000000000000000061b8335e1d2042975c4ed849943334bd07fb504").into(),
            ],
            data: hex!("0000000000000000000000000000000000000000000000056bc75e2d631000000000000000000000000000000000000000000000000000056bb73f60696ee4160000000000000000000000000000000000000000000000000000000060da02bd").into(),
            block_hash: Some(H256::from_low_u64_be(2)),
            block_number: Some(1.into()),
            transaction_hash: Some(H256::from_low_u64_be(3)),
            transaction_index: Some(0.into()),
            log_index: Some(0.into()),
            transaction_log_index: Some(0.into()),
            log_type: None,
            removed: Some(true),
        };
        let mut r = LogReader::new(&log, 1, Some(3)).unwrap();
        assert_eq!(
            r.address(),
            hex!("061b8335e1d2042975c4ed849943334bd07fb504").into()
        );
        assert_eq!(
            r.value(),
            hex!("0000000000000000000000000000000000000000000000056bc75e2d63100000").into()
        );
        assert_eq!(
            r.value(),
            hex!("0000000000000000000000000000000000000000000000056bb73f60696ee416").into()
        );
        assert_eq!(
            r.value(),
            hex!("0000000000000000000000000000000000000000000000000000000060da02bd").into()
        );
    }

    #[test]
    pub fn test_reads_meta_data() {
        let log = Log {
            address: Address::from_low_u64_be(1),
            topics: vec![
                hex!("4d72fe0577a3a3f7da968d7b892779dde102519c25527b29cf7054f245c791b9").into(),
                hex!("0000000000000000000000000000000000000000000000000000000000000000").into(),
                hex!("000000000000000000000000061b8335e1d2042975c4ed849943334bd07fb504").into()
            ],
            data: hex!(
                "00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000047311f7472616e7366657228616464726573732c75696e74323536291f4d7920666972737420415049332070726f706f73616c1f466f722074657374696e6720707572706f73657300000000000000000000000000000000000000000000000000"
            ).into(),
            block_hash: Some(H256::from_low_u64_be(2)),
            block_number: Some(1.into()),
            transaction_hash: Some(H256::from_low_u64_be(3)),
            transaction_index: Some(0.into()),
            log_index: Some(0.into()),
            transaction_log_index: Some(0.into()),
            log_type: None,
            removed: Some(true),
        };
        let mut r = LogReader::new(&log, 2, None).unwrap();
        assert_eq!(
            r.value(),
            hex!("0000000000000000000000000000000000000000000000000000000000000000").into()
        );
        assert_eq!(
            r.address(),
            hex!("061b8335e1d2042975c4ed849943334bd07fb504").into()
        );
        assert_eq!(
            r.text(),
            "G1|transfer(address,uint256)|My first API3 proposal|For testing purposes"
        );
    }
}
