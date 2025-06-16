### customwritertg - Telegram userbot for custom typing effect 

# Requirements 
- cargo
- settings file (already existing)

# Usage

Open file settings and set your api_id, api_hash (which you can receive from https://my.telegram.org/auth), and phone number 

Insert them into the file "settings" spaced from 
API_ID 
API_HASH 
PHONENUMBER

Save the file and run the program:

```cmd 
cargo run 
```

After that in Telegram chat type following text:
!t command

The world "command" will be typed im the chat with custom type symbol

# Info

- The general code structure of client initialization and event processing was used from grammers-client/examples/echo.rs (https://github.com/Lonami/grammers)
- The program is build and tested only on Windows x64
- Also works with UTF-8 symbols and other locales (not English)