// Almost everything shamelessly copied from glib-sys-0.3.3/build.rs.

extern crate pkg_config;

use pkg_config::{Config, Error};
use std::env;
use std::io::prelude::*;
use std::io;
use std::process;

fn main() {
    if env::var("CARGO_FEATURE_STATIC_WLC").is_ok() {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-search=/usr/local/lib64");
        println!("cargo:rustc-link-search=/lib64");
        println!("cargo:rustc-link-search=native=/lib64");
        println!("cargo:rustc-link-lib=dylib=wayland-client");
        println!("cargo:rustc-link-lib=dylib=wayland-server");
        println!("cargo:rustc-link-lib=dylib=systemd");
        println!("cargo:rustc-link-lib=dylib=input");
        println!("cargo:rustc-link-lib=dylib=udev");
        println!("cargo:rustc-link-lib=dylib=GLESv2");
        println!("cargo:rustc-link-lib=dylib=drm");
        println!("cargo:rustc-link-lib=dylib=gbm");
        println!("cargo:rustc-link-lib=dylib=xcb");
        println!("cargo:rustc-link-lib=dylib=xcb-composite");
        println!("cargo:rustc-link-lib=dylib=xcb-ewmh");
        println!("cargo:rustc-link-lib=dylib=xcb-xkb");
        println!("cargo:rustc-link-lib=dylib=xcb-image");
        println!("cargo:rustc-link-lib=dylib=xcb-xfixes");
        println!("cargo:rustc-link-lib=dylib=pixman-1");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=X11-xcb");
        println!("cargo:rustc-link-lib=dylib=EGL");
        println!("cargo:rustc-link-search=native=/usr/include");
        println!("cargo:rustc-link-lib=static=chck-atlas");
        println!("cargo:rustc-link-lib=static=chck-pool");
        println!("cargo:rustc-link-lib=static=chck-buffer");
        println!("cargo:rustc-link-lib=static=chck-buffer");
        println!("cargo:rustc-link-lib=static=chck-dl");
        println!("cargo:rustc-link-lib=static=chck-fs");
        println!("cargo:rustc-link-lib=static=chck-lut");
        println!("cargo:rustc-link-lib=static=chck-pool");
        println!("cargo:rustc-link-lib=static=chck-sjis" );
        println!("cargo:rustc-link-lib=static=chck-string");
        println!("cargo:rustc-link-lib=static=chck-tqueue");
        println!("cargo:rustc-link-lib=static=chck-unicode");
        println!("cargo:rustc-link-lib=static=chck-xdg");
        println!("cargo:rustc-link-lib=static=wlc-protos");
        println!("cargo:rustc-link-lib=static=wlc");
    } else if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}

fn find() -> Result<(), Error> {

    let package_name = "wlc";
    let shared_libs = ["wlc"];

    let version = if cfg!(feature = "0.0.8") {
        "0.0.8"
    } else if cfg!(feature = "0.0.7") {
        "0.0.7"
    } else if cfg!(feature = "0.0.6") {
        "0.0.6"
    } else {
        "0.0.5"
    };

    // Is this Windoze-specific? Do we need to carry this along on Linux with
    // Wayland at all?
    let target = env::var("TARGET").unwrap();
    let hardcode_shared_libs = target.contains("windows");

    let mut config = Config::new();
    config.atleast_version(version);

    if hardcode_shared_libs {
        config.cargo_metadata(false);
    }

    match config.probe(package_name) {
        Ok(library) => {
            if hardcode_shared_libs {
                for lib_ in shared_libs.iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib_);
                }
                for path in library.link_paths.iter() {
                    println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
                }
            }
            Ok(())
        }
        Err(Error::EnvNoPkgConfig(_)) | Err(Error::Command { .. }) => {
            for lib_ in shared_libs.iter() {
                println!("cargo:rustc-link-lib=dylib={}", lib_);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}
