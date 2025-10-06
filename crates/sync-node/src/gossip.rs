use crate::storage::Storage;
use crate::wire::Msg;
use anyhow::Result;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, net::TcpStream};
use tracing::{info, warn};

pub async fn listen(addr: &str, node_id: &str, store: Storage) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!(%addr, "listening");
    loop {
        let (sock, _peer) = listener.accept().await?;
        let mut sock = sock;
        let store = Storage { db: store.db.clone() };
        let node_id = node_id.to_string();
        tokio::spawn(async move {
            if let Err(e) = handle(&mut sock, &node_id, store).await { warn!(?e, "session error"); }
        });
    }
}

async fn handle(sock: &mut TcpStream, node_id: &str, store: Storage) -> Result<()> {
    // send hello
    let hello = bincode::serialize(&Msg::Hello { node_id: node_id.to_string() })?;
    sock.write_u32(hello.len() as u32).await?; sock.write_all(&hello).await?;

    // simplistic read loop
    loop {
        let len = match sock.read_u32().await { Ok(n) => n as usize, Err(_) => break };
        let mut buf = vec![0u8; len];
        sock.read_exact(&mut buf).await?;
        let msg: Msg = bincode::deserialize(&buf)?;
        match msg { Msg::Pull { doc } => {
            if let Some(bytes) = store.get_doc(&doc)? {
                let m = Msg::Delta { doc, bytes };
                let data = bincode::serialize(&m)?;
                sock.write_u32(data.len() as u32).await?; sock.write_all(&data).await?;
            }
        }
        _ => {}
        }
    }
    Ok(())
}
