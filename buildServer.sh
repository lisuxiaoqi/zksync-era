set -a 
source etc/env/dev.env
set +a
cargo run --bin zksync_server --release -- --genesis
