use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

mod haversine;

use haversine::haversine;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Position {
    latitude: f64,
    longitude: f64,
}

#[wasm_bindgen(start)]
fn main() {
    let window = web_sys::window().expect("Window not found");
    let document = window.document().expect("Document not found");

    // find bus location
    let bus_location_element = document
        .query_selector("input#MainContent_NestContent_hfBusLocation")
        .expect("Invalid query")
        .expect("Bus location element not found")
        .dyn_into::<HtmlInputElement>()
        .expect("Element is not an input");
    let bus_position: Position =
        serde_json::from_str(&bus_location_element.value()).expect("Couldn't decode bus location");

    // find stop locations
    let stop_locations_element = document
        .query_selector("input#MainContent_NestContent_hfBusStopLocations")
        .expect("Invalid query")
        .expect("Stop locations element not found")
        .dyn_into::<HtmlInputElement>()
        .expect("Element is not an input");
    let stop_positions: Vec<Position> = serde_json::from_str(&stop_locations_element.value())
        .expect("Couldn't decode stop locations");

    // calculate distance
    let bus_lat = bus_position.latitude;
    let bus_lon = bus_position.longitude;
    let stop_lat = stop_positions[0].latitude;
    let stop_lon = stop_positions[0].longitude;
    let distance = haversine(stop_lat, stop_lon, bus_lat, bus_lon);

    // Add element showing "Distance: distance"
    let distance_str = if distance >= 1000.0 {
        format!("Distance: {:.1} kilometers", distance / 1000.0)
    } else {
        format!("Distance: {distance:.0} meters")
    };
    let distance_el = document.create_element("span").expect("Invalid element");
    distance_el.set_inner_html(&distance_str);
    let refresh_btn = document
        .query_selector("input#MainContent_NestContent_MapRefresh")
        .expect("Invalid query")
        .expect("Refresh button not found");
    refresh_btn
        .after_with_node_1(&distance_el)
        .expect("Element injection failed");

    // TODO if distance <500 make some noise

    // sleep 5 seconds then reload page
    let reload_callback = Closure::once(|| {
        web_sys::window()
            .unwrap()
            .location()
            .reload()
            .expect("Page reload failed");
    });
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            reload_callback.as_ref().unchecked_ref(),
            60 * 1000,
        )
        .expect("Timeout set failed");
    reload_callback.forget();
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
