use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer};

/// Stop (arret/gare)
#[derive(Debug, Deserialize)]
pub struct Stop {
    stop_id: String,
    stop_name: String,
    stop_desc: String,
    stop_lat: f64,
    stop_lon: f64,
    zone_id: String,
    stop_url: String,
    parent_station: String,
}

#[test]
fn deserialize_stops() {
    let mut rdr = csv::Reader::from_path("sncf-ter-gtfs/stops.txt").expect("opening file");
    for record in rdr.deserialize() {
        let _stop: Stop = record.unwrap();
    }
}

/// Route (ligne)
#[derive(Debug, Deserialize)]
pub struct Route {
    route_id: String,
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: u32,
    route_url: String,
    route_color: String,
    route_text_color: String,
}

#[test]
fn deserialize_routes() {
    let mut rdr = csv::Reader::from_path("sncf-ter-gtfs/routes.txt").expect("opening file");
    for record in rdr.deserialize() {
        let _route: Route = record.unwrap();
        println!("{:?}", _route);
    }
}

/// Trip on a route (voyage d'une ligne)
#[derive(Debug, Deserialize)]
pub struct Trip {
    route_id: String,
    service_id: u32,
    trip_id: String,
    trip_headsign: String,
    direction_id: u32,
    block_id: String,
    shape_id: String,
}

#[test]
fn deserialize_trips() {
    let mut rdr = csv::Reader::from_path("sncf-ter-gtfs/trips.txt").expect("opening file");
    for record in rdr.deserialize() {
        let _route: Trip = record.unwrap();
        println!("{:?}", _route);
    }
}

/// StopTime a train arrival/departure from a stop on a trip (arret dans un voyage)
#[derive(Debug, Deserialize)]
pub struct StopTime {
    trip_id: String,
    arrival_time: String,
    departure_time: String,
    stop_id: String,
    stop_sequence: u32,
    stop_headsign: String,
    pickup_type: u32,
    drop_off_type: u32,
    shape_dist_traveled: String,
}

#[test]
fn deserialize_stoptimes() {
    let mut rdr = csv::Reader::from_path("sncf-ter-gtfs/stop_times.txt").expect("opening file");
    for record in rdr.deserialize() {
        let _route: StopTime = record.unwrap();
        println!("{:?}", _route);
    }
}

/// Service is a weekly calendar of availability of a Route
#[derive(Debug, Deserialize)]
pub struct Service {
    service_id: u32,
    #[serde(deserialize_with = "bool_from_int")]
    monday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    tuesday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    wednesday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    thursday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    friday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    saturday: bool,
    #[serde(deserialize_with = "bool_from_int")]
    sunday: bool,
    start_date: String,
    end_date: String,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

#[test]
fn deserialize_service_calendar() {
    let services: Vec<Service> = from_csv("sncf-ter-gtfs/calendar.txt").unwrap();
    for service in services {
        println!("{:?}", service);
    }
}

pub fn from_csv<'a, T>(path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: Deserialize<'a>,
{
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for csv_record in rdr.deserialize() {
        let record: T = csv_record?;
        records.push(record);
    }
    Ok(records)
}
