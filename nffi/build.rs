use cxx_build::CFG;
use std::path::{Path, PathBuf};
fn main() {
	// CFG.include_prefix = "/"
    // let opencv2 = pkg_config::probe_library("opencv4").unwrap();
    // let opencv_include_paths = opencv2.include_paths.iter().map(PathBuf::as_path);
    let paths = vec!["/usr/local/lib", "/usr/local/lib/opencv4"].iter().map(PathBuf::from);
    CFG.exported_header_dirs.extend(
    	vec![
    		&Path::new("/usr/local/lib"),
    		&Path::new("/usr/local/lib/opencv4")
    	],
    );
    // CFG.exported_header_prefixes = vec!["/usr/include/opencv4"];
    cxx_build::bridge("src/main.rs")
        .file("src/imageprovider.cc")
        .includes(vec!["/usr/local/include"])
        // .file("/usr/include/opencv4/opencv2/core/core.hpp")
        // .file("../raspicam/src/raspicam_cv.h")
        .compile("nffi");
    println!("cargo:rustc-link-lib=dylib=raspicam");
    println!("cargo:rustc-link-lib=dylib=opencv2");
}
