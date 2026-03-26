use std::collections::HashMap;

use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use jiff::{civil::Date, ToSpan, Zoned};
use wasm_bindgen::prelude::*;

use crate::BusPosition;

type History = HashMap<Date, Vec<BusPosition>>;

const STORAGE_KEY: &str = "bus-extension.bus-position-history";

pub fn store_bus_location(bus_position: BusPosition) -> Result<(), JsValue> {
    let mut history: History = LocalStorage::get(STORAGE_KEY).unwrap_or_default();
    let today = Zoned::now().date();

    // Prune history older than 30 days
    prune_history(&mut history, today);

    // Insert current position into today's list
    history
        .entry(today)
        .and_modify(|today_history| {
            today_history.push(bus_position);
        })
        .or_insert(vec![bus_position]);

    // Re-save history
    if let Err(err) = LocalStorage::set(STORAGE_KEY, history) {
        let message = match err {
            StorageError::JsError(e) => e.to_string(),
            StorageError::SerdeError(e) => e.to_string(),
            StorageError::KeyNotFound(_) => unreachable!(),
        };
        return Err(message.into());
    }

    Ok(())
}

fn prune_history(history: &mut History, today: Date) {
    history.retain(|date, _| (today - 30.days()) <= *date);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use jiff::civil::Date;

    #[test]
    fn prune_history() {
        let today = "2026-02-15".parse().unwrap();

        // A history with 15 days and 45 days ago
        let mut history = HashMap::from([
            ("2026-02-01".parse().unwrap(), vec![]),
            ("2026-01-01".parse().unwrap(), vec![]),
        ]);

        super::prune_history(&mut history, today);

        // The first entry should be kept and the second pruned
        assert_eq!(
            vec![&"2026-02-01".parse::<Date>().unwrap()],
            history.keys().collect::<Vec<_>>()
        );
    }
}
