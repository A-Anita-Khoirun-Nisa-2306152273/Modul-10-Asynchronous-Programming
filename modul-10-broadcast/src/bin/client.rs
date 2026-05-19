use futures::SinkExt;
use futures::StreamExt;
use std::error::Error;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(http::Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut stdin = tokio::io::BufReader::new(stdin);
        let mut line = String::new();
        loop {
            line.clear();
            stdin.read_line(&mut line).await.unwrap();
            tx.send(line.clone()).unwrap();
        }
    });

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("Anita's Computer - From server: {}", text);
                        }
                    }
                    _ => break,
                }
            }
            line = rx.recv() => {
                if let Some(line) = line {
                    ws_stream.send(Message::text(line)).await?;
                }
            }
        }
    }
    Ok(())
}