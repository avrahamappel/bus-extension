pub fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371_000.0; // Radius of the Earth in meters
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let delta_phi = (lat2 - lat1).to_radians();
    let delta_lambda = (lon2 - lon1).to_radians();

    let a = f64::sin(delta_phi / 2.0).powi(2)
        + phi1.cos() * phi2.cos() * f64::sin(delta_lambda / 2.0).powi(2);
    let c = 2.0 * f64::atan2(a.sqrt(), f64::sqrt(1.0 - a));

    R * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine() {
        let point1 = (34.0522, -118.2437); // Los Angeles
        let point2 = (34.0522 + 0.0045, -118.2437); // Slightly east
        let expected_distance = 500.0; // Approximate distance in meters

        let calculated_distance = haversine(point1.0, point1.1, point2.0, point2.1);
        assert!(
            (calculated_distance - expected_distance).abs() < 10.0,
            "Expected ~{expected_distance}m but got {calculated_distance}m"
        );
    }
}
