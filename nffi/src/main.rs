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
        type Cam;

        fn get_image_packet() -> UniquePtr<ImagePacket>;
        fn get_number() -> u32;
        fn get_camera() -> UniquePtr<Cam>;
        fn get_number_from_camera(cam: UniquePtr<Cam>) -> u32;

    }
}
fn main() {
    println!("Hello from Rust!");
    let _client = ffi::get_image_packet();
    println!("{}", ffi::get_number());
    let camera = ffi::get_camera();    
    println!("{}", ffi::get_number_from_camera(camera));
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
