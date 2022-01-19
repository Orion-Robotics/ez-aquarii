use std::io::Result;

fn main() -> Result<()> {
	prost_build::compile_protos(&["./protocol/comms.proto"], &["./protocol"])?;
	Ok(())
}
