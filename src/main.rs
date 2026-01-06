fn haversine(lat1, lon1, lat2, lon2) {
    const R = 6371000; // Radius of the Earth in meters
    let phi1 = lat1.radians();
    let phi2 = lat2.radians();
    let delta_phi = (lat2 - lat1).radians();
    let delta_lambda = (lon2 - lon1).radians();

    let a = (delta_phi / 2).sin() ** 2 + phi1.cos() * phi2.cos() * (delta_lambda / 2).sin() ** 2;
    let c = 2 * a.sqrt().atan2((1 - a).sqrt());

    R * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haverine() {
        let point1 = (34.0522, -118.2437); // Los Angeles
        let point2 = (point1.0 + 0.0045, point1.1); // Slightly east
        let expected_distance = 500; // Approximate distance in meters

        let calculated_distance = haversine(point1[0], point1[1], point2[0], point2[1]);
        assert!((calculated_distance - expected_distance).abs() < 10, format!("Expected ~{expected_distance}m but got {calculated_distance}m"));
    }
}
