## 2048 Game in Disxus

Live Demo -> https://rust-2048.vercel.app/

### Test and run
```
1. Run server
cargo run -p server

2. Run client
// web
dioxus serve --platform web

// desktop
dioxus serve --platform desktop
```

### Target

* Cross platform
* Full stack in rust, with a simple backend server
* Platform support
  - [x] Web
  - [x] Native MacOS
  - [ ] Mobile ??
  - [ ] Terminal ??

### Progress

- [x] Basic UI
- [x] Basic move logic
- [x] Success or fail
- [x] How to deploy
  * Web -> scripts/build-web.sh
  * Desktop -> scripts/build-desktop.sh
- [x] Log the highest score and the total score
- [ ] CI/CD
  * Web -> Vercel, done
  * Desktop?
- [x] Server, record game progress
- [ ] Server, record progress for multiple users

### Server 

TODO -> https://github.com/tokio-rs/axum