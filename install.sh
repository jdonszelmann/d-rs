
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo build --release
unlink ~/.local/bin/d
ln -s $SCRIPT_DIR/target/release/d-rs ~/.local/bin/d
