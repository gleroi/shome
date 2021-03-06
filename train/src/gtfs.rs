use crate::schema::{routes, stops, trips};
use chrono::NaiveTime;
use diesel::{Insertable, Queryable};
use serde::de::{self, DeserializeOwned, Unexpected};
use serde::{Deserialize, Deserializer};

/// Stop (arret/gare)
#[derive(Debug, Deserialize, Queryable, Insertable)]
pub struct Stop {
    pub stop_id: String,
    pub stop_name: String,
    pub stop_desc: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: String,
    pub stop_url: String,
    pub parent_station: String,
}

#[test]
fn deserialize_stops() {
    let entities: Vec<Stop> = from_csv("sncf-ter-gtfs/stops.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
    }
}

/// Route (ligne)
#[derive(Debug, Deserialize, Eq, Queryable, Insertable)]
pub struct Route {
    pub route_id: String,
    pub agency_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: String,
    pub route_type: i32,
    pub route_url: String,
    pub route_color: String,
    pub route_text_color: String,
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.route_id == other.route_id
    }
}

impl std::hash::Hash for Route {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.route_id.hash(state);
    }
}

#[test]
fn deserialize_routes() {
    let entities: Vec<Route> = from_csv("sncf-ter-gtfs/routes.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
    }
}

/// Trip on a route (voyage d'une ligne)
#[derive(Debug, Deserialize, Queryable, Insertable)]
pub struct Trip {
    pub route_id: String,
    pub service_id: i32,
    pub trip_id: String,
    pub trip_headsign: String,
    pub direction_id: i32,
    pub block_id: String,
    pub shape_id: String,
}

#[test]
fn deserialize_trips() {
    let entities: Vec<Trip> = from_csv("sncf-ter-gtfs/trips.txt").unwrap();
    for entity in entities {
        println!("{:?}", entity);
    }
}

#[derive(Debug)]
pub struct Duration {
    d: chrono::Duration,
}

impl From<chrono::Duration> for Duration {
    fn from(d: chrono::Duration) -> Self {
        Duration { d }
    }
}

/// StopTime a train arrival/departure from a stop on a trip (arret dans un voyage)
#[derive(Debug, Deserialize)]
pub struct StopTime {
    pub trip_id: String,
    #[serde(deserialize_with = "time_from_string")]
    pub arrival_time: NaiveTime,
    #[serde(deserialize_with = "time_from_string")]
    pub departure_time: NaiveTime,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub stop_headsign: String,
    pub pickup_type: i32,
    pub drop_off_type: i32,
    pub shape_dist_traveled: String,
}

fn time_from_string<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match parse_time_str(&s) {
        Err(err) => Err(de::Error::invalid_value(
            Unexpected::Str(&s),
            &err.description(),
        )),
        Ok(duration) => Ok(duration),
    }
}

fn parse_time_str(s: &str) -> Result<NaiveTime, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err(Box::from("format should be 11:22:33"));
    }
    let result = parse_time(parts[0], parts[1], parts[2]);
    match result {
        Err(err) => Err(Box::from(format!("{} : format should be 11:22:33", err))),
        Ok(duration) => Ok(duration),
    }
}

fn parse_time(hours: &str, min: &str, sec: &str) -> Result<NaiveTime, Box<dyn std::error::Error>> {
    println!("debug: {} {} {}", hours, min, sec);
    Ok(NaiveTime::from_hms(
        hours.parse()?,
        min.parse()?,
        sec.parse()?,
    ))
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
    service_id: i32,
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
