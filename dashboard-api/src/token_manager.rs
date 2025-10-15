use chrono::{DateTime, Duration, Utc};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use oauth2::{AuthType, ClientId, ClientSecret, EndpointNotSet, EndpointSet, StandardRevocableToken, TokenUrl};
use serde::{Deserialize, Serialize};
use worker::{Env, Method, Request, Response, State};
use worker_macros::durable_object;

pub struct TokenManager {
    client: BasicClient,
}

impl TokenManager {
    pub fn new(client: BasicClient) -> Self {
        Self { client }
    }
}

