use serde::{Deserialize, Serialize};

/// Core domain model representing a User in the database or storage layer.
///
/// Unlike Data Transfer Objects (DTOs) which are meant for public API communication,
/// an Entity represents the internal, raw state of the application's data.
/// It contains sensitive information (such as `password_hash`) that should
/// never be directly exposed or serialized back to the client.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEntity {
    /// The unique primary key identifying the user in the storage system.
    pub id: i32,

    /// The user's chosen display name or handle.
    pub username: String,

    /// The user's registered email address.
    pub email: String,

    /// The securely hashed version of the user's password.
    /// Plain-text passwords must never be stored in this field; it should only
    /// contain the output of a cryptographic hashing algorithm (like Argon2 or bcrypt).
    pub password_hash: String,

    /// The exact date and time the user account was created.
    /// Uses `chrono::NaiveDateTime` to represent a date and time without timezone information.
    pub created_at: chrono::NaiveDateTime,
}