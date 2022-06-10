fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/imageprovider.cc")
        .compile("nffi");
}
