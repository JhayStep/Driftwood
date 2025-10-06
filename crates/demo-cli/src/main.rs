use anyhow::Result;
use clap::{Parser, Subcommand};
use crdt_core::LwwMap;

#[derive(Parser)]
#[command(name="drift", about="Offline-first notes over CRDTs")]
struct Cli { #[command(subcommand)] cmd: Cmd }

#[derive(Subcommand)]
enum Cmd {
    New { id: String },
    Put { id: String, key: String, val: String },
    Show { id: String },
}

fn now_micros() -> i128 { use std::time::{SystemTime, UNIX_EPOCH}; let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap(); (d.as_secs() as i128)*1_000_000 + (d.subsec_micros() as i128) }

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::New { id } => {
            let m: LwwMap<String, String> = LwwMap::new("cli");
            let bytes = bincode::serialize(&m)?;
            std::fs::write(format!("{id}.bin"), bytes)?;
            println!("created {id}.bin");
        }
        Cmd::Put { id, key, val } => {
            let mut m: LwwMap<String, String> = bincode::deserialize(&std::fs::read(format!("{id}.bin"))?)?;
            m.put(key, val, now_micros());
            std::fs::write(format!("{id}.bin"), bincode::serialize(&m)?)?;
            println!("updated");
        }
        Cmd::Show { id } => {
            let m: LwwMap<String, String> = bincode::deserialize(&std::fs::read(format!("{id}.bin"))?)?;
            println!("{:?}", m.assigns.keys().collect::<Vec<_>>() );
        }
    }
    Ok(())
}
