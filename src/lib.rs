use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement};

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
    console_error_panic_hook::set_once();

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
    let eta_chart = document
        .query_selector("canvas#ETAChart")
        .expect("Invalid query")
        .expect("ETA chart not found");
    eta_chart
        .before_with_node_1(&distance_el)
        .expect("Element injection failed");
    eta_chart.remove();

    // If distance <500, make some noise
    if distance < 500.0 {
        let mut flashing = true;
        let flash_callback = Closure::new(move || {
            let color = if distance < 100.0 { "red" } else { "yellow" };
            let map_element_style = document
                .query_selector("#wheresMyBusOsm .osm-container-outer")
                .expect("Invalid query")
                .expect("Map element not found")
                .dyn_into::<HtmlElement>()
                .expect("Not an HTML element")
                .style();
            if flashing {
                map_element_style
                    .set_property("border-color", color)
                    .expect("Setting border color failed");
            } else {
                map_element_style
                    .remove_property("border-color")
                    .expect("Unsetting border color failed");
            }
            flashing = !flashing;
        });
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                flash_callback.as_ref().unchecked_ref(),
                200,
            )
            .expect("Interval set failed");
        flash_callback.forget();
    }

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
