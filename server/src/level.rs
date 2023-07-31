/// Gets you the total amount of XP needed to reach a certain level.
pub fn get_xp_for_level(level: usize) -> usize {
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

/// Gets you the progress towards the next role based on your current level and total XP earned.
/// Returns a tuple of role name, progress towards next role, and XP needed to reach next role.
pub fn get_next_role_progress(
    level: usize,
    total_xp: usize,
) -> (Option<String>, Option<f64>, Option<usize>) {
    if level >= 100 {
        return (None, None, None);
    }

    let (next_role_level, level_name) = if level >= 75 {
        (100, "「Legend (LVL 100)」".to_string())
    } else if level >= 50 {
        (75, "「Veteran (LVL 75)」".to_string())
    } else if level >= 25 {
        (50, "「Comrade (LVL 50)」".to_string())
    } else if level >= 10 {
        (25, "「Soldier (LVL 25)」".to_string())
    } else {
        (10, "「Cadet (LVL 10)」".to_string())
    };

    (
        Some(level_name),
        Some((total_xp as f64) / (get_xp_for_level(next_role_level) as f64)),
        Some(get_xp_for_level(next_role_level) - total_xp),
    )
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

    #[test]
    fn test_get_role_progress() {
        assert_eq!(
            get_next_role_progress(21, 28394),
            (
                Some("「Soldier (LVL 25)」".to_string()),
                Some(0.676047619047619),
                Some(13606)
            )
        );
        assert_eq!(get_next_role_progress(101, 0), (None, None, None));
        assert_eq!(
            get_next_role_progress(1, 0),
            (
                Some("「Cadet (LVL 10)」".to_string()),
                Some(0.0),
                Some(4675)
            )
        );
        assert_eq!(
            get_next_role_progress(9, 4675),
            (Some("「Cadet (LVL 10)」".to_string()), Some(1.0), Some(0))
        );
    }
}
