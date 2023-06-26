curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source "$HOME/.cargo/env"

cargo install dioxus-cli

rustup target add wasm32-unknown-unknown

dioxus build --release