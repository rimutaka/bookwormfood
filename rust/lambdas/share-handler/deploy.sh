# Run this script from the root of the project

target=aarch64-unknown-linux-gnu
region=us-east-1
lambda=share-handler
crate=share-handler

cargo build --release --target $target --package $crate
cp ./target/$target/release/$crate ./bootstrap && zip proxy.zip bootstrap && rm bootstrap
aws lambda update-function-code --region $region --function-name $lambda --zip-file fileb://proxy.zip
rm proxy.zip

# Available targets: 
# x86_64-unknown-linux-gnu
# x86_64-unknown-linux-musl
# aarch64-unknown-linux-gnu
# aarch64-unknown-linux-musl

# permissions script
# aws lambda add-permission \--statement-id "AllowCloudFrontServicePrincipal" \--action "lambda:InvokeFunctionUrl" \--principal "cloudfront.amazonaws.com" \--source-arn "arn:aws:cloudfront::512295225992:distribution/E3FGXRC3VXQ2IF" \--region "us-east-1" \--function-name share-handler