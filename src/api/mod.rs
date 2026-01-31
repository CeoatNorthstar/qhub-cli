pub mod client;
pub mod deepseek;
pub mod ibm_quantum;
pub mod backend;

pub use client::{
    ApiClient, ApiError, AuthResponse, ChatRequest, ChatResponse, LoginRequest, RegisterRequest,
    User,
};
