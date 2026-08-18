use crate::storage::Storage;
use crate::wire::Msg;
use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

pub async fn listen(addr: &str, node_id: &str, store: Storage) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!(%addr, "listening");

    loop {
        let (mut sock, peer) = listener.accept().await?;
        info!(%peer, "peer connected");

        let store = Storage {
            db: store.db.clone(),
        };

        let node_id = node_id.to_string();

        tokio::spawn(async move {
            if let Err(e) = handle(&mut sock, &node_id, store).await {
                warn!(?e, "session error");
            }
        });
    }
}

pub async fn connect(addr: &str, node_id: &str, store: Storage) -> Result<()> {
    let mut sock = TcpStream::connect(addr).await?;

    info!(%addr, "connected to peer");

    handle(&mut sock, node_id, store).await
}

async fn handle(sock: &mut TcpStream, node_id: &str, store: Storage) -> Result<()> {
    send_message(
        sock,
        &Msg::Hello {
            node_id: node_id.to_string(),
        },
    )
    .await?;

    send_message(
        sock,
        &Msg::StateDigest {
            counts: store.digest()?,
        },
    )
    .await?;

    while let Ok(n) = sock.read_u32().await {
        let len = n as usize;

        let mut buf = vec![0u8; len];
        sock.read_exact(&mut buf).await?;

        let msg: Msg = bincode::deserialize(&buf)?;

        match msg {
            Msg::Hello {
                node_id: remote_node,
            } => {
                info!(node = %remote_node, "received peer hello");
            }

            Msg::StateDigest { counts } => {
                let local = store.digest()?;

                for (doc, remote_version) in counts {
                    let local_version = local.get(&doc).copied().unwrap_or(0);

                    if remote_version > local_version {
                        send_message(sock, &Msg::Pull { doc }).await?;
                    }
                }
            }

            Msg::Pull { doc } => {
                if let Some(bytes) = store.get_doc(&doc)? {
                    send_message(sock, &Msg::Delta { doc, bytes }).await?;
                }
            }

            Msg::Delta { doc, bytes } => {
                store.put_doc(&doc, &bytes)?;

                info!(
                    document = %doc,
                    "applied remote document state"
                );
            }
        }
    }

    Ok(())
}

async fn send_message(sock: &mut TcpStream, msg: &Msg) -> Result<()> {
    let data = bincode::serialize(msg)?;

    sock.write_u32(data.len() as u32).await?;
    sock.write_all(&data).await?;

    Ok(())
}
