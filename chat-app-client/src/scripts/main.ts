import { addChatMessage, connectSocket } from "./util";

window.addEventListener('load', async (_event) => {
    try {
        const socket = await connectSocket();
        const chatbox = document.querySelector('div#chat-box')!;
        const chat_message_input = 
            document.querySelector('input#send-message-text')! as HTMLInputElement;


        socket.addEventListener('message', (event) => {
            try {
                const received_data = JSON.parse(event.data);

                addChatMessage(
                    received_data.sender_msg, 
                    received_data.sender_id, 
                    received_data.sender_color_hue, 
                    chatbox
                );
            } catch(err) {
                console.log(`Error in parsing the received data ${err}`)
            }
        });

        window.addEventListener("beforeunload", (_event) => {
            socket.close();
        });
        
        document.querySelector('form#send-message-form')!
            .addEventListener('submit', (event) => {
                event.preventDefault();
                
                if(chat_message_input.value !== '') {
                    socket.send(chat_message_input.value);
                    chat_message_input.value = '';
                }
            });
        
    } catch(err) {
        console.log(err);
    }
});


