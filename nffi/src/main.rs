// use opencv::{prelude, core, highgui};
// use opencv::prelude::*;
#[cxx::bridge]
mod ffi {
    pub struct ImagePacket {
        integer: i32,
        float: f32
    }
    unsafe extern "C++" {
        include!("nffi/include/imageprovider.h");
        // include!("/usr/include/opencv4/opencv2/");

        type ImagePacket;

        fn get_image_packet() -> UniquePtr<ImagePacket>;
        fn get_number() -> u32;

    }
}
fn main() {
    println!("Hello from Rust!");
    let _client = ffi::get_image_packet();
    println!("{}", ffi::get_number());
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