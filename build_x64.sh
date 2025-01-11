cargo build --target x86_64-unknown-linux-gnu --package idmangler-cli --bin idmangler-cli --release
cargo build --target x86_64-pc-windows-gnu --package idmangler-cli --bin idmangler-cli --release

mkdir -p product

cp ./target/x86_64-pc-windows-gnu/release/idmangler-cli.exe ./product/idmangler-cli.exe
cp ./target/x86_64-unknown-linux-gnu/release/idmangler-cli ./product/idmangler-cli
cp -u -p ./example_configs ./product/

pkgversion=$(cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "idmangler-cli") | .version')
echo "package version is:"
echo "$pkgversion"
balls=${pkgversion::-1}
balls2=${balls:1}
echo "$balls2"

cd product

zip "idmangler-windows-x64-${balls2}.zip" example_configs idmangler-cli.exe
zip "idmangler-linux-x64-${balls2}.zip" example_configs idmangler-cli
