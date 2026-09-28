/// A simple public function for semver testing with features.
pub fn hello() -> &'static str {
    "hello"
}

/// A public function gated behind the extra feature.
#[cfg(feature = "extra")]
pub fn hello_extra() -> &'static str {
    "hello extra"
}
