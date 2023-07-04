const WebSocket = require('ws');

const socket = new WebSocket('wss://ws-feed.exchange.coinbase.com');

socket.addEventListener('open', function (event) {
    const subscriptionData = {
        type: "subscribe",
        product_ids: ["BTC-USD"],
        channels: [
            'level2',
            'heartbeat',
            {
                name: 'ticker',
                product_ids: ['BTC-USD']
            }
        ]
    };

    socket.send(JSON.stringify(subscriptionData));
});

socket.addEventListener('message', function (event) {
    console.log('Received message:', event.data);
});

socket.addEventListener('error', function (event) {
    console.error('WebSocket error:', event);
});
