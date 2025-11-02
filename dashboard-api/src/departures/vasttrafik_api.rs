use worker::{Env, Fetch, Headers, Method, Request, RequestInit, Response, Result};
use worker::wasm_bindgen::JsValue;
use crate::models::{VtPeriodApiPlaneraResaPeriodWebPeriodV4PeriodModelsPeriodJourneysPeriodGetJourneysResponse, VtPeriodApiPlaneraResaPeriodWebPeriodV4PeriodModelsPeriodJourneysPeriodJourneyApiModel};
use crate::RefreshTokenBody;

pub struct VasttrafikAPI {
    access_token: String,
    env: Env,
}

pub type Journey = VtPeriodApiPlaneraResaPeriodWebPeriodV4PeriodModelsPeriodJourneysPeriodJourneyApiModel;

impl VasttrafikAPI {
    pub async fn new(env: Env) -> Result<VasttrafikAPI> {
        let access_token = Self::get_token(&env).await?;

        Ok(Self {
            access_token,
            env,
        })
    }

    pub async fn get_journeys(&mut self, from_gid: &str, to_gid: &str) -> Result<Vec<Journey>> {

        let mut response = self.get_journeys_internal(from_gid, to_gid).await?;

        if response.status_code() == 401 {
            self.refresh_token().await?;

            response = self.get_journeys_internal(from_gid, to_gid).await?;
        }

        if response.status_code() != 200 {
            return Err(format!("Västtrafik returned unexpected status code {:?}", response.status_code()).into());
        }

        let data: VtPeriodApiPlaneraResaPeriodWebPeriodV4PeriodModelsPeriodJourneysPeriodGetJourneysResponse = response.json().await?;

        Ok(data.results.unwrap().unwrap())
    }

    async fn get_journeys_internal(&self, from_gid: &str, to_gid: &str) -> Result<Response> {
        let headers = Headers::new();
        headers.append("Authorization", &format!("Bearer {}", self.access_token))?;
        let mut init = RequestInit::new();
        init.with_method(Method::Get);
        init.with_headers(headers);
        let request = Request::new_with_init(&format!("https://ext-api.vasttrafik.se/pr/v4/journeys?originGid={}&destinationGid={}&dateTimeRelatesTo=departure&limit=2&onlyDirectConnections=true&includeNearbyStopAreas=false&useRealTimeMode=false&includeOccupancy=false&bodSearch=false", from_gid, to_gid), &init)?;
        Fetch::Request(request).send().await
    }

    async fn get_token(env: &Env) -> Result<String> {
        let ns = env.durable_object("TOKEN_KEEPER")?;
        let stub = ns.get_by_name("singleton")?;
        let mut resp = stub.fetch_with_str("http://do/token").await?;

        Ok(resp.text().await?)
    }

    async fn refresh_token(&mut self) -> Result<()> {
        let ns = self.env.durable_object("TOKEN_KEEPER")?;
        let stub = ns.get_by_name("singleton")?;

        let body = RefreshTokenBody {
            seen_token: self.access_token.clone()
        };
        let json = serde_json::to_string(&body)?;

        let mut headers = Headers::new();
        headers.set("content-type", "application/json; charset=utf-8")?;

        let mut init = RequestInit::new();
        init.with_method(Method::Post);
        init.with_headers(headers);
        init.with_body(Some(JsValue::from_str(&json)));
        /*init.with_body(Some(serde_wasm_bindgen::to_value(&RefreshTokenBody {
            seen_token: self.access_token.clone(),
        })?));*/
        let request = Request::new_with_init("http://do/token", &init)?;
        let mut resp = stub.fetch_with_request(request).await?;
        self.access_token = resp.text().await?;

        Ok(())
    }
}