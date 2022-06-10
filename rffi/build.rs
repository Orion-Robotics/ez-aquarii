// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .compile("rffi");
}