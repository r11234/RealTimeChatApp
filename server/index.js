import { WebSocketServer } from 'ws';
import {randomBytes} from 'node:crypto';

const wss = new WebSocketServer({ port: 7000 });

const client_data = new Set();

wss.on('connection', (ws) => {
    const id = randomBytes(4).toString('hex');
    const color_hue = Math.floor(Math.random() * 360);

    client_data.add(ws);

    ws.on('message', (msg) => {
        const broadcast_message = JSON.stringify({
            sender_id: id,
            sender_color_hue: color_hue,
            sender_msg: msg.toString('utf-8')
        });
        
        [ ...client_data.keys() ].forEach(client => {
            client.send(broadcast_message);
        });
    })

    ws.on('close', () => {
        client_data.delete(ws)
    })
});

