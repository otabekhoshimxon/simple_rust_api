use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use super::{
    dto::{CreateUserDto, UserResponseDto},
    service::UserService,
};

/// Represents the shared application state injected into every HTTP handler.
///
/// Axum uses the `State` extractor to pass this struct to request handlers safely
/// across multiple asynchronous threads. It holds an instance of `UserService`
/// which contains all the business logic for user operations.
#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
}

/// Handler to create a new user.
///
/// Extracts the `AppState` and a JSON payload (`CreateUserDto`) from the incoming POST request.
/// If successful, returns a `201 Created` status code along with the created user's data.
/// If validation fails (e.g., email already exists), returns a `400 Bad Request`.
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "User successfully created", body = UserResponseDto),
        (status = 400, description = "Bad request (e.g., email already taken)")
    )
)]
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserResponseDto>), (StatusCode, String)> {
    match state.user_service.create_user(payload) {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

/// Handler to fetch a single user by their ID.
///
/// Extracts the `id` from the URL path (e.g., `/api/users/5`).
/// If the user is found, returns a `200 OK` with the user data.
/// If no user exists with the given ID, returns a `404 Not Found`.
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "The unique ID of the user")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponseDto),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponseDto>, (StatusCode, String)> {
    match state.user_service.get_user_by_id(id) {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err((StatusCode::NOT_FOUND, err)),
    }
}

/// Handler to fetch all registered users.
///
/// Returns a JSON array containing all users in the system (`200 OK`).
/// In a real-world production application, this endpoint should typically
/// include pagination parameters (e.g., limit and offset).
#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of all users", body = [UserResponseDto])
    )
)]
pub async fn get_all_users_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponseDto>>, (StatusCode, String)> {
    match state.user_service.get_all_users() {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

/// Handler to delete a user by their ID.
///
/// Extracts the `id` from the URL path, attempts to delete the record in the database/memory.
/// If successful, returns the deleted user's data (`200 OK`).
/// If the user does not exist, returns a `404 Not Found`.
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "The unique ID of the user to delete")
    ),
    responses(
        (status = 200, description = "User successfully deleted", body = UserResponseDto),
        (status = 404, description = "User not found")
    )
)]
pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponseDto>, (StatusCode, String)> {
    match state.user_service.delete(id) {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err((StatusCode::NOT_FOUND, err)),
    }
}