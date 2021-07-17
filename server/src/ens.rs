use hex_literal::hex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tiny_keccak::{Hasher, Keccak};
use web3::contract::{Contract, Options};
use web3::types::{Address, H160, H256};

const ENS_REVERSE_REGISTRAR_DOMAIN: &str = "addr.reverse";

struct EnsSetting {
    mainnet_addr: Address,
}

lazy_static! {
    static ref ENS_SETTING: EnsSetting = EnsSetting {
        mainnet_addr: H160::from(hex!("00000000000C2E074eC69A0dFb2997BA6C7d2e1e")),
    };
}

#[derive(Debug)]
struct Resolver<T: web3::Transport> {
    contract: Contract<T>,
}

impl<T: web3::Transport> Resolver<T> {
    async fn new(ens: &ENS<T>, resolver_addr: &str) -> anyhow::Result<Self> {
        // tracing::debug!("resolving {:?}", resolver_addr);
        let addr_namehash = H256::from_slice(namehash(resolver_addr).as_slice());
        // tracing::debug!("addr_namehash {:?}", addr_namehash);
        let exists: bool = ens
            .contract
            .query(
                "recordExists",
                (addr_namehash,),
                None,
                Options::default(),
                None,
            )
            .await
            .expect("recordExists error");
        if !exists {
            return Err(anyhow::Error::msg("no resolver"));
        }

        let result =
            ens.contract
                .query("resolver", (addr_namehash,), None, Options::default(), None);
        let resolver_addr: Address = result.await.expect("resolver.result.wait()");
        // tracing::debug!("resolver_addr {:?}", resolver_addr);
        if resolver_addr == H160::from(hex!("0000000000000000000000000000000000000000")) {
            return Err(anyhow::Error::msg("no resolver addr"));
        }

        // resolve
        let resolver_contract = Contract::from_json(
            ens.web3.eth(),
            resolver_addr,
            include_bytes!("./contract/ens_reverseresolver.abi.json"),
        )
        .expect("fail load resolver contract");
        Ok(Self {
            contract: resolver_contract,
        })
    }

    async fn name(self, resolver_addr: &str) -> Result<String, String> {
        let addr_namehash = H256::from_slice(namehash(resolver_addr).as_slice());
        let result = self
            .contract
            .query("name", (addr_namehash,), None, Options::default(), None);
        match result.await {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("error: name.result.wait(): {:?}", e)),
        }
    }
}

#[derive(Debug)]
pub struct ENS<T: web3::Transport> {
    pub web3: web3::Web3<T>,
    pub contract: Contract<T>,
    pub cache_dir: String,
}

impl<T: web3::Transport> ENS<T> {
    pub fn new(web3: web3::Web3<T>, cache_dir: &str) -> Self {
        let contract = Contract::from_json(
            web3.eth(),
            ENS_SETTING.mainnet_addr,
            include_bytes!("./contract/ens_registry.abi.json"),
        )
        .expect("fail contract::from_json(ens_registry.abi.json)");
        ENS {
            web3: web3,
            contract: contract,
            cache_dir: cache_dir.to_string(),
        }
    }

    pub fn cache_fn(&self, address: &str) -> String {
        format!("{}/{}.txt", self.cache_dir, address)
    }

    pub fn has_cached(&self, address: &str) -> bool {
        if self.cache_dir.len() == 0 {
            return false;
        }
        Path::new(self.cache_fn(address).as_str()).exists()
    }

    pub fn get_cached(&self, address: &str) -> anyhow::Result<String> {
        let mut f = File::open(self.cache_fn(address)).expect("Unable to open file");
        let mut data = String::new();
        f.read_to_string(&mut data).expect("Reading failure");
        let res = data.trim();
        if res.len() > 0 {
            Ok(res.to_string())
        } else {
            Err(anyhow::Error::msg("empty string".to_string()))
        }
    }

    pub fn save_cached(&self, address: &str, result: &str) -> anyhow::Result<()> {
        if self.cache_dir.len() == 0 || address.len() == 0 {
            return Ok(());
        }
        std::fs::write(self.cache_fn(address), result).expect("Unable to write file");
        Ok(())
    }

    pub async fn name(&self, address: Address) -> Option<String> {
        let resolver_addr = format!("{:x}.{}", address, ENS_REVERSE_REGISTRAR_DOMAIN);
        if self.has_cached(&resolver_addr) {
            if let Ok(cached) = self.get_cached(&resolver_addr) {
                return Some(cached);
            }
        }

        let resolver = match Resolver::new(self, resolver_addr.as_str()).await {
            Ok(x) => x,
            Err(_) => return None,
        };
        match resolver.name(resolver_addr.as_str()).await {
            Ok(x) => {
                let _ = self.save_cached(&resolver_addr, &x);
                Some(x)
            }
            Err(_) => return None,
        }
    }
}

// namehash generates a hash from a name that can be used to look up the name in ENS
fn namehash(name: &str) -> Vec<u8> {
    let mut node = vec![0u8; 32];
    if name.is_empty() {
        return node;
    }
    let n = name.clone().to_lowercase();
    let mut labels: Vec<&str> = n.as_str().split(".").collect();
    labels.reverse();
    for label in labels.iter() {
        let mut labelhash = [0u8; 32];

        let mut hasher = Keccak::v256();
        hasher.update(label.as_bytes());
        hasher.finalize(&mut labelhash);

        node.append(&mut labelhash.to_vec());
        labelhash = [0u8; 32];

        let mut hasher = Keccak::v256();
        hasher.update(node.as_slice());
        hasher.finalize(&mut labelhash);
        node = labelhash.to_vec();
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn namehash_works() {
        assert_eq!(
            namehash("eth"),
            hex!("93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae")
        );
        assert_eq!(
            namehash("Eth"),
            hex!("93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae")
        );
        assert_eq!(
            namehash(".eth"),
            hex!("8cc9f31a5e7af6381efc751d98d289e3f3589f1b6f19b9b989ace1788b939cf7")
        );
        assert_eq!(
            namehash("resolver.eth"),
            hex!("fdd5d5de6dd63db72bbc2d487944ba13bf775b50a80805fe6fcaba9b0fba88f5")
        );
        assert_eq!(
            namehash("foo.eth"),
            hex!("de9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f")
        );
        assert_eq!(
            namehash("Foo.eth"),
            hex!("de9b09fd7c5f901e23a3f19fecc54828e9c848539801e86591bd9801b019f84f")
        );
        assert_eq!(
            namehash("foo..eth"),
            hex!("4143a5b2f547838d3b49982e3f2ec6a26415274e5b9c3ffeb21971bbfdfaa052")
        );
        assert_eq!(
            namehash("bar.foo.eth"),
            hex!("275ae88e7263cdce5ab6cf296cdd6253f5e385353fe39cfff2dd4a2b14551cf3")
        );
        assert_eq!(
            namehash("Bar.foo.eth"),
            hex!("275ae88e7263cdce5ab6cf296cdd6253f5e385353fe39cfff2dd4a2b14551cf3")
        );
        assert_eq!(
            namehash("addr.reverse"),
            hex!("91d1777781884d03a6757a803996e38de2a42967fb37eeaca72729271025a9e2")
        );
    }
}
