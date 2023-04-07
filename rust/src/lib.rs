use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn shuffle<T: Clone>(list: &[T], seed: u64) -> Vec<T> {
    let mut shuffled_list = Vec::from(list);
    
    if shuffled_list.len() <= 2 {
        return shuffled_list;
    }
    
    let mut rng = SmallRng::seed_from_u64(seed);
    
    for i in 0..(shuffled_list.len() - 2) {
        let j = (rng.gen_range(i..shuffled_list.len()) + i) % shuffled_list.len();
        
        shuffled_list.swap(i, j);
    }
    
    shuffled_list
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn should_not_modify_parameter_array_and_return_a_copy_of_it_shuffled() {
        let list = vec![1, 2, 3, 4, 5];
        let shuffled = shuffle(&list, 100);
        assert_ne!(list, shuffled);
        assert_eq!(list.len(), shuffled.len());
    }

    // test all original items are in the result shuffled list
    #[test]
    fn should_return_a_shuffled_list_with_all_original_items() {
        let list = vec![1, 2, 3, 4, 5];
        let shuffled = shuffle(&list, 100);

        assert_eq!(list.len(), shuffled.len());
        
        for item in list {
            assert!(shuffled.contains(&item));
        }
    }

    #[test]
    fn should_shuffle_large_arrays() {
        let mut list = Vec::new();
        
        for i in 0..50000000 {
            list.push(i);
        }

        let start_time = Instant::now();

        shuffle(&list, 100);
        
        let timeresult = start_time.elapsed();
        assert!(timeresult < Duration::from_secs(10), "Shuffling took too long: {:?}", timeresult);
    }
}
