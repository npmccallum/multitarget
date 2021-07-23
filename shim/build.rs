fn main() {
    println!("cargo:rustc-link-arg-bin=shim=-Wl,--sort-section=alignment");
    println!("cargo:rustc-link-arg-bin=shim=-nostartfiles");
}
