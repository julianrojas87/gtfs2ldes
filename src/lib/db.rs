use mongodb::{sync::*, options::*};

pub fn init_db(mongo_uri: &str) -> Client {
    // Get a handle to the deployment.
    let client = Client::with_uri_str(mongo_uri).unwrap() ;
    return client;
}

pub fn create_timeseries<T>(db: &Database, name: &str) -> Collection<T> {
    let ts_opts = TimeseriesOptions::builder()
        .time_field(String::from("generatedAtTime"))
        .meta_field(Some(String::from("isVersionOf")))
        .granularity(Some(TimeseriesGranularity::Seconds))
        .build();

    let coll_opts = CreateCollectionOptions::builder()
        .timeseries(Some(ts_opts))
        .build();

    // Create timeseries collection
    db.create_collection(name, coll_opts).unwrap_or_else(|error| {
        println!("WARN: {}", error);
    });
    
    return db.collection(name);
}
