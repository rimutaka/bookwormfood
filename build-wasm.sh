## --release or --dev - exclude/include debug info
## --no-typescript - disable .d.ts files output
## --out-dir - where to write the compiled files
## --out-name - force output file names
## --target - always use "web"!
## See https://rustwasm.github.io/wasm-pack/book/commands/build.html
echo Building wasm module...
wasm-pack build rust/wasm_mod --dev --out-dir "../../vue/src/wasm-rust" --out-name "isbn_mod" --target web

## wasm-pack creates bunch of useless files:
echo Removing trash files...
rm -f vue/src/wasm-rust/package.json
rm -f vue/src/wasm-rust/.gitignore