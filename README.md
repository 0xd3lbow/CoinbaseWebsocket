# Orderbook Snapshots
This WebSocket feed uses a bidirectional protocol that encodes all messages as JSON objects.

All messages have a type attribute that can be used to handle the message appropriately.

https://github.com/0xd3lbow/CoinbaseWebsocket/assets/130616587/9a45b045-d312-4dbe-a28d-89be4ecda198

# Connections
8 requests every second per IP and up to 20 requests for bursts

Messages sent by the client: 100 every second per IP on each connection

# Channels
The level2 channel reduces the overhead required when consuming the full channel, while the ticker channel provides real-time price updates every time a match happens. It batches updates in case of cascading matches, greatly reducing bandwidth requirements.

```python
import websocket
import json
      
        'channels': [
            {'name': 'level2', 'product_ids': ['BTC-USD']},
            {'name': 'ticker', 'product_ids': ['BTC-USD']}
        ]
```

A size integer of "0.00" indicates the removal of orders.
```
{'type': 'l2update',
'product_id': 'BTC-USD',
'changes': [['sell', '30862.60', '0.00000000']],
'time': '2023-07-04T18:14:39'}
```

Even though a WebSocket connection is over TCP, the WebSocket servers receive market data in a manner that can result in dropped messages. Your feed consumer should be designed to handle sequence gaps and out of order messages, or should use channels that guarantee delivery of messages.
To guarantee that messages are delivered and your order book is in sync, use the level2 channel.


# Traditional feed available without authentication:

```
wss://ws-feed.exchange.coinbase.com
```
