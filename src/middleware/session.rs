use axum::{
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, Expiration, SameSite},
};
use reqwest::StatusCode;
use time::Duration;

use crate::{app::AppState, database::SessionModel};

pub async fn create_session(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let (mut parts, body) = req.into_parts();
    let cookies = CookieJar::from_request_parts(&mut parts, &state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let api_route = parts.uri.path().starts_with("/api");
    let req = Request::from_parts(parts, body);

    if let Some(session_cookie) = cookies.get("__Host-Http-Session") {
        let session_id = session_cookie.value();

        if state
            .db
            .get_session_store(session_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .is_some()
        {
            return Ok((cookies, next.run(req).await));
        }
    }

    // only create new session if user isn't accessing the API
    // (avoids making sessions for bot requests)
    if !api_route {
        let session_id: String = state
            .db
            .new_session()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let cookie = Cookie::build(("__Host-Http-Session", session_id))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .path("/")
            .expires(Expiration::Session)
            .max_age(Duration::days(7))
            .build();
        Ok((cookies.add(cookie), next.run(req).await))
    } else {
        Ok((cookies, next.run(req).await))
    }
}
