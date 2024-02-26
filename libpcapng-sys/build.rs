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
     let mut proc =  Command::new("git")
        .arg("submodule")
        .arg("init")
        .spawn()
        .expect("error executing git command");
    let exit_status = proc.wait().expect("error waiting on git command to finish");
    if !exit_status.success() {
        panic!("error cloning libpcapng code");
    }
    let dst = Config::new("libpcapng").build_target("pcapng_static").build();
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=pcapng_static");
}