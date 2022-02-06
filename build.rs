extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("wireshark")
        .define("CMAKE_C_FLAGS", "-std=c99")
        .define("BUILD_wireshark", "OFF")
        .define("BUILD_tshark", "OFF")
        .define("BUILD_rawshark", "OFF")
        .define("BUILD_dumpcap", "OFF")
        .define("BUILD_text2pcap", "OFF")
        .define("BUILD_mergecap", "OFF")
        .define("BUILD_reordercap", "OFF")
        .define("BUILD_editcap", "OFF")
        .define("BUILD_capinfos", "OFF")
        .define("BUILD_captype", "OFF")
        .define("BUILD_randpkt", "OFF")
        .define("BUILD_dftest", "OFF")
        .define("BUILD_corbaidl2wrs", "OFF")
        .define("BUILD_dcerpcidl2wrs", "OFF")
        .define("BUILD_androiddump", "OFF")
        .define("BUILD_sshdump", "OFF")
        .define("BUILD_ciscodump", "OFF")
        .define("BUILD_dpauxmon", "OFF")
        .define("BUILD_randpktdump", "OFF")
        .define("BUILD_udpdump", "OFF")
        .define("BUILD_sharkd", "OFF")
        .define("BUILD_fuzzshark", "OFF")
        .define("BUILD_mmdbresolve", "OFF")
        .define("ENABLE_STATIC", "ON")
        .define("ENABLE_KERBEROS","OFF")
        .define("ENABLE_APPLICATION_BUNDLE", "OFF")
        .define("HAVE_NGHTTP2","ON")
        .build();

    let glib = pkg_config::Config::new()
        .probe("glib-2.0")
        .expect("glib-2.0 not found");

    let mut builder = bindgen::Builder::default().header("wrapper.h");

    // Add glib-2.0 include path
    for include_path in glib.include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path.to_string_lossy()));
    }

    // Add libwireshark include patb
    let mut include_path = dst.clone();
    include_path.push("include");
    include_path.push("wireshark");
    builder = builder.clang_arg(format!("-I{}", include_path.to_string_lossy()));

    // builder = builder.clang_arg(format!("-I{}", "/opt/gaosg/Desktop/Rust/lua-5.2.4/dist/include"));

    let mut library_path = dst;
    library_path.push("lib");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search=native={}", library_path.display());
    // println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu/");
    // println!("cargo:rustc-link-lib=nghttp2");
    println!("cargo:rustc-link-lib=static=wireshark");
    println!("cargo:rustc-link-lib=static=wsutil");
    println!("cargo:rustc-link-lib=static=wiretap");
    // println!("cargo:rustc-link-lib=nghttp2");
    
    println!("cargo:rustc-link-lib=lua");
    println!("cargo:rustc-link-lib=xml2");
    println!("cargo:rustc-link-lib=lz4");
    println!("cargo:rustc-link-lib=k5crypto");
    println!("cargo:rustc-link-lib=krb5");
    
    println!("cargo:rustc-link-lib=gcrypt");
    println!("cargo:rustc-link-lib=gnutls");
    println!("cargo:rustc-link-lib=cares");
    println!("cargo:rustc-link-lib=pcre2-8");

    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=gmodule-2.0");
    println!("cargo:rustc-link-lib=gthread-2.0");
    println!("cargo:rustc-link-lib=nghttp2");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    builder
        // max_align_t fails (see https://github.com/rust-lang/rust-bindgen/issues/550)
        .blacklist_type("max_align_t")
        // comments contain indented parts that are parsed as doctests
        // see (https://github.com/rust-lang/rust-bindgen/issues/1313 )
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
