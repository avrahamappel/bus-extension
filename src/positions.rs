use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BusPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub heading: Option<Direction>,
    pub heading_degrees: Option<f64>,
    pub time: Option<DateTime>,
    pub speed: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BusPositions {
    Single(BusPosition),
    List(Vec<BusPosition>),
}

impl BusPositions {
    pub fn get(self) -> Result<BusPosition, &'static str> {
        match self {
            Self::Single(bp) => Ok(bp),
            Self::List(mut bps) => {
                if bps.len() > 1 {
                    Err("Multiple bus positions given")
                } else {
                    bps.pop().ok_or("Bus position list was empty")
                }
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopPosition {
    pub latitude: f64,
    pub longitude: f64,
}

#[cfg(test)]
mod tests {
    use jiff::Zoned;

    use super::*;

    const BUS_LOCATION_JSON: &str = r#"{"Latitude":43.7395222,"Longitude":-79.4443416,"Heading":"E","HeadingDegrees":74.0,"Time":"2026-01-06T08:59:49","Speed":15.998398780822754}"#;
    const STOP_LOCATIONS_JSON: &str =
        r#"[{"X":625727.54153501848,"Y":4842863.9580828669,"Longitude":-79.43,"Latitude":43.72}]"#;

    #[test]
    fn position_decoding() {
        let bus_location: BusPosition = serde_json::from_str(BUS_LOCATION_JSON).unwrap();
        let stop_locations: Vec<StopPosition> = serde_json::from_str(STOP_LOCATIONS_JSON).unwrap();

        assert_eq!(43.7395222, bus_location.latitude);
        assert_eq!(43.72, stop_locations[0].latitude);
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
    fn bus_locations_can_decode_from_single() {
        let json = r#"{"Latitude":43.7369693,"Longitude":-79.4339603,"Label":""}"#;
        assert!(matches!(
            serde_json::from_str(json),
            Ok(BusPositions::Single(BusPosition { .. }))
        ));
    }

    #[test]
    fn bus_locations_can_decode_from_list() {
        let json = r#"[{"Latitude":43.7369693,"Longitude":-79.4339603,"Label":""}]"#;
        assert!(matches!(
            serde_json::from_str(json),
            Ok(BusPositions::List(_))
        ));
    }

    #[test]
    fn bus_locations_single_get_returns_single() {
        let bus_positions = BusPositions::Single(BusPosition {
            latitude: 43.75,
            longitude: -79.45,
            heading: Some(Direction::SE),
            heading_degrees: Some(12.34),
            speed: Some(45.0),
            time: Some(Zoned::now().datetime()),
        });

        assert_eq!(43.75, bus_positions.get().unwrap().latitude);
    }

    #[test]
    fn bus_locations_list_get_returns_first() {
        let bus_positions = BusPositions::List(vec![BusPosition {
            latitude: 43.75,
            longitude: -79.45,
            heading: Some(Direction::SE),
            heading_degrees: Some(12.34),
            speed: Some(45.0),
            time: Some(Zoned::now().datetime()),
        }]);

        assert_eq!(43.75, bus_positions.get().unwrap().latitude);
    }

    #[test]
    fn bus_locations_list_get_returns_err_if_empty() {
        let bus_positions = BusPositions::List(vec![]);

        assert!(matches!(bus_positions.get(), Err(_)));
    }

    #[test]
    fn bus_locations_list_get_returns_err_if_more_than_one() {
        let bus_positions = BusPositions::List(vec![
            BusPosition {
                latitude: 43.75,
                longitude: -79.45,
                heading: Some(Direction::SE),
                heading_degrees: Some(12.34),
                speed: Some(45.0),
                time: Some(Zoned::now().datetime()),
            },
            BusPosition {
                latitude: 43.73,
                longitude: -79.41,
                heading: Some(Direction::S),
                heading_degrees: Some(12.45),
                speed: Some(47.0),
                time: Some(Zoned::now().datetime()),
            },
        ]);

        assert!(matches!(bus_positions.get(), Err(_)));
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
