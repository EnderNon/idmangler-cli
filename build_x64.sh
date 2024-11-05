cargo build --target x86_64-unknown-linux-gnu --package idmanglercli --bin idmanglercli --release
cargo build --target x86_64-pc-windows-gnu --package idmanglercli --bin idmanglercli --release

mkdir -p product

cp ./target/x86_64-pc-windows-gnu/release/idmanglercli.exe ./product/idmangler-gui-windows-x64
cp ./target/x86_64-unknown-linux-gnu/release/idmanglercli ./product/idmangler-gui-linux-x64
cp -u -p ./{config.json,config_hanafupookie.json,config_singu.json} ./product/

cd product
zip 
