use crate::types::Matches;

/// Calculates the rating that is displayed to the user.
pub fn get_display_rating(rating: f64, deviation: f64) -> f64 {
    ((rating - (3.0 * deviation)) * 100.0 + 1000.0).max(0.0)
}

/// Gets the longest win streak, longest lose streak, current win streak, and current lose streak from a String of match results.
pub fn get_streaks(matches: String) -> (usize, usize, usize, usize) {
    let mut longest_win_streak = 0;
    let mut longest_lose_streak = 0;
    let mut current_win_streak = 0;
    let mut current_lose_streak = 0;

    for m in matches.chars() {
        if m == 'W' {
            current_win_streak += 1;
            current_lose_streak = 0;
        } else {
            current_lose_streak += 1;
            current_win_streak = 0;
        }

        if current_win_streak > longest_win_streak {
            longest_win_streak = current_win_streak;
        }

        if current_lose_streak > longest_lose_streak {
            longest_lose_streak = current_lose_streak;
        }
    }

    (
        longest_win_streak,
        longest_lose_streak,
        current_win_streak,
        current_lose_streak,
    )
}

/// Gets the last X matches from a String of match results, in reversed order.
pub fn get_recent_matches(matches: String, amount: usize) -> String {
    matches.chars().rev().take(amount).collect()
}

/// Gets the recent performance of a user in the last 5 matches.
pub fn get_recent_performance(
    matches: &[Matches],
    user_id: &String,
    rating: f64,
    deviation: f64,
) -> f64 {
    let m = match matches.last() {
        Some(m) => m,
        None => return 0.0,
    };

    let (old_mu, old_sigma) = if &m.winner_id == user_id {
        (m.old_winner_rating, m.old_winner_deviation)
    } else {
        (m.old_loser_rating, m.old_loser_deviation)
    };

    get_display_rating(rating, deviation) - get_display_rating(old_mu, old_sigma)
}

pub fn get_last_ratings(matches: &[Matches], user_id: &String) -> Vec<f64> {
    let mut ratings = Vec::new();

    for m in matches {
        if &m.winner_id == user_id {
            ratings.push(get_display_rating(
                m.new_winner_rating,
                m.new_winner_deviation,
            ));
        } else {
            ratings.push(get_display_rating(
                m.new_loser_rating,
                m.new_loser_deviation,
            ));
        }
    }

    ratings
}

/// Calculates the average rating of a vector of ratings.
pub fn get_average_opponent(ratings: &Vec<f64>) -> f64 {
    if ratings.is_empty() {
        return 0.0;
    }

    let mut sum = 0.0;

    for r in ratings {
        sum += r;
    }

    sum / ratings.len() as f64
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

    #[test]
    fn test_streaks() {
        let str = "WWWWLWLLLLWWWLLLWWLLLLW".to_string();

        let (longest_win_streak, longest_lose_streak, current_win_streak, current_lose_streak) =
            get_streaks(str);

        assert!(longest_win_streak == 4);
        assert!(longest_lose_streak == 4);
        assert!(current_win_streak == 1);
        assert!(current_lose_streak == 0);

        let str = "WWWWWWWWWWWWWWWWWWWWWWWWWWWW".to_string();

        let (longest_win_streak, longest_lose_streak, current_win_streak, current_lose_streak) =
            get_streaks(str.clone());

        assert!(longest_win_streak == str.len());
        assert!(longest_lose_streak == 0);
        assert!(current_win_streak == str.len());
        assert!(current_lose_streak == 0);
    }

    #[test]
    fn test_recent_matches() {
        let str = "WWWWLWLLLLWWWLLLWWLLLLW".to_string();

        assert!(get_recent_matches(str, 10) == "WLLLL");

        let str = "LWW".to_string();

        assert!(get_recent_matches(str, 10) == "WWL");

        let str = "L".to_string();

        assert!(get_recent_matches(str, 10) == "L");

        let str = "".to_string();

        assert!(get_recent_matches(str, 10).is_empty());
    }

    #[test]
    fn test_recent_performance() {
        let matches = vec![
            Matches {
                match_id: "0".to_string(),
                winner_id: "0".to_string(),
                loser_id: "1".to_string(),
                old_winner_rating: 25.0,
                old_winner_deviation: 4.0,
                old_loser_rating: 25.0,
                old_loser_deviation: 4.0,
                new_winner_rating: 30.0,
                new_winner_deviation: 3.0,
                new_loser_rating: 20.0,
                new_loser_deviation: 3.0,
                timestamp: 0,
                old_loser_display_rating: 0.0,
                old_winner_display_rating: 0.0,
                new_loser_display_rating: 0.0,
                new_winner_display_rating: 0.0,
                winner_display_rating_change: 0.0,
                loser_display_rating_change: 0.0,
            },
            Matches {
                match_id: "0".to_string(),
                winner_id: "0".to_string(),
                loser_id: "1".to_string(),
                old_winner_rating: 25.0,
                old_winner_deviation: 4.0,
                old_loser_rating: 25.0,
                old_loser_deviation: 4.0,
                new_winner_rating: 30.0,
                new_winner_deviation: 3.0,
                new_loser_rating: 20.0,
                new_loser_deviation: 3.0,
                timestamp: 0,
                old_loser_display_rating: 0.0,
                old_winner_display_rating: 0.0,
                new_loser_display_rating: 0.0,
                new_winner_display_rating: 0.0,
                winner_display_rating_change: 0.0,
                loser_display_rating_change: 0.0,
            },
        ];

        let current_rating = get_display_rating(30.0, 3.0);
        let old_rating = get_display_rating(25.0, 4.0);

        assert!(
            get_recent_performance(&matches, &"0".to_string(), 30.0, 3.0)
                == current_rating - old_rating
        );
    }

    #[test]
    fn test_average_opponent() {
        let ratings = vec![25.0, 25.0, 25.0, 25.0, 25.0];

        assert!(get_average_opponent(&ratings) == 25.0);

        let ratings = vec![5.0, 15.0, 25.0, 35.0, 45.0, 55.0];

        assert!((get_average_opponent(&ratings) - 30.0).abs() < f64::EPSILON);
    }
}
