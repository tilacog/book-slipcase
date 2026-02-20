# Book Slipcase

Plotter generative art sketch for designing a book slipcase.

Built with [whiskers](https://github.com/abey79/vsvg).

## Run

```bash
cargo run --release
```

## Web

A live version is deployed via GitHub Pages on every push to `main`:
https://tilacog.github.io/book-slipcase/

To run locally:

```bash
cargo build --lib --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/book_slipcase.wasm \
  --out-dir web --out-name book_slipcase --no-modules --no-typescript
python -m http.server -d web 8080
```

## Inspirations

- https://www.youtube.com/watch?v=YnIKGBTMCgo
- https://www.philobiblon.com/slipcase.shtml
- https://biblio-tout.blogspot.com/2012/03/making-simple-slipcases-and-boxes.html
