#![feature(trait_alias)]
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
