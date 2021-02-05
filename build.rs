use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
	let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let manifest_path = Path::new(&manifest_dir);

	let wren_lib_dir = manifest_path.join("wren/lib");
	let wren_make_dir = if cfg!(target_os = "macos") {
		manifest_path.join("wren/projects/make.mac")
	} else {
		manifest_path.join("wren/projects/make")
	};

	let status = Command::new("make")
		.current_dir(wren_make_dir)
		.status();
		
	assert!(status.unwrap().success());

	println!("cargo:rustc-link-lib=static=wren");
    println!("cargo:rustc-link-search={}", wren_lib_dir.display());
}