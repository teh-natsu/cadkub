//! Windows only: embed the app icon and version info into `cadkub.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/cadkub.ico");
    println!("cargo:rerun-if-env-changed=CADKUB_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=CADKUB_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/cadkub.ico").exists() {
        res.set_icon("../../assets/app-icon/cadkub.ico");
    }
    res.set("ProductName", "CadKub")
        .set("FileDescription", "CadKub computer-aided design")
        .set("CompanyName", "Nattpol Chaisri")
        .set("LegalCopyright", "Copyright (c) the CadKub authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "cadkub.exe")
        .set("InternalName", "cadkub");
    if let Err(e) = res.compile() {
        if std::env::var_os("CADKUB_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=cadkub.exe built without icon/version resources: {e}");
    }
}
