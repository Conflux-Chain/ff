#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_arch == "x86_64" {
        cc::Build::new()
            .flag("-c")
            .file("./asm/mul_4.S")
            .compile("libff-derive-crypto.a");
    }
}

#[cfg(any(not(target_arch = "x86_64"), target_os = "windows"))]
fn main() {}
