/// Declaration of the user domain feature module.
///
/// This statement exposes the `user` module (located in the `src/modules/user/` directory)
/// to the rest of the application. By declaring it here (typically within `src/modules/mod.rs`),
/// the application gains access to all internal sub-modules including handlers, services,
/// repositories, DTOs, and entities.
pub mod user;