//! GraphQL configs

use axum::http::{header, HeaderValue, Method};
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};

/// Config for graphql
#[derive(Debug, Clone)]
pub struct Graphql {
    /// The graphql server address
    pub graphql: SocketAddr,

    /// CORS configuration
    pub cors: Cors,
}

/// CORS configuration for the GraphQL endpoint
#[derive(Debug, Clone)]
pub struct Cors {
    /// Allow all origins (permissive mode - development only)
    pub allow_all_origins: bool,

    /// Allowed origins (comma-separated list)
    pub allowed_origins: String,

    /// Allow credentials in CORS requests
    pub allow_credentials: bool,

    /// CORS max age in seconds
    pub max_age: u64,

    /// Additional allowed headers (comma-separated list)
    pub extra_headers: String,
}

impl Cors {
    /// Build a CORS layer from configuration
    pub fn layer(&self) -> CorsLayer {
        if self.allow_all_origins {
            tracing::warn!("Using permissive CORS configuration (allow all origins) - only suitable for development!");
            return self.permissive();
        }

        let mut cors = CorsLayer::new();

        // Parse and add allowed origins
        let origins: Vec<&str> = self
            .allowed_origins
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for origin in origins {
            if let Ok(header_value) = origin.parse::<HeaderValue>() {
                cors = cors.allow_origin(header_value);
            } else {
                tracing::warn!("Invalid CORS origin: {}", origin);
            }
        }

        // Configure methods
        cors = cors.allow_methods([Method::GET, Method::POST, Method::OPTIONS]);

        // Base headers
        let mut headers = vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
            header::ACCESS_CONTROL_REQUEST_METHOD,
            header::ACCESS_CONTROL_REQUEST_HEADERS,
        ];

        // Add extra headers if specified
        if !self.extra_headers.is_empty() {
            let extra_headers: Vec<&str> = self
                .extra_headers
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            for header_name in extra_headers {
                if let Ok(header) = header_name.parse() {
                    headers.push(header);
                } else {
                    tracing::warn!("Invalid CORS header: {}", header_name);
                }
            }
        }

        cors = cors.allow_headers(headers);
        cors = cors.allow_credentials(self.allow_credentials);
        cors = cors.max_age(Duration::from_secs(self.max_age));
        cors
    }

    /// Build a permissive CORS layer from configuration (allows all origins)
    pub fn permissive(&self) -> CorsLayer {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::OPTIONS,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_headers(Any)
            .allow_credentials(false) // Note: credentials cannot be used with wildcard origins
            .max_age(Duration::from_secs(self.max_age))
    }
}

impl Default for Cors {
    fn default() -> Self {
        Self {
            allow_all_origins: false,
            allowed_origins: "http://localhost:3000,http://localhost:8080,http://127.0.0.1:3000,http://127.0.0.1:8080".to_string(),
            allow_credentials: true,
            max_age: 3600,
            extra_headers: String::new(),
        }
    }
}
