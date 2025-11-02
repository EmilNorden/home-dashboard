mod models;
pub mod token_manager;
mod departures;

use oauth2::basic::BasicClient;
use oauth2::{ClientId, ClientSecret, TokenResponse, TokenUrl};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use worker::*;
use crate::departures::{get_departures, Departure, Departures};


#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("A new request!!");
    Router::new()
        .get("/", |_req, _ctx| Response::ok("ok"))
        .get_async("/v1/dashboard", dashboard)
        .get_async("/v0/dashboard", fake_dashboard)
        .get_async("/test", test)
        .run(req, env)
        .await
}

pub async fn test(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let token = get_vasttrafik_token(&_ctx.env).await?;

    Response::ok(token)
}

pub async fn fake_dashboard(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let data = DashboardData {
        departures: DepartureData::Results(vec![
            Departures {
                stop_name: "Borås C".to_string(),
                departures: vec![
                    Departure {
                        line_name: "Buss X".to_string(),
                        time: Default::default(),
                    }
                ],
            }
        ]),
        weather: WeatherData::Results(Weather {
            temperature: 0,
            rain_likely: false,
        })
    };

    Response::from_json(&data)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

async fn get_vasttrafik_journeys(token: &str, from_gid: &str, to_gid: &str) -> Result<Response> {
    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", token))?;
    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);
    let request = Request::new_with_init(&format!("https://ext-api.vasttrafik.se/pr/v4/journeys?originGid={}&destinationGid={}&dateTimeRelatesTo=departure&limit=2&onlyDirectConnections=true&includeNearbyStopAreas=false&useRealTimeMode=false&includeOccupancy=false&bodSearch=false", from_gid, to_gid), &init)?;
    Fetch::Request(request).send().await
}

async fn get_vasttrafik_token(env: &Env) -> Result<String> {
    console_debug!("get_vasttrafik_token env={:?}", env);
    let ns = env.durable_object("TOKEN_KEEPER")?;
    let stub = ns.get_by_name("singleton")?;
    //console_debug!("before calling DO");
    let mut resp = stub.fetch_with_str("http://do/token").await?;
    //console_debug!("after calling DO");
    //Ok(resp.text().await?)
    Ok(resp.text().await.unwrap_or_default())
}

async fn refresh_vasttrafik_token(current_token: &str, env: &Env) -> Result<String> {
    let ns = env.durable_object("TOKEN_KEEPER")?;
    let stub = ns.get_by_name("singleton")?;
    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_body(Some(serde_wasm_bindgen::to_value(&RefreshTokenBody {
        seen_token: current_token.to_string(),
    })?));
    let request = Request::new_with_init("http://do/token", &init)?;
    let mut resp = stub.fetch_with_request(request).await?;
    Ok(resp.text().await?)
}

pub async fn dashboard(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let env = ctx.env;

    let db  =env.d1("dashboard_db")?;

    let departure_data = match get_departures(env, &db).await {
        Ok(departures) => DepartureData::Results(departures),
        Err(e) => DepartureData::Error(format!("An error occurred: {}", e))
    };

    let headers = Headers::new();
    headers.set("content-type", "application/json; charset=utf-8")?;

    let data = DashboardData {
        departures: departure_data,
        weather: WeatherData::Results(Weather {
            temperature: 0,
            rain_likely: false,
        })
    };

    Ok(Response::from_json(&data)?.with_headers(headers))
}

#[derive(Serialize)]
pub enum DepartureData {
    #[serde(rename = "results")]
    Results(Vec<Departures>),
    #[serde(rename = "error")]
    Error(String)
}

#[derive(Serialize)]
pub enum WeatherData {
    #[serde(rename = "results")]
    Results(Weather),
    #[serde(rename = "error")]
    Error(String)
}

#[derive(Serialize)]
pub struct Weather {
    pub temperature: i8,
    pub rain_likely: bool,
}

#[derive(Serialize)]
pub struct DashboardData {
    pub departures: DepartureData,
    pub weather: WeatherData,
}


#[durable_object]
pub struct TokenKeeper {
    state: State,
    env: Env,
    tokens: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenBody {
    pub seen_token: String,
}

impl DurableObject for TokenKeeper {
    fn new(state: State, env: Env) -> Self {
        Self {
            state,
            env,
            tokens: HashMap::new(),
        }
    }

    async fn fetch(&self, mut req: Request) -> Result<Response> {
        match (req.method(), req.path().as_str()) {
            (Method::Get, "/token") => {
                match self
                    .state
                    .storage()
                    .get::<String>(TokenKeeper::VASTTRAFIK_TOKEN)
                    .await
                {
                    Ok(token) => Response::ok(token),
                    Err(_) => match self.refresh_access_token().await {
                        Ok(token) => Response::ok(token),
                        Err(e) => Response::error(e.to_string(), 500),
                    },
                }
            }
            (Method::Post, "/token") => {
                let body: RefreshTokenBody = req.json().await?;
                let stored_token = self
                    .state
                    .storage()
                    .get::<String>(TokenKeeper::VASTTRAFIK_TOKEN)
                    .await?;
                if body.seen_token == stored_token {
                    Response::ok(self.refresh_access_token().await.unwrap())
                } else {
                    Response::ok(stored_token)
                }
            }
            _ => Response::error("Not found", 404),
        }
    }

    async fn alarm(&self) -> worker::Result<Response> {
        Response::ok("OK")
    }
}

impl TokenKeeper {
    const VASTTRAFIK_TOKEN: &'static str = "VT_ACCESS_TOKEN";
    const KV_STORE: &'static str = "DASH_VAR";
    const VASTTRAFIK_CLIENTID_KEY: &'static str = "VASTTRAFIK_CLIENTID";
    const VASTTRAFIK_CLIENTSECRET_KEY: &'static str = "VASTTRAFIK_CLIENT_SECRET";
    async fn refresh_access_token(&self) -> anyhow::Result<String> {
        let kv_store = self.env.kv(Self::KV_STORE)?;
        let token_url = kv_store
            .get("VASTTRAFIK_TOKEN_URL")
            .text()
            .await
            .unwrap()
            .unwrap();
        let client_id = kv_store
            .get(Self::VASTTRAFIK_CLIENTID_KEY)
            .text()
            .await
            .unwrap()
            .unwrap();
        let client_secret = self
            .env
            .secret_store(Self::VASTTRAFIK_CLIENTSECRET_KEY)?.get().await?.unwrap();
        let client = BasicClient::new(ClientId::new(client_id))
            .set_client_secret(ClientSecret::new(client_secret))
            .set_token_uri(TokenUrl::new(token_url)?);

        let http_client = reqwest::ClientBuilder::new()
            //.redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Unable to build HTTP client");

        let token_response = client
            .exchange_client_credentials()
            .request_async(&http_client)
            .await?;

        let token = token_response.access_token().secret().clone();

        self.state
            .storage()
            .put(Self::VASTTRAFIK_TOKEN, token.clone())
            .await?;

        Ok(token)
    }
}
