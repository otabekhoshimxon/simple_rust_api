use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use super::entity::UserEntity;

/// Data Transfer Object (DTO) for creating a new user.
///
/// This struct is used to deserialize incoming JSON payloads from the client's HTTP request.
/// The `ToSchema` derive macro automatically generates the OpenAPI schema for Swagger UI,
/// while `#[schema(example = "...")]` provides mock data to make the API documentation interactive.
#[derive(Deserialize, ToSchema)]
pub struct CreateUserDto {
    /// The chosen username for the new account.
    #[schema(example = "ali")]
    pub username: String,

    /// The user's email address. Used for identification and login.
    #[schema(example = "ali@example.com")]
    pub email: String,

    /// The raw password provided by the client.
    /// This will be hashed in the service layer before being saved to the repository.
    #[schema(example = "secret123")]
    pub password: String,
}

/// Data Transfer Object (DTO) for outgoing user responses.
///
/// This struct is used to serialize data into JSON when sending user information back to the client.
/// It strictly represents public/safe data—sensitive fields like `password_hash`
/// are intentionally excluded from this struct to prevent accidental data leaks.
#[derive(Serialize, ToSchema)]
pub struct UserResponseDto {
    /// The unique identifier of the user.
    pub id: i32,

    /// The user's username.
    pub username: String,

    /// The user's registered email address.
    pub email: String,

    /// The timestamp of when the user account was created, formatted as a string.
    pub created_at: String,
}

/// Provides a standard, idiomatic way to convert an internal `UserEntity` into a public `UserResponseDto`.
///
/// Implementing the `From` trait allows the service layer to easily map database models
/// to response models using `UserResponseDto::from(entity)` or `entity.into()`.
impl From<UserEntity> for UserResponseDto {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            username: entity.username,
            email: entity.email,
            // Convert the internal `chrono::NaiveDateTime` object into a readable String
            created_at: entity.created_at.to_string(),
        }
    }
}