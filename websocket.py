import websocket
import json

def on_open(ws):
    print('WebSocket connection opened')
    subscription_data = {
        'type': 'subscribe',
        'product_ids': ['BTC-USD'],
        'channels': [
            {'name': 'level2', 'product_ids': ['BTC-USD']},
            {'name': 'ticker', 'product_ids': ['BTC-USD']}
        ]
    }
    ws.send(json.dumps(subscription_data))

def on_message(ws, message):
    message = json.loads(message)
    if message['type'] in ('snapshot', 'l2update'):
        print(message)

def on_error(ws, error):
    print('WebSocket error:', error)

socket = websocket.WebSocketApp('wss://ws-feed.exchange.coinbase.com')
socket.on_open = on_open
socket.on_message = on_message
socket.on_error = on_error
socket.run_forever()
