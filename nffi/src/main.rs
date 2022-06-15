// use opencv::{prelude, core, highgui};
// use opencv::prelude::*;
use std::slice;

#[cxx::bridge]
mod ffi {
    struct ImagePacket {
         data: *const u8,
         len: usize,
    }

    unsafe extern "C++" {
        include!("nffi/include/imageprovider.h");
        include!("raspicam/raspicam.h");
        // include!("/usr/include/opencv4/opencv2/");

        type Cam;
        type ImagePacket;

        fn get_image_packet() -> ImagePacket;
        fn initialize_camera() -> ();
        // fn get_image(cam: UniquePtr<Cam>) -> *mut u8;
    }
}
fn main() {
    println!("Hello from Rust!");
    ffi::initialize_camera();
    let pkt = ffi::get_image_packet();
    let imslice = unsafe {
    	slice::from_raw_parts(pkt.data, pkt.len)
    };
    println!("{:?}", imslice)
    // let imslice = unsafe {
    	// slice::from_raw_parts(ffi::get_image(&camera), 200)
   	// };
    // println!("{}", ffi::get_image(camera));
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
