use mongodb::{sync::*};

pub fn init_db(mongo_uri: &str) -> Client {
    // Get a handle to the deployment.
    let client = Client::with_uri_str(mongo_uri).unwrap() ;
    return client;
}

/*pub fn create_timeseries(db: &Database, name: &str) -> Collection {
    let ts_opts = TimeseriesOptions::builder()
        .time_field(String::from("created"))
        .meta_field(Some(String::from("isVersionOf")))
        .granularity(Some(TimeseriesGranularity::Hours))
        .build();

    let coll_opts = CreateCollectionOptions::builder()
        .

}*/
