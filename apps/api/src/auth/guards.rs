use axum::extract::Extension;

use crate::auth::claims::Claims;

use axum::{
    body::Body,
    extract::Request, // Axum 0.7 style
    http::StatusCode,
    middleware::Next,
    response::Response,
};

// pub async fn require_trainer(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
//     // Extract the claims previously inserted by your auth_middleware
//     let extensions = request.extensions();
//     let claims = extensions.get::<Claims>().ok_or(StatusCode::UNAUTHORIZED)?;

//     // Check if "trainer" exists in the realm_access.roles vector
//     let is_trainer = claims
//         .realm_access
//         .as_ref()
//         .map(|ra| ra.roles.contains(&"trainer".to_string()))
//         .unwrap_or(false);

//     if is_trainer {
//         // Role found, proceed to the handler
//         Ok(next.run(request).await)
//     } else {
//         // Log the failure for debugging in your terminal
//         eprintln!(
//             "❌ Role check failed: User {} missing 'trainer' role",
//             claims.sub
//         );
//         eprintln!("Current roles: {:?}", claims.realm_access);
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }
// pub async fn require_trainer(request: Request, next: Next) -> Result<Response, StatusCode> {
//     // If we reach this line, the token is valid.
//     // Let's just pass everything through to see if the DB works.
//     Ok(next.run(request).await)
// }

pub async fn require_trainer(request: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Make sure this logic is UNCOMMENTED
    let is_trainer = claims
        .realm_access
        .as_ref()
        .map(|ra| ra.roles.contains(&"trainer".to_string()))
        .unwrap_or(false);

    if is_trainer {
        Ok(next.run(request).await)
    } else {
        // This protects your data!
        eprintln!("❌ Role check failed for user: {}", claims.sub);
        Err(StatusCode::FORBIDDEN) // Changed to 403 to indicate "Logged in, but no permission"
    }
}

pub fn require_client(Extension(claims): Extension<Claims>) -> Result<(), StatusCode> {
    if claims.has_role("client") {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
