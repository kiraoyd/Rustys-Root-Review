///allows the project to build without the database running
/// Code courtesy of Bart Massey
fn main() {
    #[cfg(feature = "offline")]
    println!("cargo:rustc-env=SQLX_OFFLINE=true")
}