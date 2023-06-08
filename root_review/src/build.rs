fn main() {
    #[cfg(feature = "offline")]
    println!("cargo:rustc-env=SQLX_OFFLINE=true")
}