use std::error::Error;
pub mod statistics_service;
pub mod plan_service;
pub mod repository;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
