use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use gtfs_structures::{Calendar, CalendarDate, Exception};
use rfc6570_level_2::UriTemplate;
use std::collections::{BTreeMap, HashSet, HashMap};

pub fn is_useful_calendar(calendar: &Calendar) -> bool {
    return calendar.monday
        || calendar.tuesday
        || calendar.wednesday
        || calendar.thursday
        || calendar.friday
        || calendar.saturday
        || calendar.sunday;
}

pub fn expand_calendar(
    calendar: &Calendar,
    calendar_dates: &Vec<CalendarDate>,
) -> HashSet<NaiveDateTime> {
    let mut deleted_dates: HashSet<NaiveDate> = vec![].into_iter().collect();
    let mut service_dates: HashSet<NaiveDateTime> = vec![].into_iter().collect();
    // Extract set of service and deleted dates from calendar_dates
    calendar_dates.into_iter().for_each(|cd| {
        if cd.exception_type == Exception::Deleted {
            deleted_dates.insert(cd.date);
        } else {
            service_dates.insert(NaiveDateTime::new(
                cd.date,
                NaiveTime::from_num_seconds_from_midnight(0, 0),
            ));
        }
    });
    // Expand calendar and merge with service dates
    if is_useful_calendar(calendar) {
        let mut curr_date = calendar.start_date;

        while calendar.end_date >= curr_date {
            if calendar.valid_weekday(curr_date) {
                if !deleted_dates.contains(&curr_date) {
                    // Add valid service date
                    service_dates.insert(NaiveDateTime::new(
                        curr_date,
                        NaiveTime::from_num_seconds_from_midnight(0, 0),
                    ));
                }
            }
            curr_date = curr_date + Duration::days(1);
        }
    }

    return service_dates;
}

// Method that builds a URI from a RFC6570 template and a key-value HashMap.
pub fn build_uri(template: String, params: &HashMap<&str, &str>) -> String {
    let uri = UriTemplate::new(&template).unwrap();
    return uri.expand(&params);
}

pub fn btreemap_to_hashmap<'a>(input: &'a BTreeMap<String, String>) -> HashMap<&'a str, &'a str> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    for (key, value) in input.iter() {
        map.insert(key, value);
    }
    return map;
}
