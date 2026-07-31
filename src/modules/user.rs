/// Module declarations for the user feature.
///
/// This file acts as the entry point for the `user` module, organizing
/// all the architectural layers according to Clean Architecture principles:
/// - `handlers`: HTTP controllers using Axum to process requests and generate responses.
/// - `dto`: Data Transfer Objects for client-server payload validation and OpenAPI schemas.
/// - `entity`: Core domain models representing the internal database/storage state.
/// - `repository`: Data Access Layer (DAL) managing thread-safe in-memory data operations.
/// - `service`: Business logic layer enforcing application rules and coordinating actions.

pub mod handlers;
pub mod dto;
pub mod entity;
pub mod repository;
pub mod service;