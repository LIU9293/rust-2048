curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source "$HOME/.cargo/env"

cargo install dioxus-cli
cargo install cargo-bundle

cargo-bundle bundle --release