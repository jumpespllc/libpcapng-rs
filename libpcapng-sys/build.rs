use std::env;
#[cfg(feature = "static")]
use std::process::Command;

#[cfg(feature = "static")]
use cmake::Config;

fn main() {
    build();
}

#[cfg(not(feature = "static"))]
fn build() {
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=pcapng");
}

#[cfg(feature = "static")]
fn build() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut proc = Command::new("git").current_dir(out_dir.to_owned())
        .arg("clone")
        .arg("https://github.com/stricaud/libpcapng.git")
        .spawn()
        .expect("error executing git command");
    let s = proc.wait().unwrap();
    if !s.success() {
        panic!("failed to init submodule")
    }

    let dst = Config::new(format!("{}/libpcapng",out_dir)).build_target("pcapng_static").build();
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=pcapng_static");
}