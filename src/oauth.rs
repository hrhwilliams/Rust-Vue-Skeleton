use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, RevocationUrl, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use reqwest;
use serde::{Deserialize, Serialize};

use crate::app::AppState;

const DISCORD_CDN: &str = "http://cdn.discordapp.com";
const DISCORD_AUTHORIZE_URL: &str = "https://discord.com/oauth2/authorize";
const DISCORD_TOKEN_URL: &str = "https://discord.com/api/oauth2/token";
const DISCORD_TOKEN_REVOKE_URL: &str = "https://discord.com/api/oauth2/token/revoke";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiscordInfo {
    pub id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DiscordToken(AccessToken);

impl DiscordInfo {
    pub fn avatar_url(&self) -> Option<String> {
        self.avatar
            .as_ref()
            .map(|avatar_id| format!("{}/avatars/{}/{}.webp?size=512", DISCORD_CDN, self.id, avatar_id))
    }
}

#[derive(Clone, Debug)]
pub enum OAuthError {
    FailedToCreateAuthUrl,
    FailedToStoreAttempt,
    FailedToRetrieveAttempt,
    FailedToGetToken(String),
    FailedQuery,
}

impl FromRef<AppState> for Key {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.key()
    }
}

#[derive(Clone)]
pub struct OAuth {
    client_id: String,
    client_secret: String,
    client_redirect: String,
}

impl OAuth {
    #[must_use]
    pub const fn new(client_id: String, client_secret: String, client_redirect: String) -> Self {
        Self {
            client_id,
            client_secret,
            client_redirect,
        }
    }

    /// Adapted from <https://docs.rs/oauth2/latest/oauth2/>
    pub fn get_oauth_url(&self) -> Result<(String, CsrfToken, PkceCodeVerifier), OAuthError> {
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new(DISCORD_AUTHORIZE_URL.to_string()).map_err(|_| OAuthError::FailedToCreateAuthUrl)?,
            )
            .set_token_uri(TokenUrl::new(DISCORD_TOKEN_URL.to_string()).map_err(|_| OAuthError::FailedToCreateAuthUrl)?)
            .set_revocation_url(
                RevocationUrl::new(DISCORD_TOKEN_REVOKE_URL.to_string())
                    .map_err(|_| OAuthError::FailedToCreateAuthUrl)?,
            )
            .set_redirect_uri(
                RedirectUrl::new(self.client_redirect.clone()).map_err(|_| OAuthError::FailedToCreateAuthUrl)?,
            );

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("identify".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok((auth_url.to_string(), csrf_token, pkce_verifier))
    }

    pub async fn get_token(&self, pkce_verifier: PkceCodeVerifier, code: &str) -> Result<DiscordToken, OAuthError> {
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new(DISCORD_AUTHORIZE_URL.to_string()).map_err(|_| OAuthError::FailedToCreateAuthUrl)?,
            )
            .set_token_uri(TokenUrl::new(DISCORD_TOKEN_URL.to_string()).map_err(|_| OAuthError::FailedToCreateAuthUrl)?)
            .set_revocation_url(
                RevocationUrl::new(DISCORD_TOKEN_REVOKE_URL.to_string())
                    .map_err(|_| OAuthError::FailedToCreateAuthUrl)?,
            )
            .set_redirect_uri(
                RedirectUrl::new(self.client_redirect.clone()).map_err(|_| OAuthError::FailedToCreateAuthUrl)?,
            );

        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token = client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| OAuthError::FailedToGetToken(e.to_string()))?;

        Ok(DiscordToken(token.access_token().clone()))
    }

    pub async fn get_discord_info(&self, token: &DiscordToken) -> Result<DiscordInfo, OAuthError> {
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| OAuthError::FailedQuery)?;

        let user_info: DiscordInfo = http_client
            .get("https://discord.com/api/users/@me")
            .bearer_auth(token.0.secret())
            .send()
            .await
            .map_err(|_| OAuthError::FailedQuery)?
            .json()
            .await
            .map_err(|_| OAuthError::FailedQuery)?;

        Ok(user_info)
    }
}
