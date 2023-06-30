pub fn get_xp_till_next_level(level: usize, current_xp: usize) -> usize {
    (5 * level.pow(2) + 50 * level + 100 - current_xp).max(0)
}

pub fn get_xp_for_level(level: usize) -> usize {
    if level == 0 {
        return 0;
    }

    let mut xp = 0;
    for i in 0..level {
        xp += get_xp_till_next_level(i, 0);
    }
    xp
}

pub fn get_level_progress(level: usize, current_xp: usize) -> f64 {
    let xp_progress = current_xp - get_xp_for_level(level);
    let xp_needed = get_xp_for_level(level + 1) - get_xp_for_level(level);

    xp_progress as f64 / xp_needed as f64
}
