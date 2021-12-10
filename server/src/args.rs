use clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum DumpMode {
        Events,
        Unknown,
        Snapshot,
    }
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "api3dao-tracker", about = "API3 DAO Tracker")]
pub struct Args {
    /// Net listening address of HTTP server
    #[structopt(long, default_value = "0.0.0.0:8000", env = "LISTEN")]
    pub listen: String,
    /// Static folder to serve web client files
    #[structopt(long, default_value = "./../client/dist", env = "STATIC_DIR")]
    pub static_dir: String,
    /// Cache folder to store responses from ETH to avoid scan
    #[structopt(long, default_value = "", env = "CACHE_DIR")]
    pub cache_dir: String,
    /// Ethereum JSON+RPC HTTP address
    #[structopt(long, default_value = "http://localhost:8545", env = "RPC_ENDPOINT")]
    pub rpc_endpoint: String,
    /// Another ethereum JSON+RPC HTTP endpoint that would be used to watch events.
    #[structopt(long, default_value = "", env = "RPC_WATCH_ENDPOINT")]
    pub watch_endpoint: String,
    /// Ethereum JSON+RPC batch size for reading
    #[structopt(long, default_value = "500", env = "RPC_BATCH_SIZE")]
    pub rpc_batch_size: u64,
    /// USDC token contract address
    #[structopt(long, default_value = "", env = "ADDR_USDC_TOKEN")]
    pub address_usdc_token: String,
    /// API3 token contract address
    #[structopt(long, default_value = "", env = "ADDR_API3_TOKEN")]
    pub address_api3_token: String,
    /// API3 pool contract address
    #[structopt(long, default_value = "", env = "ADDR_API3_POOL")]
    pub address_api3_pool: String,
    /// API3 contract with supply
    #[structopt(long, env = "ADDR_API3_CIRCULATION")]
    pub address_api3_circulation: Option<String>,

    /// API3 convenience contract address
    #[structopt(long, default_value = "", env = "ADDR_API3_CONVENIENCE")]
    pub address_convenience: String,
    /// API3 primary voting contract address
    #[structopt(long, default_value = "", env = "ADDR_API3_VOTING_PRIMARY")]
    pub address_voting1: String,
    /// API3 primary voting agent address
    #[structopt(long, default_value = "", env = "ADDR_API3_AGENT_PRIMARY")]
    pub address_agent1: String,
    /// API3 secondary voting contract address
    #[structopt(long, default_value = "", env = "ADDR_API3_VOTING_SECONDARY")]
    pub address_voting2: String,
    /// API3 secondary voting agent address
    #[structopt(long, default_value = "", env = "ADDR_API3_AGENT_SECONDARY")]
    pub address_agent2: String,
    /// API3 circulation contract address (optional)
    #[structopt(long, env = "ADDR_API3_CIRCULATION")]
    pub address_circulation: Option<String>,
    /// Number of the first block to start watching
    #[structopt(long, default_value = "8842400", env = "GENESIS_BLOCK")]
    pub genesis_block: u64,
    /// Max block to stop contract events listening
    #[structopt(long, env = "MAX_BLOCK")]
    pub max_block: Option<u64>,
    /// Dump (show logs) mode instead of running HTTP server
    #[structopt(short, long, possible_values = &DumpMode::variants(), case_insensitive = true)]
    pub dump: Option<DumpMode>,
    /// Continue listening to blockchain events
    #[structopt(short, long)]
    pub watch: bool,
    /// Disable ENS reserve resolution for the wallets
    #[structopt(long)]
    pub no_ens: bool,
}

pub fn parse() -> anyhow::Result<Args> {
    dotenv::dotenv().ok();
    let log_level: String = std::env::var("LOG_LEVEL").unwrap_or("info".to_owned());
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .init();
    let res = Args::from_args();
    tracing::debug!("{:?}", res);
    Ok(res)
}
