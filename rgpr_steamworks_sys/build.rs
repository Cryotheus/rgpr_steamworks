use std::env::consts::DLL_EXTENSION;
use std::env::var;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

// copy a directory's files to another,
// creating a list of 
fn copy_files(search_dir: impl Into<PathBuf>, copy_dir: impl Into<PathBuf>) -> Result<Vec<String>, std::io::Error> {
	let copy_dir = copy_dir.into();
	let mut copied_file_names: Vec<String> = Vec::new();
	let search_dir = search_dir.into();

	for entry in fs::read_dir(&search_dir)? {
		let Ok(entry) = entry else {
			continue;
		};

		let Ok(file_type) = entry.file_type() else {
			continue;
		};

		if file_type.is_dir() {
			continue;
		}

		let path = entry.path();

		let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) else {
			continue;
		};
		
		//ignore the sdk dylibs/libs if we don't have the feature enabled
		#[cfg(not(feature = "sdk_encrypted_app_ticket"))]
		if file_name.contains("sdkencryptedappticket") {
			continue;
		}

		copied_file_names.push(file_name.to_string());

		fs::copy(&path, copy_dir.join(file_name))?;
	}

	Ok(copied_file_names)
}

fn main() {
	let cargo_manifest_dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());
	let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
	let target_os = var("CARGO_CFG_TARGET_OS").unwrap();
	let target_pointer_width = var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
	let steamworks_path = cargo_manifest_dir.join("lib").join("steamworks_sdk");
	let steamworks_redis_path = steamworks_path.join(&target_os).join(&target_pointer_width);
	let copied_file_names = copy_files(steamworks_redis_path, &out_dir).unwrap();

	//the dylibs are required to setup a steam pipe
	//unless you hack together a solution yourself
	assert!(!copied_file_names.is_empty(), "missing redistributables");

	//`DLL_EXTENSION` is for the host, not the target
	let dll_extension: &str = match target_os.as_str() {
		"linux" => "so",
		"macos" | "apple" => "dylib",
		"windows" => "dll",
		os => {
			println!("cargo:warning=failed to resolve dylib extension for os `{os}`, defaulting to the host's extension of `{DLL_EXTENSION}` which will break cross-compilation");

			DLL_EXTENSION
		}
	};

	let mut redis_names: Vec<&str> = Vec::new();

	//link the redistributable binaries (dylibs)
	//ignore anything else, as only dylibs need to be adjacent
	for file_name in &copied_file_names {
		let Some((stem, extension)) = file_name.split_once('.') else {
			continue;
		};

		if extension == dll_extension {
			if cfg!(any(target_os = "linux", target_os = "macos")) {
				redis_names.push(stem.trim_start_matches("lib"));
			} else {
				redis_names.push(stem);
			}
		}
	}

	/*
	// error: linking with `cc` failed: exit status: 1
	//   = note: rust-lld: error: unable to find library -llibsdkencryptedappticket
	// 		  rust-lld: error: unable to find library -llibsteam_api
	// 		  collect2: error: ld returned 1 exit status
	//
	// # Why did the error happen?
	// Linux prefixes the requested names of the dylibs with `lib` before trying to search for them
	// so we had `libsteam_api` and linux searched for `liblibsteam_api`
	//
	// # Solution
	// Strip `lib` from the copied file's names

	println!("cargo:rustc-link-lib=dylib=sdkencryptedappticket"); //libsdkencryptedappticket.so
	println!("cargo:rustc-link-lib=dylib=steam_api"); */

	//make sure we have both of the dylibs - the steam api and the drm
	assert!(!redis_names.is_empty(), "missing Steam API dylib ({}), this target may not be supported", DLL_EXTENSION);
	
	//ensure the dir we copy our dylibs to will be searched
	println!("cargo:rustc-link-search={}", out_dir.display());

	//specify the dylibs we need to link to
	for redis_name in redis_names {
		println!("cargo:rustc-link-lib=dylib={redis_name}");
	}

	//generate bindings for rust to the Steam API's functions
	#[cfg(feature = "generate_bindings")]
	rgpr_steamworks_bindgen::generate_bindings(steamworks_path.join("headers"), cargo_manifest_dir.join("src").join("bindings").join(target_os).with_extension("rs")).unwrap();
}
