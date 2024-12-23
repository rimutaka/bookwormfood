target=aarch64-unknown-linux-gnu
region=us-east-1
lambda=photo-tracker
crate=proxy-lambda

cd /home/mx/projects/gh-forks/lambda-runtime-emulator
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target $target
cp ./target/$target/release/$crate ./bootstrap && zip proxy.zip bootstrap && rm bootstrap
aws lambda update-function-code --region $region --function-name $lambda --zip-file fileb://proxy.zip