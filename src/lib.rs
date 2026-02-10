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

const CLOSE_DISTANCE_THRESHOLD: f64 = 750.0;
const CLOSE_DISTANCE_FLASH_INTERVAL: i32 = 500;
const CLOSER_DISTANCE_THRESHOLD: f64 = 250.0;
const CLOSER_DISTANCE_FLASH_INTERVAL: i32 = 200;
const PAGE_RELOAD_TIMEOUT: i32 = 60 * 1000;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().ok_or("Window not found")?;
    let document = window.document().ok_or("Document not found")?;

    // find bus location
    let bus_location_element = document
        .query_selector("input#MainContent_NestContent_hfBusLocation")?
        .ok_or("Bus location element not found")?
        .dyn_into::<HtmlInputElement>()?;
    let bus_position: Position = serde_json::from_str(&bus_location_element.value())
        .map_err(|err| format!("Error decoding bus location: {:?}", err.classify()))?;

    // find stop locations
    let stop_locations_element = document
        .query_selector("input#MainContent_NestContent_hfBusStopLocations")?
        .ok_or("Stop locations element not found")?
        .dyn_into::<HtmlInputElement>()?;
    let stop_positions: Vec<Position> = serde_json::from_str(&stop_locations_element.value())
        .map_err(|err| format!("Error decoding stop locations: {:?}", err.classify()))?;

    // calculate distance
    let bus_lat = bus_position.latitude;
    let bus_lon = bus_position.longitude;
    let stop_lat = stop_positions[0].latitude;
    let stop_lon = stop_positions[0].longitude;
    let distance = haversine(stop_lat, stop_lon, bus_lat, bus_lon);

    // Add element showing "Distance: distance"
    let distance_str = if distance >= 1000.0 {
        format!("{:.1} kilometers", distance / 1000.0)
    } else {
        format!("{distance:.0} meters")
    };
    let distance_label = document.create_element("label")?;
    distance_label.set_class_name("control-label");
    distance_label.append_with_str_1("Distance:")?;
    let distance_el = document.create_element("label")?;
    distance_el.append_with_str_1(&distance_str)?;
    let distance_col = document.create_element("div")?;
    distance_col.set_class_name("col-6");
    distance_col.append_with_node_2(&distance_label, &distance_el)?;
    let route_info = document
        .query_selector("#MainContent_NestContent_lblLatestPosition")?
        .ok_or("Route info element not found")?;
    let first_row = route_info.children().item(0).ok_or("No 1st row")?;
    first_row // Shrink first col to 6
        .children()
        .item(0)
        .ok_or("No 1st col in 1st row")?
        .set_class_name("col-6");
    first_row.append_with_node_1(&distance_col)?;

    // Lights and action when the bus gets closer
    if distance < CLOSE_DISTANCE_THRESHOLD {
        let mut flashing = true;
        let map_element_style = document
            .query_selector("#wheresMyBusOsm .osm-container-outer")?
            .ok_or("Map element not found")?
            .dyn_into::<HtmlElement>()?
            .style();
        let flash_callback = Closure::new(move || {
            let style_change_result = if flashing {
                map_element_style.set_property("border-color", "yellow")
            } else {
                map_element_style
                    .remove_property("border-color")
                    .map(|_| ())
            };
            if let Err(err) = style_change_result {
                panic!(
                    "{}",
                    err.as_string()
                        .unwrap_or("Unknown error changing map border style".into())
                );
            }
            flashing = !flashing;
        });
        let interval = if distance < CLOSER_DISTANCE_THRESHOLD {
            CLOSER_DISTANCE_FLASH_INTERVAL
        } else {
            CLOSE_DISTANCE_FLASH_INTERVAL
        };
        window.set_interval_with_callback_and_timeout_and_arguments_0(
            flash_callback.into_js_value().unchecked_ref(),
            interval,
        )?;
    }

    // sleep 5 seconds then reload page
    let location = window.location();
    let reload_callback = Closure::once_into_js(move || {
        if let Err(err) = location.reload() {
            panic!(
                "{}",
                err.as_string()
                    .unwrap_or("Unknown error reloading page".into())
            );
        }
    });
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        reload_callback.unchecked_ref(),
        PAGE_RELOAD_TIMEOUT,
    )?;

    Ok(())
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
