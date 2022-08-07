# http-server
A simple http server, in Rust 🦀

This was a self-learning project I did over a weekend. *Do not use it in production!* 😊

Inside you will find a single threaded TCP based HTTP server which supports:
- parsing simple HTTP/1.1 requests, including
  - path and query string
  - headers
  - no real BODY support 🤔
- support for serving static files from a `public` directory, including html and css

I learned a lot implementing this, including:
- how to make and expose different packages
- lifetimes `'buf`
- macros and compiler flags
- rust flavor of enums and pattern matching
- How to implement [different traits, such as Display](https://github.com/trevor-atlas/http-server/blob/main/src/http/status_code.rs)
- Various methods for handling different `Result` and `Option` outcomes.

Feel free to use any of this code for whatever you like!
