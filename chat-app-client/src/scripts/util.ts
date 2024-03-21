export async function connectSocket(): Promise<WebSocket> {
    const ws = new WebSocket('ws://localhost:7000');
    return new Promise((resolve, reject) => {
        let count = 0;
        
        const timer = setInterval(() => {
            count += 1;
            if(ws.readyState === 1) {
                console.log('Connected to the socket');
                clearInterval(timer)
                resolve(ws)
            }
        }, 10);


        if(count > 100) reject(new Error('Unable to connect to the socket'));
    });
}

function createNameTag(id: string, colour: string) : HTMLSpanElement {
    const name_tag = document.createElement('span'); 
    name_tag.classList.add('msg-name-tag');
    name_tag.style.backgroundColor = `hsl(${colour}, 50%, 50%)`;
    name_tag.innerText = id;


    return name_tag;
}

function createChatMessage(message: string) {
    const chat_msg = document.createElement('span'); 
    chat_msg.classList.add('chat-msg');
    chat_msg.innerText = message;

    return chat_msg;

} 

export function addChatMessage(msg: string, id: string, colour: string, chatbox: Element): void {
    const name_tag = createNameTag(id, colour);
    const chat_msg = createChatMessage(msg); 

    const chat_message_container = document.createElement('div');
    chat_message_container.classList.add('chat-message-container');
    chat_message_container.append(name_tag);
    chat_message_container.append(chat_msg);

    chatbox.append(chat_message_container);
}
