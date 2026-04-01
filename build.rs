fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let rsc_path = std::path::PathBuf::from(&manifest_dir).join("rsc");
    
    println!("cargo:rustc-link-arg=--preload-file");
    println!("cargo:rustc-link-arg={}@/rsc", rsc_path.display());
    println!("cargo:rustc-link-arg=-sWASM");
    println!("cargo:rustc-link-arg=-sMEMORY64=0");
    println!("cargo:rustc-link-arg=-sASSERTIONS");
}