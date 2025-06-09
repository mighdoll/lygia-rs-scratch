fn main() {
    println!("cargo:warning=Hello from build.rs!");

    wesl::PkgBuilder::new("lygia")
        .scan_directory(".")
        .expect("failed to scan WESL files")
        .validate()
        .inspect_err(|e| {
            eprintln!("lygia build error: {e}");
            panic!();
        })
        .unwrap()
        .build_artifact()
        .expect("failed to build artifact")
}