use std::env::var;
use std::fs;
use std::path::PathBuf;

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

		//HashSet
		let path = entry.path();

		let Some(file_name) = path.file_name().and_then(|os_str| os_str.to_str()) else {
			continue;
		};

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
	let mut redis_names: Vec<&str> = Vec::new();

	for file_name in &copied_file_names {
		let Some((stem, extension)) = file_name.split_once('.') else {
			continue;
		};

		if extension == std::env::consts::DLL_EXTENSION {
			redis_names.push(stem);
		}
	}

	//make sure we have both of the dlls - the steam api and the drm
	assert_eq!(redis_names.len(), 2, "missing Steam API dylib ({}), this target may not be supported", std::env::consts::DLL_EXTENSION);
	println!("cargo:rustc-link-search={}", out_dir.display());

	for redis_name in redis_names {
		println!("cargo:rustc-link-lib=dylib={redis_name}");
	}

	//use bindgen to generate bindings for rust
	#[cfg(feature = "generate_bindings")]
	{
		use bindgen::Formatter;

		let Ok(rust_ver) = bindgen::RustTarget::stable(82, 0) else {
			panic!("bindgen does not support Rust v1.82.0")
		};

		let sdk_dir = steamworks_path.join("headers");
		let headers_dir = sdk_dir.join("steam");
		let bindgen_dt = bindgen::Builder::default()
			.headers([
				headers_dir.join("steam_api_flat.h").to_string_lossy(),
				headers_dir.join("steamencryptedappticket.h").to_string_lossy(),
				//deprecated?
				//no method of getting the interface ptr
				headers_dir.join("isteamappticket.h").to_string_lossy(),
				//Steam docs don't like this, but its fine - I swear
				headers_dir.join("steam_gameserver.h").to_string_lossy(),
			])
			.rust_target(rust_ver)
			.clang_arg("-xc++")
			.clang_arg("-std=c++14")
			.clang_arg(format!("-I{}", sdk_dir.display()))
			.default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
			.formatter(
				//don't waste time formatting the linux version - we will have to anyways
				if cfg!(all(target_os = "linux", target_pointer_width = "64")) {
					Formatter::None
				} else {
					Formatter::Rustfmt
				},
			)
			.generate()
			.expect("steamworks bindgen failed");

		let rs_path = cargo_manifest_dir.join("src").join("bindings").join(&target_os).with_extension("rs");

		#[cfg(any(not(target_os = "linux"), target_pointer_width = "32"))]
		{
			bindgen_dt.write_to_file(rs_path).unwrap();
		}

		//exception for 64-bit linux!
		//bindgen's memory layout tests are borken for a few steamworks interfaces
		//this overrides the ones *known* to be broken
		#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
		{
			use std::collections::HashMap;
			use std::io::Write;
			use syn::fold::Fold;
			use syn::{Expr, ExprLit, GenericArgument, Ident, ItemConst, Lit, LitInt, PathArguments, Stmt};

			struct SynOverrideLayoutTests {
				/// Key: type as written in the source code.
				/// Value: a tuple for the memory layout: `(size: usize, align: usize)`.
				overrides: HashMap<String, (usize, usize)>,
			}

			impl SynOverrideLayoutTests {
				fn fold_item_const_fallible(&mut self, mut i: ItemConst) -> Option<ItemConst> {
					let Expr::Block(const_expr_block) = i.expr.as_mut() else {
						return None;
					};

					for stmt in &mut const_expr_block.block.stmts {
						match stmt {
							Stmt::Expr(Expr::Index(index_expr), _) => {
								match index_expr.index.as_mut() {
									Expr::Binary(binary_expr) => {
										//the left is the fn
										//the op is subtract
										//the right is the usize

										//get the function and type given to the function
										//we're looking for size_of::<Type> and align_of::<Type>
										let (fn_ident, generic_ident): (Ident, Ident) = {
											let Expr::Call(call_expr) = binary_expr.left.as_ref() else {
												continue;
											};

											let Expr::Path(path_expr) = call_expr.func.as_ref() else {
												continue;
											};

											let Some(last_seg) = path_expr.path.segments.last() else {
												continue;
											};

											let fn_ident = last_seg.ident.clone();

											let PathArguments::AngleBracketed(path_args) = &last_seg.arguments else {
												continue;
											};

											let Some(fn_call_generic_ident) = path_args
												.args
												.get(0)
												.and_then(|arg| if let GenericArgument::Type(generic_type) = arg { Some(generic_type) } else { None })
												.and_then(|generic_type| if let syn::Type::Path(type_path) = generic_type { type_path.path.segments.last() } else { None })
												.map(|path_seg| path_seg.ident.clone())
											else {
												continue;
											};

											(fn_ident, fn_call_generic_ident)
										};

										let Some((new_size, new_align)) = self.overrides.get(&generic_ident.to_string()) else {
											continue;
										};

										//the value will we set on the right of the expression
										let new_literal = {
											if fn_ident == "size_of" {
												new_size
											} else if fn_ident == "align_of" {
												new_align
											} else {
												continue;
											}
										};

										let Expr::Lit(ExprLit { lit: Lit::Int(lit_int), .. }) = binary_expr.right.as_mut() else {
											continue;
										};

										*lit_int = LitInt::new(&format!("{new_literal}usize"), lit_int.span());

										//println!("cargo:warning=Overwrote `{}` for type `{}` to {new_size}@{new_align} (dbg {}, {})", fn_ident.to_string(), generic_ident.to_string(), fn_ident == "size_of", fn_ident == "align_of");
									} //arm Expr::Binary(binary_expr)

									_ => continue,
								} //match index_expr.index.as_mut()
							} //arm Stmt::Expr(Expr::Index(index_expr), _)

							_ => continue,
						} //match stmt
					} //for

					Some(i)
				}
			}

			impl Fold for SynOverrideLayoutTests {
				fn fold_item_const(&mut self, mut i: ItemConst) -> ItemConst {
					//make sure the const is named "_" before we do our clone
					if i.ident == "_" {
						if let Some(modified) = self.fold_item_const_fallible(i.clone()) {
							i = modified;
						}
					}

					syn::fold::fold_item_const(self, i)
				}
			}

			let bindings_str = bindgen_dt.to_string();
			let mut syn_file = syn::parse_str::<syn::File>(&bindings_str).unwrap();
			let mut syn_folder = SynOverrideLayoutTests { overrides: HashMap::new() };

			//sometimes on 64-bit Linux, bindgen states raw pointers to interfaces are 4-byte aligned
			//but not for all of the interfaces, only a few
			//this is obviously
			let ptr_sized_types = ["ISteamParties", "ISteamUGC", "ISteamInventory", "ISteamTimeline", "ISteamVideo"];
			let ptr_layout: (usize, usize) = (8, 8);

			for ptr_sized_type in ptr_sized_types {
				syn_folder.overrides.insert(ptr_sized_type.to_string(), ptr_layout);
			}

			//TODO: alert when a specified override didnt happen, or there was no need for the change (e.g. 8@8 -> 8@8)
			//finally, do the post-processing and write the file
			syn_file = syn_folder.fold_file(syn_file);
			let mut file_handle = fs::OpenOptions::new().write(true).truncate(true).create(true).open(rs_path).unwrap();

			file_handle.write_all(prettyplease::unparse(&syn_file).as_bytes()).unwrap();
			file_handle.flush().unwrap();
		}
	}
}
