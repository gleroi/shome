#[macro_use]
extern crate diesel;

use std::collections::HashSet;

mod gtfs;
mod schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let routes = gtfs::from_csv::<gtfs::Route>("train/sncf-ter-gtfs/routes.txt")?;
    Ok(())
}

fn from_csv() -> Result<(), Box<dyn std::error::Error>> {
    // Problem: display a route starting from a given stop (stop_point)
    println!("pwd: {:?}", std::env::current_dir()?);

    // find stop Gare de Lyon-Part-Dieu
    let stops = gtfs::from_csv::<gtfs::Stop>("train/sncf-ter-gtfs/stops.txt")?;
    let station = stops
        .iter()
        .find(|&x| x.stop_name == "Gare de Lyon-Part-Dieu" && x.stop_id.starts_with("StopPoint:"))
        .unwrap();
    println!("station: {:?}", station);

    // find all stop/time for this stop
    let stop_times = gtfs::from_csv::<gtfs::StopTime>("train/sncf-ter-gtfs/stop_times.txt")?;
    let station_sts: Vec<&gtfs::StopTime> = stop_times
        .iter()
        .filter(|x| x.stop_id == station.stop_id)
        .collect();
    println!("count stop times {:?}", station_sts.len());

    // find routes passing by the stop/time
    let trips = gtfs::from_csv::<gtfs::Trip>("train/sncf-ter-gtfs/trips.txt")?;
    let routes = gtfs::from_csv::<gtfs::Route>("train/sncf-ter-gtfs/routes.txt")?;
    let mut station_routes = HashSet::new();
    for st in station_sts {
        // a stoptime is part of a trip, which are instances of a route
        let trip_id = &st.trip_id;
        let trip = trips.iter().find(|&t| t.trip_id == *trip_id).unwrap();
        let route = routes.iter().find(|r| r.route_id == trip.route_id).unwrap();
        station_routes.insert(route);
    }

    for r in station_routes {
        println!("{} {}", r.route_id, r.route_long_name);
    }
    Ok(())
}
