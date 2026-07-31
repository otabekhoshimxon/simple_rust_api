use crate::modules::user::dto::{CreateUserDto, UserResponseDto};
use crate::modules::user::handlers::{
    create_user_handler, delete_user_handler, get_all_users_handler, get_user_handler, AppState,
};
use modules::user::{repository::UserRepository, service::UserService};

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod modules;

use axum::{
    body::{Body},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
/// Middleware that intercepts, logs, and reconstructs both incoming HTTP request
/// and outgoing HTTP response body payloads.
pub async fn log_request_response(request: Request, next: Next) -> Result<Response, Response> {
    // --- 1. INCOMING REQUEST LOGGING ---
    let (parts, body) = request.into_parts();

    // Read the request body into raw bytes asynchronously
    let req_bytes = body
        .collect()
        .await
        .map_err(|err| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read request body: {}", err))
                .into_response()
        })?
        .to_bytes();

    // Log request body if it is valid UTF-8 text
    if let Ok(body_str) = std::str::from_utf8(&req_bytes) {
        tracing::info!("📥 Incoming Request Body: {}", body_str);
    } else {
        tracing::info!("📥 Incoming Request Body (binary): {} bytes", req_bytes.len());
    }

    // Reconstruct the request so downstream handlers can process it
    let request = Request::from_parts(parts, Body::from(req_bytes));

    // --- 2. EXECUTE HANDLER & CAPTURE RESPONSE ---
    let response = next.run(request).await;

    // --- 3. OUTGOING RESPONSE LOGGING ---
    let (parts, body) = response.into_parts();

    // Read the response body into raw bytes asynchronously
    let res_bytes = body
        .collect()
        .await
        .map_err(|err| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read response body: {}", err))
                .into_response()
        })?
        .to_bytes();

    // Log response body if it is valid UTF-8 text
    if let Ok(body_str) = std::str::from_utf8(&res_bytes) {
        tracing::info!("📤 Outgoing Response Body: {}", body_str);
    } else {
        tracing::info!("📤 Outgoing Response Body (binary): {} bytes", res_bytes.len());
    }

    // Reconstruct the response so the client receives the data correctly
    let response = Response::from_parts(parts, Body::from(res_bytes));
    Ok(response)
}

/// OpenAPI documentation configuration using Utoipa.
///
/// This struct aggregates all path handlers, data models (schemas),
/// and API tags to generate the OpenAPI Specification (JSON schema).
#[derive(OpenApi)]
#[openapi(
    paths(
        modules::user::handlers::create_user_handler,
        modules::user::handlers::get_all_users_handler,
        modules::user::handlers::get_user_handler,
        modules::user::handlers::delete_user_handler,
    ),
    components(
        schemas(CreateUserDto, UserResponseDto)
    ),
    tags(
        (name = "Users", description = "API endpoints for managing user operations")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // 1. Initialize Tracing Subscriber for structured logging
    // Configures log filters (defaulting to debug for this app and tower_http)
    // and outputs formatted log messages to stdout.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::new("debug,tower_http=debug"),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Initialize Application Dependency Injection (DI) Layers
    // Repositories manage memory storage, services encapsulate business logic,
    // and state passes these components across Axum HTTP handlers.
    let user_repo = UserRepository::new();
    let user_service = UserService::new(user_repo);
    let state = AppState { user_service };

    // 3. Build the Axum Router
    // Maps HTTP routes to their respective handler functions, mounts Swagger UI,
    // attaches the custom body-logging and trace middlewares, and injects shared application state.
    let app = Router::new()
        .route("/api/users", post(create_user_handler))
        .route("/api/users", get(get_all_users_handler))
        .route("/api/users/{id}", get(get_user_handler))
        .route("/api/users/{id}", delete(delete_user_handler))
        // Custom middleware to log the incoming HTTP request payload content
        .layer(axum::middleware::from_fn(log_request_response))
        // Mount Swagger UI interface at /swagger-ui serving OpenAPI specs from /api-docs/openapi.json
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Middleware layer to automatically log incoming request metadata and outgoing responses
        .layer(TraceLayer::new_for_http())
        // Supply shared state to all handlers
        .with_state(state);

    // 4. Bind TCP Listener and Start the HTTP Server
    // Binds to all network interfaces (0.0.0.0) using the port from the `PORT`
    // environment variable (as most hosting platforms assign it), falling back to 3000.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("🚀 Server running on: http://localhost:{port}");
    tracing::info!("📚 Swagger UI available at: http://localhost:{port}/swagger-ui");

    // Launch the Axum server using the Tokio async runtime
    axum::serve(listener, app).await.unwrap();
}