/// A simple public function for testing semver checks.
pub fn hello() -> &'static str {
    "hello"
}

/// A public struct for testing semver checks.
pub struct Greeter {
    pub name: String,
}

impl Greeter {
    /// Creates a new Greeter.
    pub fn new(name: String) -> Self {
        Greeter { name }
    }

    /// Returns a greeting string.
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}
