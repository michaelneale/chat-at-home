# Run Ollama Chat type GUI as desktop app

"why do something with a shell script you can do with a rust app"

"Mom can we have ChatGPT" "No, we have chatgpt at home". 

ChatGPT at home:

![ChatGPT at home](logo.jpg)


# Running

Ensure you have docker (desktop) installed with docker compose. Then can run as a binary or: 
`cargo run`

The first time this will pull images, and then pre-load it with a model (which is a few G normally to pull).

This is my first rust project, be kind. 

# Building

`cargo build`

Uses `wry` for the webview, and `tao` for x-platform windowing. Ollama (https://github.com/jmorganca/ollama) for model hosting and Ollama chat gui for the actual GUI (https://github.com/ollama-webui/ollama-webui). Docker compose as well. 


![Screenshot of GUI](./shot.png)
