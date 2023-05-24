use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

/// Shuffles a slice of generic type `T` using the Fisher-Yates shuffle algorithm.
///
/// # Arguments
///
/// * `list` - A slice of generic type `T` to be shuffled.
///
/// # Returns
///
/// A new `Vec<T>` that is a shuffled version of the input slice.
///
/// # Examples
///
/// ```
/// let list = vec![1, 2, 3, 4, 5];
///
/// let shuffled_list = shufflr::shuffle(&list);
///
/// assert_ne!(list, shuffled_list);
/// ```
pub fn shuffle<T: Clone>(list: &[T]) -> Vec<T> {
    let mut shuffled_list = Vec::from(list);
    
    if shuffled_list.len() <= 2 {
        return shuffled_list;
    }
    
    let mut rng = SmallRng::from_entropy();
    
    for i in (1..shuffled_list.len()).rev() {
        let j = rng.gen_range(0..=i);
        
        shuffled_list.swap(i, j);
    }
    
    return shuffled_list
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn should_not_modify_parameter_array_and_return_a_copy_of_it_shuffled() {
        let list = vec![1, 2, 3, 4, 5];
        let shuffled = shuffle(&list);
        assert_ne!(list, shuffled);
        assert_eq!(list.len(), shuffled.len());
    }

    // test all original items are in the result shuffled list
    #[test]
    fn should_return_a_shuffled_list_with_all_original_items() {
        let list = vec![1, 2, 3, 4, 5];
        let shuffled = shuffle(&list);

        assert_eq!(list.len(), shuffled.len());
        
        for item in list {
            assert!(shuffled.contains(&item));
        }
    }

    // test shuffling two times return different results
    #[test]
    fn should_return_different_results_when_shuffling_two_times() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let shuffled1 = shuffle(&list);
        let shuffled2 = shuffle(&list);

        assert_ne!(shuffled1, shuffled2);
    }

    // test shiffling a big list is performant
    #[test]
    fn should_shuffle_array_with_50m_elements_in_under_25s() {
        let mut list = Vec::new();
        
        for i in 0..50000000 {
            list.push(i);
        }

        let start_time = Instant::now();

        shuffle(&list);
        
        let timeresult = start_time.elapsed();
        assert!(timeresult < Duration::from_secs(12), "Shuffling took too long: {:?}", timeresult);
    }
}
