# pts_gem-chat

## **Unlimited and uncensored**

Local without limitations(expect token limit) chat with Google Gemini models. You can use many experimental Google
models for free.

## Usage

1. Install built or build from source by running `cargo build --release`.
2. Put in folder with built binary or repo folder `.env` file.

### In chat commands(not matching case)

* `/exit` - stops chat
* `/clear` - clears chat

## Configuring `.env`

### Create `.env` file with next content, for example

``` dotenv
API_KEY=...
MODEL=gemini-2.0-flash-thinking-exp-01-21
API_VERSION=v1beta
PROXY=socks5://127.0.0.1:2080
```

Get your API key in https://aistudio.google.com/apikey. If you won't use proxy, leave the field blank.
