fn main() {
    // https://aeshirey.github.io/code/2020/06/13/simple-c-library-with-rust.html
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
}
