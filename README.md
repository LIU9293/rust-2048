## 2048 Game in Disxus

It's a demo application for me to figure out if I can use rust/dioxus in production code, this codebase contains a frontend client that can compiled to Web(wasm) and Desktop, a simple rust server to handle some basic API logic and some shared components between client and server.

Live Demo -> https://rust-2048.vercel.app/

### Features in this application

- [x] Use tailwind and daisy css for styling
- [x] Handle Dark / light theme
- [x] Try to inplenment i18n use fermi state
- [x] Access cookies / localstorage
- [x] Build web and native app
- [x] A simple server to record user progress

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

### Server 

- [x] Server, record game progress
- [ ] Server, record progress for multiple users

Github
-
https://github.com/LIU9293/rust-2048/
