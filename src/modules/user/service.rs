use super::{
    dto::{CreateUserDto, UserResponseDto},
    repository::UserRepository,
};

/// The Service layer responsible for executing core business logic.
///
/// This struct acts as a bridge between the HTTP handlers (controllers) and the
/// Data Access Layer (repository). It enforces business rules (like ensuring unique emails),
/// handles data transformations (like hashing passwords), and maps internal `UserEntity`
/// models to public `UserResponseDto` models.
#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    /// Creates a new instance of the `UserService`, injecting the required `UserRepository`.
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    /// Registers a new user in the system.
    ///
    /// # Business Rules enforced:
    /// 1. Verifies that the provided email is not already registered.
    /// 2. Hashes the raw password before saving it to the repository.
    ///
    /// Returns the newly created user as a `UserResponseDto` on success,
    /// or an error message if the email is already taken.
    pub fn create_user(&self, dto: CreateUserDto) -> Result<UserResponseDto, String> {
        // 1. Check for duplicate email
        if let Some(_) = self.repo.find_by_email(&dto.email)? {
            return Err("Bu email allaqachon ro'yxatdan o'tgan".to_string());
        }

        // 2. Hash the password (simulated here for demonstration purposes)
        // In a real production application, use libraries like `argon2` or `bcrypt`.
        let password_hash = format!("hashed_{}", dto.password);

        // 3. Persist the user using the repository layer
        let user = self.repo.create(dto.username, dto.email, password_hash)?;

        // 4. Transform the internal entity into an external response DTO
        Ok(UserResponseDto::from(user))
    }

    /// Fetches a specific user by their unique identifier.
    ///
    /// Converts the `Option<UserEntity>` returned by the repository into a `Result`.
    /// If the user is missing, it yields a "User not found" error string.
    pub fn get_user_by_id(&self, id: i32) -> Result<UserResponseDto, String> {
        let user = self.repo
            .find_by_id(id)?
            .ok_or_else(|| "Foydalanuvchi topilmadi".to_string())?;

        Ok(UserResponseDto::from(user))
    }

    /// Retrieves all registered users in the system.
    ///
    /// Maps the `Vec<UserEntity>` fetched from the repository directly into
    /// a `Vec<UserResponseDto>` suitable for public API consumption.
    pub fn get_all_users(&self) -> Result<Vec<UserResponseDto>, String> {
        let users = self.repo.find_all()?;

        // Iterates over the entities and maps each one to a DTO
        Ok(users.into_iter().map(UserResponseDto::from).collect())
    }

    /// Removes a user from the system by their ID.
    ///
    /// If the user exists and is successfully deleted, their final state is returned
    /// as a `UserResponseDto`. If the ID does not exist, an error is returned.
    pub fn delete(&self, id: i32) -> Result<UserResponseDto, String> {
        // 1. Attempt to remove the user via the repository layer
        let user = self.repo
            .delete(id)?
            .ok_or_else(|| "Foydalanuvchi topilmadi".to_string())?;

        // 2. Transform the deleted UserEntity into a UserResponseDto and return
        Ok(UserResponseDto::from(user))
    }
}