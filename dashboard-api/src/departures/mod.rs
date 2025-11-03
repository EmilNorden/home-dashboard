mod vasttrafik_api;

use std::str::FromStr;
use serde::{Deserialize, Serialize};
use worker::{D1Database, Env, Result};
use crate::departures::vasttrafik_api::{Journey, VasttrafikAPI};

#[derive(Default, Serialize)]
pub struct Departures {
    pub stop_name: String,
    pub departures: Vec<Departure>
}

#[derive(Default, Serialize)]
pub struct Departure {
    pub line_name: String,
    pub time: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Deserialize, Debug)]
struct JourneyRow {
    from_gid: String,
    to_gid: String,
}


pub async fn get_departures(env: Env, db: &D1Database) -> Result<Vec<Departures>> {
    let batch_result = db.batch(vec![ db.prepare("SELECT from_gid, to_gid from journeys")]).await?;
    let d1result = &batch_result[0];
    if let Some(error) = d1result.error() {
        return Err(error.into());
    }

    let rows: Vec<JourneyRow> = d1result.results()?;
    if rows.is_empty() {
        return Err("No journeys configured in db".into())
    }

    let mut departures = Vec::new();
    let mut vt_api = VasttrafikAPI::new(env).await?;
    for journey in &rows {


        let journeys = vt_api.get_journeys(&journey.from_gid, &journey.to_gid).await?;

        if !journeys.is_empty() {
            departures.push(Departures {
                stop_name: journeys[0].trip_legs.as_ref().unwrap().as_ref().unwrap()[0].origin.stop_point.name.clone(),
                departures: journeys.iter().map(|x| {
                    let trip_leg = &x.trip_legs.as_ref().unwrap().as_ref().unwrap()[0];

                    Departure {
                        line_name: trip_leg.service_journey.as_ref().unwrap().line.as_ref().unwrap().name.as_ref().unwrap().as_ref().unwrap().clone(),
                        time: chrono::DateTime::from_str(trip_leg.estimated_departure_time.as_ref().unwrap().as_ref().unwrap()).unwrap()
                    }
                }).collect()
            })
        }
    }

    Ok(departures)
}