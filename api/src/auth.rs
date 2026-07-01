use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

impl User {
    pub fn sanitize_id(id: &str) -> String {
        id.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let id = parts
            .headers
            .get("Remote-User")
            .and_then(|v| v.to_str().ok())
            .map(User::sanitize_id)
            .unwrap_or_else(|| "dev".to_string());
        let email = parts
            .headers
            .get("Remote-Email")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let name = parts
            .headers
            .get("Remote-Name")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        Ok(User { id, email, name })
    }
}
