extern crate cc;
extern crate pkg_config;

fn main() {
  println!("cargo::rerun-if-changed=build.rs");
  println!("cargo::rerun-if-changed=src/config.c");

  // get rtems platform information from target pkgconf files

  let target = "arm-rtems6-stm32f4";

  let arch = pkg_config::get_variable(target, "RTEMS_ARCH").unwrap();
  let version = pkg_config::get_variable(target, "RTEMS_MAJOR").unwrap();
  let include = pkg_config::get_variable(target, "includedir").unwrap();
  let lib = pkg_config::get_variable(target, "libdir").unwrap();
  let flags = pkg_config::get_variable(target, "ABI_FLAGS").unwrap();
  let flags: Vec<&str> = flags.split_whitespace().collect();

  // compile the config file and link rtems libraries

  let mut build = cc::Build::new();
  build.file("src/config.c");
  build.compiler(format!("{arch}-rtems{version}-gcc"));
  build.include(include);
  build.warnings(false);
  for flag in flags { build.flag(flag); }
  build.compile("config");

  println!("cargo::rustc-link-search={lib}");
}
