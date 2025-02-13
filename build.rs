use std::{env, io, path::PathBuf};

fn main() -> io::Result<()> {
	let out_dir = PathBuf::from(
		env::var("OUT_DIR").expect("OUT_DIR ENVIRONMENT VAIRABLE NOT SET")
	);
	
	tonic_build::configure()
		.file_descriptor_set_path(out_dir.join("calculator_descriptor.bin"))
		.compile_protos(&["proto/calculator.proto"], &["proto"])?;

	tonic_build::compile_protos("proto/calculator.proto")?;

	Ok(())
}