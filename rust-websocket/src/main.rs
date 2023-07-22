use std::error::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tungstenite::protocol::Message;
use url::Url;
use serde::{Deserialize, Serialize};

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
    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
    let (write, read) = ws_stream.split();

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

    let reader = BufReader::new(read);
    let lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        let message = serde_json::from_str::<serde_json::Value>(&line)?;
        if let Some(message_type) = message.get("type").and_then(|t| t.as_str()) {
            if message_type == "snapshot" || message_type == "l2update" {
                println!("{:?}", message);
            }
        }
    }

    Ok(())
}
