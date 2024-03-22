# RealTimeChatApp

## Prerequisites

Frontend
- `node` - `v20.0.0+` 
- `npm` - `v10.2.4`

Backend
- `rustc` - `v1.76.0` 
- `cargo` - `v1.76.0`

## Setup to run

Clone the project:

```bash
git clone https://github.com/r11234/RealTimeChatApp.git
cd RealTimeChatApp # change directory to the cloned project
```

Start the frontend:

```bash
cd ./client
npm i
npm run build
npm run preview 
```

Start the backend:

```bash
cd ./server
cargo run --bin server -r
```

## Demo
![](./res/demo.gif)

## Design decisions
- Each client, on connection, is assigned a random v4 UUid id by the server. 
- First 8 characters of the generated UUid is used as the name of the client.
- After the client is connected, the server sends its nane along with a random
  number between 0 and 360 which is used as the colour for client in the chat 
  box.
- A hashmap is used to store all the connected clients, when a client disconnects
  it's entry is removed from the hashmap. The unique UUid is used as the key
  of the hashmap.
- The config for frontend is stored in `.env` file and for backend in `config.toml`.

## Scope for improvement 
- Allowing users to choose their own name.
- A dynamic list of current users for shown in the UI.
- Multiple lobbies.
- Allow users to reconnect to the server without reloading.
- Preserving the chat history for lobbies.
