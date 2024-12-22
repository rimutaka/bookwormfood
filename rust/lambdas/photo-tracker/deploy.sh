# Run this script from the root of the project

target=aarch64-unknown-linux-gnu
region=us-east-1
lambda=photo-tracker
crate=photo-tracker

RUSTFLAGS='-C target-feature=+crt-static'  cargo build --release --target $target --package $crate
cp ./target/$target/release/$crate ./bootstrap && zip proxy.zip bootstrap && rm bootstrap
aws lambda update-function-code --region $region --function-name $lambda --zip-file fileb://proxy.zip
rm proxy.zip

# Available targets: 
# x86_64-unknown-linux-gnu
# x86_64-unknown-linux-musl
# aarch64-unknown-linux-gnu
# aarch64-unknown-linux-musl