pub fn get_display_rating(rating: f64, deviation: f64) -> f64 {
    ((rating - (3.0 * deviation)) * 100.0 + 1000.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratings() {
        assert!(get_display_rating(0.0, 0.0) == 1000.0);
        assert!(get_display_rating(25.0, 25.0 / 3.0) == 1000.0);
        assert!(get_display_rating(25.0, 2.0) == 2900.0);
        assert!(get_display_rating(0.0, 8.0) == 0.0);
    }
}
