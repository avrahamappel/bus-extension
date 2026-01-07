use serde::Deserialize;

mod haversine;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Position {
    latitude: f64,
    longitude: f64,
}

fn main() {
    // TODO
    // find bus location
    // find stop locations
    // calculate distance
    // change button to "Distance: distance"
    // if distance <500 make some noise
    //
    // sleep 5 seconds then reload page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_decoding() {
        let bus_location_json = r#"{"Latitude":43.7395222,"Longitude":-79.4443416,"Heading":"E","HeadingDegrees":74.0,"Time":"2026-01-06T08:59:49","Speed":15.998398780822754}"#;
        let stop_locations_json = r#"[{"X":625727.54153501848,"Y":4842863.9580828669,"Longitude":-79.43893765643179,"Latitude":43.728150882692312}]"#;

        let bus_location: Position = serde_json::from_str(bus_location_json).unwrap();
        let stop_locations: Vec<Position> = serde_json::from_str(stop_locations_json).unwrap();

        assert_eq!(43.7395222, bus_location.latitude);
        assert_eq!(43.728150882692312, stop_locations[0].latitude);
    }
}
