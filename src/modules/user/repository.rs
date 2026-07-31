use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use super::entity::UserEntity;

/// Internal storage structure holding the in-memory database state.
///
/// This struct is not exposed publicly. It maintains a `HashMap` to simulate
/// a database table and an auto-incrementing `next_id` to generate primary keys.
#[derive(Default)]
struct MemoryStorage {
    users: HashMap<i32, UserEntity>,
    next_id: i32,
}

/// The Data Access Layer (DAL) for user operations.
///
/// `UserRepository` wraps the internal `MemoryStorage` in an `Arc<RwLock<...>>`.
/// - `Arc` (Atomic Reference Counted) allows the repository state to be cloned and shared
///   safely across multiple asynchronous Tokio threads.
/// - `RwLock` (Read-Write Lock) ensures memory safety by allowing multiple concurrent
///   readers or exactly one exclusive writer at any given time.
#[derive(Clone)]
pub struct UserRepository {
    storage: Arc<RwLock<MemoryStorage>>,
}

impl UserRepository {
    /// Creates a new, empty in-memory repository with the initial ID set to 1.
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(MemoryStorage {
                users: HashMap::new(),
                next_id: 1,
            })),
        }
    }

    /// Inserts a new user into the database.
    ///
    /// Acquires an exclusive write lock, generates a new sequential ID, stores the `UserEntity`,
    /// and returns a clone of the newly created record.
    /// Fails with an error string if the lock is poisoned by a panicked thread.
    pub fn create(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<UserEntity, String> {
        let mut db = self.storage.write().map_err(|_| "Lock failed")?;

        let id = db.next_id;
        db.next_id += 1;

        let user = UserEntity {
            id,
            username,
            email,
            password_hash,
            created_at: Utc::now().naive_utc(),
        };

        db.users.insert(id, user.clone());
        Ok(user)
    }

    /// Retrieves a user by their unique primary key (`id`).
    ///
    /// Acquires a shared read lock and performs an O(1) lookup in the underlying `HashMap`.
    /// Returns `Ok(Some(UserEntity))` if found, or `Ok(None)` if the user does not exist.
    pub fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>, String> {
        let db = self.storage.read().map_err(|_| "Lock failed")?;
        Ok(db.users.get(&id).cloned())
    }

    /// Retrieves a user by their exact email address.
    ///
    /// Acquires a shared read lock and performs an O(N) linear search across all users.
    /// (Note: In a real relational database, this would typically utilize a unique index).
    pub fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, String> {
        let db = self.storage.read().map_err(|_| "Lock failed")?;
        let user = db.users.values().find(|u| u.email == email).cloned();
        Ok(user)
    }

    /// Retrieves all registered users currently in the database.
    ///
    /// Acquires a shared read lock and clones all values from the internal `HashMap`
    /// into a dynamically allocated `Vec`.
    #[allow(dead_code)]
    pub fn find_all(&self) -> Result<Vec<UserEntity>, String> {
        let db = self.storage.read().map_err(|_| "Lock failed")?;
        Ok(db.users.values().cloned().collect())
    }

    /// Removes a user from the database by their `id`.
    ///
    /// Acquires an exclusive write lock and attempts to remove the key-value pair
    /// from the `HashMap`. Returns the removed `UserEntity` if the ID existed,
    /// or `None` if it did not.
    pub fn delete(&self, id: i32) -> Result<Option<UserEntity>, String> {
        let mut db = self.storage.write().map_err(|_| "Lock failed")?;

        // `HashMap::remove` returns the removed value wrapped in an Option.
        Ok(db.users.remove(&id))
    }
}