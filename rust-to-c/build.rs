extern crate cc;
use std::path::Path;
fn main() {
    cc::Build::new()
        // .file("./src/c/player.c")
        // .file("./src/c/double.c")
        .files(vec!["./src/c/player.c", "./src/c/double.c"])
        .compile("libdouble.a");
}
