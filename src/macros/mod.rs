/// Macros enable you to write code that writes other code (meta programming)
/// Macros cost no runtime cost, since its code will be expanded during compile time
///
/// Type of Marcro
/// - Declarative Macros with macro_rules!
/// - Procedural Macros

pub use declarative_macros::*;
pub mod declarative_macros;
