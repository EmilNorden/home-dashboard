mod vasttrafik_api;

use std::str::FromStr;
use serde::Serialize;
use worker::{D1Database, Env, Result};
use crate::departures::vasttrafik_api::VasttrafikAPI;

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

pub async fn get_departures(env: Env, db: &D1Database) -> Result<Vec<Departures>> {
    /*let db_result = db.batch(vec![ db.prepare("SELECT from_gid, to_gid from journeys")]).await?;
    if db_result.is_empty() {
        return Err("No journeys configured in DB".into());
    }*/
    let db_result = vec![0];

    let mut departures = Vec::new();
    let mut vt_api = VasttrafikAPI::new(env).await?;
    for row in db_result {
        //let columns: Vec<String> = row.results()?;

        let from_gid: &str = "9021014002238000"; // &columns[0];
        let to_gid: &str  = "9021014004945000"; // &columns[1];

        let journeys = vt_api.get_journeys(&from_gid, &to_gid).await?;

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