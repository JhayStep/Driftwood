mod storage; mod wire; mod gossip;
use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name="sync-node", version, about="CRDT sync node")]
struct Cli {
    /// Node id (actor id)
    #[arg(long)] node: String,
    /// Listen address (e.g., 127.0.0.1:7070)
    #[arg(long, default_value="127.0.0.1:7070")] listen: String,
    /// Database path
    #[arg(long, default_value="./data")]
    db: String,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the node server
    Run,
    /// Put a key/value in a doc (for demo)
    Put { doc: String, key: String, val: String },
    /// Get a key from a doc
    Get { doc: String, key: String },
}

fn now_micros() -> i128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (d.as_secs() as i128)*1_000_000 + (d.subsec_micros() as i128)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let cli = Cli::parse();
    let store = storage::Storage::open(&cli.db)?;
    match cli.cmd {
        Cmd::Run => gossip::listen(&cli.listen, &cli.node, store).await?,
        Cmd::Put { doc, key, val } => {
            use bincode; use crdt_core::LwwMap; 
            let mut m: LwwMap<String, String> = if let Some(bytes) = store.get_doc(&doc)? { bincode::deserialize(&bytes)? } else { LwwMap::new(cli.node.clone()) };
            m.put(key, val, now_micros());
            store.put_doc(&doc, &bincode::serialize(&m)?)?;
            println!("ok");
        }
        Cmd::Get { doc, key } => {
            if let Some(bytes) = store.get_doc(&doc)? { let m: crdt_core::LwwMap<String, String> = bincode::deserialize(&bytes)?; if let Some(v) = m.get(&key) { println!("{}", v); } }
        }
    }
    Ok(())
}
