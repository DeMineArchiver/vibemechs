use std::{env, path::PathBuf};

fn main() {
  let dist = cmake::Config::new("lib/uiohook")
    // .define("BUILD_SHARED_LIBS", "ON")
    .always_configure(true)
    .generator("Visual Studio 17 2022")
    // .define("CMAKE_C_COMPILER", "clang-cl.exe")
    // .define("CMAKE_CXX_COMPILER", "clang-cl.exe")
    .build();

  println!("cargo:rustc-link-search=native={}", dist.join("lib").display());
  println!("cargo:rustc-link-lib=static=uiohook");

  let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .generate()
    .expect("Unable to generate bindings");
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
      .expect("Could not save the bindings");
}
