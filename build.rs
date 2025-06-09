fn main() {
    #[cfg(feature = "build-time")]
    wesl::Wesl::new("src/shaders")
        .add_package(&lygia::lygia::math::cubic::Mod)
        .build_artifact("main", "main");
}