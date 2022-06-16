// use opencv::{prelude, core, highgui};
// use opencv::prelude::*;
use std::slice;
use std::thread;
use std::time::Duration;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use opencv::core::{Mat, CV_8SC3};
use std::ffi::c_void;

#[cxx::bridge]
mod ffi {
    struct ImagePacket {
         data: *mut u8,
         len: usize,
    }

    unsafe extern "C++" {
        include!("nffi/include/imageprovider.h");
        include!("raspicam/raspicam.h");
        // include!("/usr/include/opencv4/opencv2/");

        type Cam;
        type ImagePacket;

        fn get_image_packet() -> ImagePacket;
        fn initialize_camera(w: u32, h: u32) -> ();
        // fn get_image(cam: UniquePtr<Cam>) -> *mut u8;
    }
}
fn main() -> Result<()> {
	let (w, h) = (720, 720);
    ffi::initialize_camera(w, h);
    thread::sleep(Duration::from_secs(3));
    let pkt = ffi::get_image_packet();
    let mut imslice = unsafe {
    	slice::from_raw_parts_mut(pkt.data, pkt.len)
    };
    let mat = unsafe { Mat::new_nd_with_data(&[w as i32, h as i32], CV_8SC3, imslice.as_mut_ptr() as *mut c_void, Some(&[1]))? };
    opencv::imgcodecs::imwrite("bruh.png", &mat, &opencv::core::Vector::new());
    Ok(())
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
