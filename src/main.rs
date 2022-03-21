mod entities;
mod lib;

use bson::DateTime;
use entities::stop::VersionedStop;
use gtfs_structures::{Gtfs/*, Trip*/};
//use lib::generator;
use std::error::Error;
//use std::time::Instant;
//use rayon::prelude::*;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("Reading GTFS source...");
    let feed = Gtfs::new("/home/julian/Desktop/nmbs.gtfs.zip")?;
    feed.print_stats();
    /*let base_uri = String::from("http://irail.be/");
    let trips: Vec<Trip> = feed.trips.into_iter().map(|(_id, trip)| trip).collect();
    let conn_gen = generator::ConnectionGenerator::new(
        trips,
        feed.calendar,
        feed.calendar_dates,
        feed.routes,
        base_uri,
    );*/

    // Connect to MongoDB and create main DB
    let mongo_client = lib::db::init_db("mongodb://localhost:27017");
    let database = mongo_client.database("ldes_db");
    println!("We have a handle for {} MongoDB!", database.name());

    // Produce collection of versioned stops
    let now = DateTime::now();

    for stop_tuple in feed.stops {
        let vstop = VersionedStop::from_stop(
            &*stop_tuple.1,
            String::from("http://irail.be/stations/NMBS/00{id}#{generatedAtTime}"),
            String::from("http://irail.be/stations/NMBS/00{id}"),
            String::from("http://irail.be/stations/NMBS/{parent_station}"),
            now,
        );
        println!("***************************************");
        println!(
            "@id: {},\nname: {},\nisVersionOf: {},\nparentStation: {},\nlat: {},\nlong: {}",
            vstop.id,
            vstop.name,
            vstop.isVersionOf,
            vstop.parent_station.unwrap_or("No parent".to_string()),
            vstop.latitude,
            vstop.longitude
        )
    }
    // Create LDES for Stops
    /*let stops_ldes =

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
