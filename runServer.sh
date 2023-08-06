ulimit -n 8176
set -a 
cd etc/env
source dev.env
source .init.env
set +a
cd -
cargo run --bin zksync_server --release 
