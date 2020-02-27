use serde::de::{self, DeserializeOwned, Unexpected};
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
    let entities: Vec<Stop> = from_csv("sncf-ter-gtfs/stops.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
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
    let entities: Vec<Route> = from_csv("sncf-ter-gtfs/routes.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
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
    let entities: Vec<Trip> = from_csv("sncf-ter-gtfs/trips.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
    }
}

/// StopTime a train arrival/departure from a stop on a trip (arret dans un voyage)
#[derive(Debug, Deserialize)]
pub struct StopTime {
    trip_id: String,
    #[serde(deserialize_with = "duration_from_string")]
    arrival_time: chrono::Duration,
    #[serde(deserialize_with = "duration_from_string")]
    departure_time: chrono::Duration,
    stop_id: String,
    stop_sequence: u32,
    stop_headsign: String,
    pickup_type: u32,
    drop_off_type: u32,
    shape_dist_traveled: String,
}

fn duration_from_string<'de, D>(deserializer: D) -> Result<chrono::Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err(de::Error::invalid_value(
            Unexpected::Str(&s),
            &"format: 11:22:33",
        ));
    }
    let result = parse_duration(parts[0], parts[1], parts[2]);
    match result {
        Err(err) => Err(de::Error::invalid_value(
            Unexpected::Str(&format!("{}: {}", s, err)),
            &"format: 11:22:33",
        )),
        Ok(duration) => Ok(duration),
    }
}

fn parse_duration(
    hours: &str,
    min: &str,
    sec: &str,
) -> Result<chrono::Duration, Box<dyn std::error::Error>> {
    use chrono::Duration;
    Ok(Duration::hours(hours.parse()?)
        + Duration::minutes(min.parse()?)
        + Duration::seconds(sec.parse()?))
}

#[test]
fn deserialize_stoptimes() {
    let entities: Vec<StopTime> = from_csv("sncf-ter-gtfs/stop_times.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
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
    #[serde(deserialize_with = "naivedate_from_string")]
    start_date: chrono::NaiveDate,
    #[serde(deserialize_with = "naivedate_from_string")]
    end_date: chrono::NaiveDate,
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

fn naivedate_from_string<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let result = chrono::NaiveDate::parse_from_str(&s, "%Y%m%d");
    match result {
        Err(err) => Err(de::Error::invalid_value(
            Unexpected::Str(&format!("{}: {}", s, err)),
            &"format: 20200322",
        )),
        Ok(date) => Ok(date),
    }
}

#[test]
fn deserialize_service_calendar() {
    let entities: Vec<Service> = from_csv("sncf-ter-gtfs/calendar.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
    }
}

pub fn from_csv<T>(path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for csv_record in rdr.deserialize() {
        let record: T = csv_record?;
        records.push(record);
    }
    Ok(records)
}
