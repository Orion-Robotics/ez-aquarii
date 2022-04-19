pub mod config;
pub mod ipc;
pub mod math;
pub mod modules;
pub mod comms {
	include!(concat!(env!("OUT_DIR"), "/comms.rs"));
}
