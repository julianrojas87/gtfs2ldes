use crate::entities::stop::VersionedStop;
use bson::DateTime;
use gtfs_structures::Stop;
use std::sync::Arc;

pub struct VersionedStopGenerator<'a> {
    index: usize,
    stops: Vec<&'a Arc<Stop>>,
    version_template: String,
    member_template: String,
    parent_template: String,
    now: DateTime,
}

impl<'a> VersionedStopGenerator<'a> {
    pub fn new(
        stops: Vec<&'a Arc<Stop>>,
        version_template: String,
        member_template: String,
        parent_template: String,
        now: DateTime,
    ) -> VersionedStopGenerator<'a> {
        VersionedStopGenerator {
            index: 0,
            stops,
            version_template,
            member_template,
            parent_template,
            now,
        }
    }
}

impl<'a> Iterator for VersionedStopGenerator<'a> {
    type Item = VersionedStop;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.stops.len() - 1 {
            None
        } else {
            // Get reference to current stop
            let stop = self.stops[self.index];
            // Advance index
            self.index += 1;
            // Return VersionedStop
            Some(VersionedStop::from_stop(
                &*stop,
                self.version_template.to_string(),
                self.member_template.to_string(),
                self.parent_template.to_string(),
                self.now,
            ))
        }
    }
}
