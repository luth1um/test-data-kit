use getrandom::fill;

// TODO: write test
pub const ALL_DIGITS: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
// TODO: write test
pub const ALL_LETTERS: &[&str] = &[
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];
pub const ALL_DIGITS_ALL_LETTERS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I",
    "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];

pub fn push_random_chars(
    string_to_expand: &mut String,
    allowed_chars: &[&str],
    len: usize,
) -> Result<(), String> {
    // TODO: write tests
    for _ in 0..len {
        string_to_expand.push_str(random_element(allowed_chars)?);
    }
    Ok(())
}

/// Returns a random element from the input vector. Returns `Err` if the vector has length 0.
fn random_element<T>(elements: &[T]) -> Result<&T, String> {
    // TODO: write tests
    if elements.is_empty() {
        return Err("Cannot choose an element from an empty vector".to_owned());
    }

    let pos = random_in_range(0, elements.len())?;

    elements
        .get(pos)
        .ok_or_else(|| "random position is out of bounds".to_owned())
}

/// Generates a random number between `lower` (including) and `upper` (excluding).
fn random_in_range(lower: usize, upper: usize) -> Result<usize, String> {
    assert!(lower < upper, "range must be non-empty");

    let mut bytes = [0u8; size_of::<usize>()];
    fill(&mut bytes).map_err(|err| err.to_string())?;

    let n = usize::from_ne_bytes(bytes);
    Ok(lower + n % (upper - lower))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const LOOP_COUNT_RND_TEST: u16 = 1000;

    #[test]
    fn random_in_range_returns_at_least_lower_bound_when_called() {
        // given
        let lower: usize = 12;
        let upper: usize = 42;

        for _ in 0..LOOP_COUNT_RND_TEST {
            // when
            let result = random_in_range(lower, upper);

            // then
            assert!(result.is_ok());
            assert!(result.unwrap() >= lower);
        }
    }

    #[test]
    fn random_in_range_returns_at_most_upper_bound_minus_1_when_called() {
        // given
        let lower: usize = 12;
        let upper: usize = 42;

        for _ in 0..LOOP_COUNT_RND_TEST {
            // when
            let result = random_in_range(lower, upper);

            // then
            assert!(result.is_ok());
            assert!(result.unwrap() < upper);
        }
    }

    #[test]
    fn random_in_range_returns_different_numbers_when_called_multiple_times() {
        // given
        let lower: usize = 0;
        let upper: usize = 1_000_000_000;
        let loop_count = LOOP_COUNT_RND_TEST;
        let expected_min_different_results = loop_count as f32 * 0.95;
        let mut results: HashSet<usize> = HashSet::with_capacity(loop_count as usize);

        // when
        for _ in 0..loop_count {
            let result = random_in_range(lower, upper);
            results.insert(result.unwrap());
        }

        // then
        assert!(results.len() as f32 > expected_min_different_results);
    }

    #[test]
    #[should_panic]
    fn random_in_range_panics_when_range_is_empty_due_to_equal_input() {
        // given
        let input = 42;

        // when / then
        _ = random_in_range(input, input);
    }

    #[test]
    #[should_panic]
    fn random_in_range_panics_when_range_is_empty_due_to_upper_bound_being_smaller() {
        // given
        let input = 42;

        // when / then
        _ = random_in_range(input, input - 1);
    }
}
