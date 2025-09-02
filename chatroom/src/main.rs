// imports from std and crates
use std::net::SocketAddr; // socket address type (not strictly required but helpful in prints)
use futures_util::{StreamExt, SinkExt}; // helpers to read/write async streams
use tokio::net::TcpListener; // async TCP listener from tokio
use tokio_tungstenite::tungstenite::protocol::Message; // WebSocket message type (Text/Binary/Close)
use tokio_tungstenite::accept_async; // performs the websocket handshake on a TcpStream
use tokio::sync::broadcast; // simple pub/sub channel useful for chat

// The entry point attribute: starts the tokio async runtime for us.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a broadcast channel for messages. `tx` is the sender, `_rx` is one receiver we drop.
    // Buffer size 100: if receivers lag behind and more than 100 messages arrive, the oldest are dropped.
    let (tx, _rx) = broadcast::channel::<String>(100);

    // Address to bind the TCP listener to.
    // Use 0.0.0.0 so Windows host (when using WSL2) can reach the server at localhost:8080.
    let addr = "0.0.0.0:8080";
    // Bind a TCP listener (async). This listens for new TCP connections on port 8080.
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on http://{}", addr);

    // Accept incoming connections in an infinite loop.
    loop {
        // Wait for a new TCP connection. `listener.accept().await` yields (TcpStream, SocketAddr).
        let (stream, peer_addr) = listener.accept().await?;
        // Clone the broadcast sender so each connection handler can send messages to the room.
        let tx = tx.clone();
        // Each connection also needs its own receiver to get broadcasted messages.
        let mut rx = tx.subscribe();

        // Spawn a new asynchronous task for this connection so the loop can accept new ones.
        tokio::spawn(async move {
            // Perform WebSocket handshake on the TCP stream. If it fails, print and return.
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("WebSocket handshake failed from {}: {}", peer_addr, e);
                    return;
                }
            };
            println!("New WebSocket connection: {}", peer_addr);

            // Split the WebSocket stream into a write half and a read half.
            // `write` is used to send messages to the client; `read` yields messages from the client.
            let (mut write, mut read) = ws_stream.split();

            // --- Task A: forward broadcast -> websocket write --------------------------------
            // Move `rx` into this task. This loop waits for messages published on `tx` and sends them
            // to this client via the WebSocket `write` half.
            let mut rx_task = rx;
            let write_task = tokio::spawn(async move {
                loop {
                    match rx_task.recv().await {
                        Ok(msg) => {
                            // Send text message to client. If sending fails, assume client disconnected.
                            if write.send(Message::Text(msg)).await.is_err() {
                                break;
                            }
                        }
                        // If the receiver lagged too far behind, it returns a Lagged error with how many
                        // messages were skipped. We just log and continue.
                        Err(broadcast::error::RecvError::Lagged(skipped)) => {
                            eprintln!("Client lagged; skipped {} messages", skipped);
                            continue;
                        }
                        // Other errors -> break and stop writer task.
                        Err(_) => break,
                    }
                }
                // writer task ends here
            });

            // --- Task B: read websocket messages -> broadcast --------------------------------
            // This task reads messages from the client and publishes them to the broadcast channel.
            // We clone `tx` (sender) to publish incoming text messages to everyone.
            let tx_task = tx.clone();
            let read_task = tokio::spawn(async move {
                while let Some(msg_result) = read.next().await {
                    match msg_result {
                        // If client sent Text, publish it to the room
                        Ok(Message::Text(text)) => {
                            // Best-effort: ignore send error (no receivers) by using `_ =`
                            let _ = tx_task.send(text);
                        }
                        // If client closes, break (connection closing)
                        Ok(Message::Close(_)) => break,
                        // ignore Ping/Pong/Binary in this simple example
                        Ok(_) => {}
                        // On error reading from socket, log and stop
                        Err(e) => {
                            eprintln!("WebSocket error from {}: {}", peer_addr, e);
                            break;
                        }
                    }
                }
                // reader task ends here
            });

            // Wait for either task to finish; when one finishes, the other will also end soon.
            let _ = tokio::join!(write_task, read_task);

            println!("Connection {} closed", peer_addr);
        });
    }
}

