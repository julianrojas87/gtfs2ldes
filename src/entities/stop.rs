#![allow(non_snake_case)]

use crate::lib::utils;
use gtfs_structures::{LocationType, Stop};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use structmap::ToMap;
use structmap_derive::ToMap;
use typed_builder::TypedBuilder;
use bson::DateTime;

/// An intermediary struct that is a subset copy of the gtfs_structures::Stop struct.
/// It is defined for being able to apply the structmap::ToMap derive macro
#[derive(Clone, Default, ToMap)]
pub struct Stop2 {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub location_type: String,
    pub parent_station: String,
    pub platform_code: String,
    pub longitude: f64,
    pub latitude: f64,
}

impl Stop2 {
    pub fn from_stop(stop: &Stop) -> Stop2 {
        Stop2 {
            id: stop.id.to_string(),
            code: stop.code.as_ref().unwrap_or(&String::from("")).to_string(),
            name: stop.name.to_string(),
            description: stop.description.to_string(),
            location_type: match stop.location_type {
                LocationType::StopPoint => String::from("Stop"),
                LocationType::StopArea => String::from("Station"),
                LocationType::StationEntrance => String::from("Entrance_Exit"),
                LocationType::GenericNode => String::from("GenericNode"),
                LocationType::BoardingArea => String::from("BoardingArea"),
                LocationType::Unknown(_i32) => String::from("Stop"),
            },
            parent_station: stop
                .parent_station
                .as_ref()
                .unwrap_or(&String::from(""))
                .to_string(),
            platform_code: stop
                .platform_code
                .as_ref()
                .unwrap_or(&String::from(""))
                .to_string(),
            longitude: stop.longitude.unwrap(),
            latitude: stop.latitude.unwrap(),
        }
    }
}

// This is the struct that will go into the LDES
#[derive(Debug, Serialize, TypedBuilder)]
pub struct VersionedStop {
    pub id: String,
    pub name: String,
    pub generatedAtTime: DateTime,
    pub isVersionOf: String,
    pub locationType: String,
    pub code: Option<String>,
    pub parent_station: Option<String>,
    pub platform_code: Option<String>,
    pub longitude: f64,
    pub latitude: f64,
}

impl VersionedStop {
    pub fn from_stop(
        stop: &Stop,
        version_template: String,
        member_template: String,
        parent_template: String,
        now: DateTime,
    ) -> VersionedStop {
        // Build Stop instance that we can introspect
        let stop2 = Stop2::from_stop(stop);
        // Convert Stop2 struct to BTreeMap of field names and values (via derived structmap macro) 
        // that we can use for URI building
        let btreemap: BTreeMap<String, String> = Stop2::to_stringmap(stop2.clone());
        // Transform BTreeMap to HashMap as required by URI template library
        let mut params: HashMap<&str, &str> = utils::btreemap_to_hashmap(&btreemap);
        // Add generated at time param to be used for URI building
        let now_text = now.to_rfc3339_string();
        params.insert("generatedAtTime", &now_text);
        // Build serializable instance with unique and reproducible URIs  
        VersionedStop::builder()
            .id(utils::build_uri(version_template, &params))
            .name(stop2.name)
            .generatedAtTime(now)
            .isVersionOf(utils::build_uri(member_template, &params))
            .locationType(stop2.location_type)
            .code(match stop2.code.as_str() {
                "" => None,
                _ => Some(stop2.code),
            })
            .parent_station(match stop2.parent_station.as_str() {
                "" => None,
                _ => Some(utils::build_uri(parent_template, &params)),
            })
            .platform_code(match stop2.platform_code.as_str() {
                "" => None,
                _ => Some(stop2.platform_code),
            })
            .longitude(stop2.longitude)
            .latitude(stop2.latitude)
            .build()
    }
}
