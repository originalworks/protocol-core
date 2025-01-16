cargo build --release --bin owen_cli
mkdir -p ./owen_cli
yes | cp -rf ./target/release/owen_cli ./owen_cli/owen_cli
zip -r ./owen_cli.zip ./owen_cli/owen_cli
rm -rf ./owen_cli