/// Gets you the total amount of XP needed to reach a certain level.
fn get_xp_for_level(level: usize) -> usize {
    let mut xp = 0;
    for i in 0..level {
        xp += 5 * i.pow(2) + 50 * i + 100;
    }
    xp
}

/// Gets you the progress towards the next level based on your current level and total XP earned.
pub fn get_level_progress(level: usize, total_xp: usize) -> f64 {
    let xp = get_xp_for_level(level) as f64;

    (total_xp as f64 - xp) / (get_xp_for_level(level + 1) as f64 - xp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_xp_for_level() {
        assert_eq!(get_xp_for_level(0), 0);
        assert_eq!(get_xp_for_level(1), 100);
        assert_eq!(get_xp_for_level(2), 255);
        assert_eq!(get_xp_for_level(10), 4675);
        assert_eq!(get_xp_for_level(20), 23850);
        assert_eq!(get_xp_for_level(80), 1003400);
    }

    #[test]
    fn test_get_level_progress() {
        assert_eq!(get_level_progress(1, 100), 0.0);
        assert_eq!(get_level_progress(2, 255), 0.0);
        assert_eq!(get_level_progress(10, 4675), 0.0);
        assert_eq!(get_level_progress(20, 23850), 0.0);
        assert_eq!(get_level_progress(80, 1003400), 0.0);

        assert_eq!(get_level_progress(1, 255), 1.0);
        assert_eq!(get_level_progress(2, 475), 1.0);
        assert_eq!(get_level_progress(10, 5775), 1.0);
        assert_eq!(get_level_progress(20, 26950), 1.0);
        assert_eq!(get_level_progress(80, 1039500), 1.0);
    }
}
