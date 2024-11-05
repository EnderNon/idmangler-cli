cargo build --target x86_64-unknown-linux-gnu --package idmanglercli --bin idmanglercli
cargo build --target x86_64-pc-windows-gnu --package idmanglercli --bin idmanglercli

mkdir -p product

cp ./target/x86_64-pc-windows-gnu/release/newestidmangler ./product/idmangler-gui-windows-x64
cp ./target/x86_64-unknown-linux-gnu/release/newestidmangler ./product/idmangler-gui-linux-x64
