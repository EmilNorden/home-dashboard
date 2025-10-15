pub mod token_manager;

use std::collections::HashMap;
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use chrono::{DateTime, Duration, Utc};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::{AuthType, ClientId, ClientSecret, EndpointNotSet, EndpointSet, StandardRevocableToken, TokenResponse, TokenUrl};
use serde::{Deserialize, Serialize};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/v1/dashboard", get(dashboard))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {

    Ok(router().call(req).await?)
}

#[event(scheduled)]
async fn refresh(_ev: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let client_id = env.var("VASTTRAFIK_CLIENT_ID")
        .expect("Missing VASTTRAFIK_CLIENT_ID env var");
    let client_secret = env.secret("VASTTRAFIK_CLIENT_SECRET")
        .expect("Missing VASTTRAFIK_CLIENT_SECRET env var");
    let vasttrafik_token_url = env.var("VASTTRAFIK_TOKEN_URL")
        .expect("Missing VASTTRAFIK_TOKEN_URL env var");

    let vasttrafik_token_url = TokenUrl::new(vasttrafik_token_url.to_string())
        .expect("Unable to parse VASTTRAFIK_TOKEN_URL as a valid URL");

    let mut client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_type(AuthType::BasicAuth)
        .set_token_uri(vasttrafik_token_url);

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Unable to build HTTP client");

    let token_response = client
        .exchange_client_credentials()
        .request_async(&http_client)
        .await
        .expect("Unable to obtain client's credentials");
//token_response.expires_in()

}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

pub async fn dashboard() -> (StatusCode, Json<DashboardData>){
    let json = Json(DashboardData {
        ferry_departure_times: [
            Utc::now(),
            Utc::now(),
        ],
        temperature: 18,
        rain_likely: false,
    });

    (StatusCode::OK, json)
}

#[derive(Serialize)]
pub struct DashboardData {
    pub ferry_departure_times: [chrono::DateTime<Utc>; 2],
    pub temperature: i8,
    pub rain_likely: bool,
}

#[durable_object]
pub struct TokenKeeper {
    tokens: HashMap<String, String>,
}


impl DurableObject for TokenKeeper {
    fn new(state: State, env: Env) -> Self {
        /*let client_id = env.var("VASTTRAFIK_CLIENT_ID")
            .expect("Missing VASTTRAFIK_CLIENT_ID env var");
        let client_secret = env.secret("VASTTRAFIK_CLIENT_SECRET")
            .expect("Missing VASTTRAFIK_CLIENT_SECRET env var");
        let vasttrafik_token_url = env.var("VASTTRAFIK_TOKEN_URL")
            .expect("Missing VASTTRAFIK_TOKEN_URL env var");

        let vasttrafik_token_url = TokenUrl::new(vasttrafik_token_url.to_string())
            .expect("Unable to parse VASTTRAFIK_TOKEN_URL as a valid URL");

        let oauth = BasicClient::new(ClientId::new(client_id.to_string()))
            .set_client_secret(ClientSecret::new(client_secret.to_string()))
            .set_auth_type(AuthType::BasicAuth)
            .set_token_uri(vasttrafik_token_url);*/
        Self {
            tokens: HashMap::new()
        }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        match (req.method(), req.path().as_str()) {
            (Method::Get, "/token") => {
                req.query()
            }
        }
        /*match (req.method(), req.path().as_str()) {
            (Method::Get, "/token") => {
                let access_token: AccessToken = self.state.storage().get("ACCESS_TOKEN")
                    .await?;

                let current_time = Utc::now();
                if access_token.expiry_date_utc() < current_time {
                    Response::ok(access_token.token)
                }
                else {
                    let access_token = self.refresh_access_token().await.unwrap();

                    Response::ok(access_token)
                }
            },
            _ => Response::error("Not found", 404)
        }*/
        Response::ok("OK")
    }

    async fn alarm(&self) -> worker::Result<Response> {
        //self.refresh_access_token().await.unwrap();

        Response::ok("OK")
    }
}
/*
impl TokenKeeper {
    async fn refresh_access_token(&self) -> anyhow::Result<String> {
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Unable to build HTTP client");

        let token_response = self.oauth
            .exchange_client_credentials()
            .request_async(&http_client)
            .await?;

        let expires_at = Duration::from_std(token_response.expires_in().unwrap())?;
        let token = token_response.access_token().secret().clone();
        let access_token = AccessToken {
            token: token.clone(),
            expires_at,
            created_at: Utc::now(),
        };

        self.state.storage().put("ACCESS_TOKEN", access_token).await?;
        self.state.storage().set_alarm(expires_at.to_std().unwrap()).await?;

        anyhow::Ok(token)
    }
}*/