fn main() {
    // Tell Cargo to re-run this build script if the CSS file changes
    println!("cargo:rerun-if-changed=resources/style.css");
}
