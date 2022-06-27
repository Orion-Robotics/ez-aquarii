// use opencv::{prelude, core, highgui};
// use opencv::prelude::*;
use std::slice;
use std::thread;
use std::time::Duration;
use anyhow::Result;
// use std::fs::OpenOptions;
// use std::io::Write;
use opencv::core::{Mat, CV_8UC3};
use opencv::highgui;
use std::ffi::c_void;
use std::time::Instant;
use chrono::Local;

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
        fn initialize_camera(w: u32, h: u32, framerate: u32, sensor_mode: u8, shutter_speed: u32) -> ();
        // fn get_image(cam: UniquePtr<Cam>) -> *mut u8;
    }
}
fn main() -> Result<()> {
	let (w, h) = (480, 480);
    ffi::initialize_camera(w, h, 90, 7, 15000);
    thread::sleep(Duration::from_secs(3));
    highgui::named_window("nya", highgui::WINDOW_AUTOSIZE).expect("could not create window");
    let (mut frames, mut last_measure) = (0, Instant::now());

    loop {
        frames += 1;
		if last_measure.elapsed() > Duration::from_millis(1000) {
			last_measure = Instant::now();
			println!("{} FPS", frames);
			frames = 0;
		}
    
	    let pkt = ffi::get_image_packet();
	    let mut imslice = unsafe {
	    	slice::from_raw_parts_mut(pkt.data, pkt.len).to_owned().clone()
	    };
	    let mut mat = unsafe { Mat::new_nd_with_data(&[w as i32, h as i32], CV_8UC3, imslice.as_mut_ptr() as *mut c_void, None)? };
	    opencv::imgproc::put_text(
	    	&mut mat,
	    	&format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S.%m")),
	    	opencv::core::Point::new(240, 100),
	    	opencv::imgproc::FONT_HERSHEY_SIMPLEX,
	    	0.5,
			opencv::core::Scalar::new(255.0, 255.0, 255.0, 255.0),
			1,
			opencv::imgproc::LINE_AA,
			false
	    )?;
	    highgui::imshow("nya", &mat).expect("could not display image");
	    if highgui::wait_key(1).expect("failed to wait for keystroke") > -1{
	    	break;
	    }
	    // opencv::imgcodecs::imwrite("bruh.png", &mat, &opencv::core::Vector::new());
	}
    Ok(())
}
