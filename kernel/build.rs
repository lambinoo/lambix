fn main() {
    println!("cargo:rustc-link-arg=-Wl,-dynamic-linker=lambix-bootloader");
}
