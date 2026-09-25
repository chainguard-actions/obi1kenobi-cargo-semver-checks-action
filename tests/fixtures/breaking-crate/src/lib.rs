// NOTE: hello() function has been removed - this is a semver violation!

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
