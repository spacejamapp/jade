//! Services for the JAM index
//!
//! TODO: make graphql, redis, and postgres and sort of middleware.

use crate::Config;

pub mod graphql;
pub mod node;
pub mod postgres;
