use gtfs_structures::{PickupDropOffType};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, TypedBuilder)]
pub struct Connection {
    pub id: String,
    pub departure_stop: String,
    pub arrival_stop: String,
    pub departure_time: i64,
    pub arrival_time: i64,
    pub trip: String,
    pub route: String,
    pub direction: String,
    #[serde(default)]
    pub pickup_type: PickupDropOffType,
    #[serde(default)]
    pub drop_off_type: PickupDropOffType
}