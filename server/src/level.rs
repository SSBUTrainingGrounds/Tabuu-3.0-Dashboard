/// Gets you the XP needed until you level up.
/// The `current_xp` parameter is your XP from **this level only, not the total amount of XP**.
fn get_xp_till_next_level(level: usize, current_xp: usize) -> usize {
    (5 * level.pow(2) + 50 * level + 100 - current_xp).max(0)
}

/// Gets you the total amount of XP needed to reach a certain level.
fn get_xp_for_level(level: usize) -> usize {
    let mut xp = 0;
    for i in 0..level {
        xp += get_xp_till_next_level(i, 0);
    }
    xp
}

/// Gets you the progress towards the next level.
/// The `total_xp` parameter is your **total XP, not the XP from this level only**.
pub fn get_level_progress(level: usize, total_xp: usize) -> f64 {
    let xp_progress = total_xp - get_xp_for_level(level);
    let xp_needed = get_xp_for_level(level + 1) - get_xp_for_level(level);

    xp_progress as f64 / xp_needed as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_xp_till_next_level() {
        assert_eq!(get_xp_till_next_level(0, 0), 100);
        assert_eq!(get_xp_till_next_level(1, 0), 155);
        assert_eq!(get_xp_till_next_level(2, 0), 220);
        assert_eq!(get_xp_till_next_level(10, 0), 1100);
        assert_eq!(get_xp_till_next_level(20, 0), 3100);
        assert_eq!(get_xp_till_next_level(80, 0), 36100);
    }

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
