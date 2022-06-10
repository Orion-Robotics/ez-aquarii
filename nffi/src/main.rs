// use opencv::{prelude, core, highgui};
use opencv::prelude::*;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("nffi/include/imageprovider.h");

        type ImagePacket;

        fn get_image_packet() -> UniquePtr<ImagePacket>;

    }
}
fn main() {
    println!("Hello, Rust and C++!");
    let client = ffi::get_image_packet();
    let size = vec![3,3];
    // let mat = Mat::zeros_nd(&size, typ: i32);
    // highgui::imshow("sus", )
}
// // use anyhow::anyhow;
// use anyhow::Result;
// // use image::RgbImage;
// // use ndarray::{Array1, ArrayView1, ArrayView3};
// use opencv::{self as cv, prelude::*};
// fn main() -> Result<()> {
//     // Read image
//     let img = cv::imgcodecs::imread("madeline.png", cv::imgcodecs::IMREAD_COLOR)?;
//     cv::highgui::imshow("sus", &img).expect("weirdchamp");
//     let key = cv::highgui::wait_key(0)?;
//     Ok(())
// }