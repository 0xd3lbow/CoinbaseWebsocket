use std::error::Error;
use url::Url;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::WebSocketStream;
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;

#[derive(Debug, Serialize, Deserialize)]
struct SubscriptionData {
    #[serde(rename = "type")]
    type_: String,
    product_ids: Vec<String>,
    channels: Vec<Channel>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    name: String,
    product_ids: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = Url::parse("wss://ws-feed.exchange.coinbase.com")?;

    let req = tokio::net::TcpStream::connect(url.host_str().unwrap())?;
    let ws_stream = tokio_tungstenite::client_async_tls(req, url.host_str().unwrap())?
        .await?;

    let (mut write, mut read) = ws_stream.split();

    let subscription_data = SubscriptionData {
        type_: "subscribe".to_string(),
        product_ids: vec!["BTC-USD".to_string()],
        channels: vec![
            Channel {
                name: "level2".to_string(),
                product_ids: vec!["BTC-USD".to_string()],
            },
            Channel {
                name: "ticker".to_string(),
                product_ids: vec!["BTC-USD".to_string()],
            },
        ],
    };

    let subscription_data_json = serde_json::to_string(&subscription_data)?;
    write.send(Message::Text(subscription_data_json)).await?;

    while let Some(message) = read.next().await {
        if let Ok(message) = message {
            if message.is_text() {
                let message_text = message.to_text().unwrap();
                let message_json: serde_json::Value = serde_json::from_str(message_text)?;
                if let Some(message_type) = message_json.get("type").and_then(|t| t.as_str()) {
                    if message_type == "snapshot" || message_type == "l2update" {
                        println!("{:?}", message_json);
                    }
                }
            }
        }
    }

    Ok(())
}
