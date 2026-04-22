use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Direction {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
    E,
    W,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct BusPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub heading: Direction,
    pub heading_degrees: f64,
    pub time: DateTime,
    pub speed: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUS_LOCATION_JSON: &str = r#"{"Latitude":43.7395222,"Longitude":-79.4443416,"Heading":"E","HeadingDegrees":74.0,"Time":"2026-01-06T08:59:49","Speed":15.998398780822754}"#;
    const STOP_LOCATIONS_JSON: &str = r#"[{"X":625727.54153501848,"Y":4842863.9580828669,"Longitude":-79.43893765643179,"Latitude":43.728150882692312}]"#;

    #[test]
    fn position_decoding() {
        let bus_location: BusPosition = serde_json::from_str(BUS_LOCATION_JSON).unwrap();
        let stop_locations: Vec<StopPosition> = serde_json::from_str(STOP_LOCATIONS_JSON).unwrap();

        assert_eq!(43.7395222, bus_location.latitude);
        assert_eq!(43.728150882692312, stop_locations[0].latitude);
    }

    #[test]
    fn bus_location_heading_supports_diagonals() {
        let json = r#"{"Latitude":43.7296517,"Longitude":-79.4411623,"Heading":"SW","HeadingDegrees":205.0,"Time":"2026-04-22T09:08:14","Speed":29.998800277709961}"#;
        assert!(matches!(
            serde_json::from_str::<BusPosition>(json),
            Ok(BusPosition { .. })
        ));
    }

    #[test]
    fn bus_reserialization() {
        assert_eq!(
            BUS_LOCATION_JSON,
            &serde_json::to_string(
                &serde_json::from_str::<BusPosition>(BUS_LOCATION_JSON).unwrap()
            )
            .unwrap()
        );
    }
}
