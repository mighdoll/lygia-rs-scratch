fn main() {
    #[cfg(feature = "build-time")]
    wesl::Wesl::new("src/shaders")
        .add_package(&lygia::lygia::Mod)
        .build_artifact("main", "main");
}