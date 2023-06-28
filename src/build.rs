use std::{
    path::Path,
    process::Command
};

const FRONTEND_DIR: &str = "../ui";

fn build_frontend<P: AsRef<Path>>(source: P) {
    Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir(source.as_ref())
        .status()
        .expect("failed to build frontend");
}

fn main() {
    // println!("cargo:rerun-if-changed={}/src", FRONTEND_DIR);
    println!("cargo:rerun-if-changed={}/index.html", FRONTEND_DIR);
    build_frontend(FRONTEND_DIR);
}
