cargo build --target x86_64-unknown-linux-gnu --package idmangler-cli --bin idmangler-cli --release
cargo build --target x86_64-pc-windows-gnu --package idmangler-cli --bin idmangler-cli --release

mkdir -p product

cp ./target/x86_64-pc-windows-gnu/release/idmangler-cli.exe ./product/idmangler-cli-windows-x64.exe
cp ./target/x86_64-unknown-linux-gnu/release/idmangler-cli ./product/idmangler-cli-linux-x64
cp -u -p ./{config.json,config_hanafupookie.json,config_singu.json} ./product/

cd product
zip 
