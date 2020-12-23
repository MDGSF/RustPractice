cargo build --target=aarch64-linux-android --release
$ANDROID_TOOLCHAIN_ARM64/bin/aarch64-linux-android-strip ./target/aarch64-linux-android/release/flowout

