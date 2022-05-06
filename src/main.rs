mod entities;
mod lib;

use bson::DateTime;
use entities::stop::VersionedStop;
use gtfs_structures::{Gtfs, Stop/*, Trip*/};
use lib::generators::versioned_stop::*;
use lib::*;
use std::error::Error;
//use std::time::Instant;
//use rayon::prelude::*;
use std::sync::Arc;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("INFO: Reading GTFS source...");
    let feed = Gtfs::new("/home/julian/Desktop/nmbs.gtfs.zip")?;
    feed.print_stats();

    // Connect to MongoDB and create main DB
    let mongo_client = db::init_db("mongodb://localhost:27017");
    let database = mongo_client.database("ldes_db");
    println!("INFO: We have a handle for {} MongoDB!", database.name());
    // Create TimeSeries (TS) collection for stops and get a handle for it
    let stops_coll = db::create_timeseries::<VersionedStop>(&database, "ts_stops_coll");
    println!(
        "INFO: We have a handle for {} timeseries collection!",
        stops_coll.name()
    );
    // Timestamp to use as generation time
    let now = DateTime::now();

    // Produce collection of versioned stops
    let stops_vec: Vec<&Arc<Stop>> = feed.stops.values().collect();
    let stops_gen = VersionedStopGenerator::new(
        stops_vec,
        String::from("http://irail.be/stations/NMBS/00{id}#{generatedAtTime}"),
        String::from("http://irail.be/stations/NMBS/00{id}"),
        String::from("http://irail.be/stations/NMBS/{parent_station}"),
        now,
    );

    // Insert versioned stops in MongoDB TS Collection
    match stops_coll.insert_many(stops_gen, None) {
        Ok(result) => println!("INFO: Inserted correctly {} new records in MongoDB", result.inserted_ids.len()),
        Err(error) => println!("ERROR: Something went wrong inserting versioned stops into MongoDB: {}", error)
    }
    
    // Produce collection of connections
    /*let base_uri = String::from("http://irail.be/");
    let trips_vec: Vec<Trip> = feed.trips.into_iter().map(|(_id, trip)| trip).collect();
    let conn_gen = generators::connection::ConnectionGenerator::new(
        trips_vec,
        feed.calendar,
        feed.calendar_dates,
        feed.routes,
        base_uri,
    );

    let mut conn_count = 0usize;
    let t0 = Instant::now();
    for conns in conn_gen {
        if conns.len() > 0 {
            conn_count += conns.len();
        }
    }

    // Process with Rayon
    //let conn_count = conn_gen.par_bridge().map(|conns| conns.len()).reduce(|| 0, |x, y| x + y);
    println!("Produced {} connections in {} ms", conn_count, t0.elapsed().as_millis());*/
    Ok(())
}
