use cxx_build::CFG;
use std::path::PathBuf;
fn main() {
    let opencv2 = pkg_config::probe_library("opencv4").unwrap();
    let opencv_include_paths = opencv2.include_paths.iter().map(PathBuf::as_path);
    CFG.exported_header_dirs.extend(opencv_include_paths);
    cxx_build::bridge("src/main.rs")
        .file("src/imageprovider.cc")
        .compile("nffi");
}
