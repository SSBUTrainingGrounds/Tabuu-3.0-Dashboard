pub fn get_display_rating(rating: f64, deviation: f64) -> f64 {
    (rating - (3.0 * deviation)) * 100.0 + 1000.0
}
