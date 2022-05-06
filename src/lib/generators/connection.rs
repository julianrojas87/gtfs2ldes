use crate::entities::connection::Connection;
use crate::lib::utils;
use chrono::Duration;
use gtfs_structures::{Calendar, CalendarDate, Route, StopTime, Trip};
use std::collections::HashMap;

pub struct ConnectionGenerator {
    trips: Vec<Trip>,
    calendars: HashMap<String, Calendar>,
    calendar_dates: HashMap<String, Vec<CalendarDate>>,
    routes: HashMap<String, Route>,
    base_uri: String,
    index: usize,
}

impl ConnectionGenerator {
    pub fn new(
        trips: Vec<Trip>,
        calendars: HashMap<String, Calendar>,
        calendar_dates: HashMap<String, Vec<CalendarDate>>,
        routes: HashMap<String, Route>,
        base_uri: String
    ) -> ConnectionGenerator {
        ConnectionGenerator {
            trips,
            calendars,
            calendar_dates,
            routes,
            base_uri,
            index: 0,
        }
    }
}

impl Iterator for ConnectionGenerator {
    type Item = Vec<Connection>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.trips.len() - 1 {
            None
        } else {
            let trip = &self.trips[self.index];
            let trip_calendar = self.calendars.get(&trip.service_id)?;
            let trip_calendar_dates = self.calendar_dates.get(&trip.service_id)?;
            let route = self.routes.get(&trip.route_id).unwrap();
            let direction = if let Some(headsign) = &trip.trip_headsign {
                &headsign
            } else {
                &route.long_name
            };

            // Vector that will be returned with all this trip's connections
            let mut connections: Vec<Connection> = Vec::new();
            // Expand calendar and merge with calendar_dates into list of operative service days
            let expanded_calendar = utils::expand_calendar(&trip_calendar, trip_calendar_dates);
            // Iterate over service dates and trip's stop times to produce connections
            for date in expanded_calendar {
                let mut prev_st: &StopTime = &trip.stop_times[0];
                // Stop_times are already ordered by stop_sequence
                for i in 1..trip.stop_times.len() - 1 {
                    connections.push(Connection::builder()
                        // id
                        .id(
                            [
                                &self.base_uri,
                                "connections/",
                                &prev_st.stop.id,
                                "/",
                                &date.format("%Y%m%d").to_string(),
                                "/",
                                &route.short_name,
                                &trip.trip_short_name.as_deref().unwrap()
                            ].concat()
                        )
                        // departure_stop
                        .departure_stop([&self.base_uri, "stations/NMBS/00", &prev_st.stop.id].concat())
                        // arrival_stop
                        .arrival_stop([&self.base_uri, "stations/NMBS/00", &trip.stop_times[i].stop.id].concat())
                        // departure_time
                        .departure_time(
                            date.checked_add_signed(Duration::seconds(
                                prev_st.departure_time.unwrap().into()
                            )).unwrap().timestamp()
                        )
                        // arrival_time
                        .arrival_time(
                            date.checked_add_signed(Duration::seconds(
                                trip.stop_times[i].arrival_time.unwrap().into()
                            )).unwrap().timestamp()
                        )
                        // trip
                        .trip(
                            [
                                &self.base_uri,
                                "vehicle/",
                                &route.short_name,
                                &trip.trip_short_name.as_deref().unwrap(),
                                "/",
                                &date.format("%Y%m%d").to_string()
                            ].concat()
                        )
                        // route
                        .route(
                            [
                                &self.base_uri,
                                "vehicle/",
                                &route.short_name,
                                &trip.trip_short_name.as_deref().unwrap()
                            ].concat()
                        )
                        // direction
                        .direction(direction.to_string())
                        // pickup_type
                        .pickup_type(prev_st.pickup_type)
                        // drop_off_type
                        .drop_off_type(trip.stop_times[i].drop_off_type)
                        .build()
                    );
                    prev_st = &trip.stop_times[i];
                }
            }
            self.index += 1;
            Some(connections)
        }
    }
}
