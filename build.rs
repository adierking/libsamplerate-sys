extern crate cmake;

fn main() {
    let manifest_path = env!("CARGO_MANIFEST_DIR");
    let mut config = cmake::Config::new("libsamplerate");
    config
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .define("LIBSAMPLERATE_TESTS", "OFF")
        .define("LIBSAMPLERATE_EXAMPLES", "OFF")
        .define("LIBSAMPLERATE_INSTALL", "OFF");

    if std::env::var("TARGET").unwrap().contains("x86_64-apple-darwin") {
        config
            .define("CMAKE_OSX_ARCHITECTURES", "x86_64");
    } else if std::env::var("TARGET").unwrap().contains("aarch64-apple-darwin") {
        config
            .define("CMAKE_OSX_ARCHITECTURES", "arm64");
    } else if std::env::var("TARGET").unwrap().contains("-ios") {
        config
            .define("CMAKE_TOOLCHAIN_FILE", format!("{}/ios-cmake/ios.toolchain.cmake", manifest_path))
            .define("PLATFORM", "OS64")
            .define("IOS_DEPLOYMENT_TARGET", "11.0")
            .define("CMAKE_OSX_SYSROOT", "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk")
            .generator("Xcode");
    }

    config
        .build_target("samplerate");

    let mut path = config.build().join("build").join("src");

    if std::env::var("TARGET").unwrap().contains("msvc") {
        path = path.join(config.get_profile());
    } else if std::env::var("TARGET").unwrap().contains("-ios") {
        path = path.join(format!("{}-iphoneos", config.get_profile()));
    }

    println!("cargo:rustc-link-search=native={}", path.display());
    println!("cargo:rustc-link-lib=static=samplerate");
}
