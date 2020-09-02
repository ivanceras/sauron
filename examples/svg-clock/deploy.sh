set -v

if ! type wasm-pack > /dev/null; then
    echo "wasm-pack is not installed"
    cargo install wasm-pack
fi

if ! type basic-http-server > /dev/null; then
    echo "basic-http-server is not installed"
    cargo install basic-http-server
fi

wasm-pack build --target web --release -- --features "wee_alloc"

dest_dir="../../../ivanceras.github.io/svg-clock"

mkdir -p $dest_dir;

cp index.html $dest_dir/index.html
cp -r pkg $dest_dir/

## Remove the ignore file on the pkg directory
rm $dest_dir/pkg/.gitignore
