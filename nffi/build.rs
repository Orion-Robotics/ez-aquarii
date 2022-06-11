// use cxx_build::CFG;
// use std::path::PathBuf;
fn main() {
	// CFG.include_prefix = "/"
    // let opencv2 = pkg_config::probe_library("opencv4").unwrap();
    // let opencv_include_paths = opencv2.include_paths.iter().map(PathBuf::as_path);
    // CFG.exported_header_dirs.extend("/usr/include/opencv4");
    // CFG.exported_header_prefixes = vec!["/usr/include/opencv4"];
    cxx_build::bridge("src/main.rs")
        .file("src/imageprovider.cc")      
        // .file("/usr/include/opencv4/opencv2/core/core.hpp")
        .file("include/raspicam/src/raspicam.h")
        // .file("../raspicam/src/raspicam_cv.h")
        .compile("nffi");
}
