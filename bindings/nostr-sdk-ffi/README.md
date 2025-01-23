# Nostr SDK FFI

## Prerequisites

* Rust: https://www.rust-lang.org/tools/install
* Just: https://just.systems/man/en/ (install with `cargo install just`)
* When building for Android:
  * NDK v26
  * Set the `ANDROID_SDK_ROOT` env variable (ex. Linux: `~/Android/Sdk`, macOS: `~/Library/Android/sdk`)
  * Set the `ANDROID_NDK_HOME` env variable (ex. Linux: `~/Android/Sdk/ndk/<version>`, macOS: `~/Library/Android/sdk/ndk/<version>`)

## Build

### Python

For most users, we recommend using our official Python package: [nostr-sdk](https://pypi.org/project/nostr-sdk/).

If you want to compile from source or need more options, read on.

### Wheel

```bash
just python
```

### Kotlin (android)

For most users, we recommend using our official Kotlin package: [org.rust-nostr:nostr-sdk](https://central.sonatype.com/artifact/org.rust-nostr/nostr-sdk/).

If you want to compile from source or need more options, read on.

#### Android Archive (AAR)

This command will build an AAR file in `ffi/android/lib-release.aar`:

```bash
just aar
```

See [Add your AAR or JAR as a dependency](https://developer.android.com/studio/projects/android-library#psd-add-aar-jar-dependency) in Android's docs for more information on how to integrate such an archive into your project.

### Kotlin Multiplatform

For most users, we recommend using our official Kotlin package: [org.rust-nostr:nostr-sdk-kmp](https://central.sonatype.com/artifact/org.rust-nostr/nostr-sdk-kmp/).

If you want to compile from source or need more options, read on.

#### Build

```bash
just build
```

```bash
just kmp
```

### Swift

For most users, we recommend using our official Swift package: [rust-nostr/nostr-sdk-swift](https://github.com/rust-nostr/nostr-sdk-swift).

If you want to compile from source or need more options, read on.

#### Swift Package

This command will produce a fully configured Swift Package in `swift/`.
See [Adding package dependencies to your app](https://developer.apple.com/documentation/xcode/adding-package-dependencies-to-your-app) in Apple's docs for more information on how to integrate such a package into your project.

```bash
just swift
```

## License

This project is distributed under the MIT software license - see the [LICENSE](../../LICENSE) file for details
