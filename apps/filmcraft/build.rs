
 //! Windows only: embed Searchifi app icon and version information.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/Searchifi.ico");
    println!("cargo:rerun-if-env-changed=FILMCRAFT_REQUIRE_WINRES");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    windows::embed_resources();
}

#[cfg(windows)]
mod windows {
    pub fn embed_resources() {
        let mut res = winresource::WindowsResource::new();

        res.set_icon("../../assets/app-icon/Searchifi.ico")
            .set("ProductName", "Searchifi Video Editor")
            .set("FileDescription", "Searchifi Video Editor")
            .set(
                "LegalCopyright",
                "Copyright (c) Searchifi. MIT OR Apache-2.0.",
            )
            .set("OriginalFilename", "Searchifi.exe")
            .set("InternalName", "Searchifi");

        if let Err(e) = res.compile() {
            if std::env::var_os("FILMCRAFT_REQUIRE_WINRES").is_some() {
                panic!("Embedding Windows resources failed: {e}");
            }

            println!(
                "cargo:warning=Searchifi.exe built without icon/version resources: {e}"
            );
        }
    }
}

#[cfg(not(windows))]
mod windows {
    pub fn embed_resources() {
        println!(
            "cargo:warning=Windows resources were not embedded \
             because the build host is not Windows."
        );
    }
}
