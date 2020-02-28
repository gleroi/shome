mod gtfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // find each trip and collect all stop by trip
    let trips: Vec<String> = station_sts.iter().map(|&x| x.trip_id.clone()).collect();
    println!("trips: {:?}", trips);
    Ok(())
}
