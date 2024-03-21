import { addChatMessage, connectSocket, setMyName } from "./util";

window.addEventListener('load', async (_event) => {
    try {
        const chatbox = document.querySelector('div#chat-box')!;
        const chat_message_input = 
            document.querySelector('input#send-message-text')! as HTMLInputElement;
        
        const name_container = 
            document.querySelector('div#my-name-container')! as HTMLInputElement;
        

        const socket = await connectSocket((event) => {
            try {
                const received_data = JSON.parse(event.data);

                if(!received_data.msg) {
                    setMyName(
                        received_data.sender_name, 
                        received_data.sender_colour_hue,
                        name_container
                    ); 

                    return;
                }

                addChatMessage(
                    received_data.msg, 
                    received_data.sender_name, 
                    received_data.sender_colour_hue, 
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


