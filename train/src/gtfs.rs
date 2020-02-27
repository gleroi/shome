use serde::Deserialize;

/// Stop (arret/gare)
#[derive(Debug, Deserialize)]
struct Stop {
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
struct Route {
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
struct Trip {
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
struct StopTime {
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
