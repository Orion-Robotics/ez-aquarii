#![feature(trait_alias)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub mod config;
pub mod ipc;
pub mod math;
pub mod modules;

#[cxx::bridge]
pub mod ffi {
	pub struct ImagePacket {
		data: *mut u8,
		len: usize,
	}

	unsafe extern "C++" {
		include!("controller/include/imageprovider.h");
		include!("raspicam/raspicam.h");

		pub type Cam;
		pub type ImagePacket;

		#[must_use]
		pub fn get_image_packet() -> ImagePacket;
		pub fn initialize_camera(
			w: u32,
			h: u32,
			framerate: u32,
			sensor_mode: u8,
			shutter_speed: u32,
		) -> ();
	}
}
