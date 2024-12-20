mod implementation {
    use std::cmp;

    fn on_plateau(ranks: &[i32], index: usize) -> bool {
        if ranks.len() == 1 {
            return true;
        }
        let last_index = ranks.len() - 1;
        if index == 0 {
            ranks[index] == ranks[index + 1]
        } else if index == last_index {
            ranks[index - 1] == ranks[index]
        } else {
            ranks[index - 1] == ranks[index] && ranks[index] == ranks[index + 1]
        }
    }

    fn is_peak(ranks: &[i32], index: usize) -> bool {
        if ranks.len() == 1 {
            return true;
        }
        let last_index = ranks.len();
        if index == 0 {
            ranks[0] < ranks[1]
        } else if index == last_index {
            ranks[last_index - 1] < ranks[last_index]
        } else {
            ranks[index - 1] < ranks[index] && ranks[index] > ranks[index + 1]
        }
    }

    struct SweetsNeeded {
        total: i32,
        largest: i32,
    }

    // Notice how, sine sections are by definition strictly increasing or strictly decreasing,
    // you don't even need the ranks here.
    fn calculate_sweets_needed_for_section(
        section_beginning: usize,
        section_end: usize,
    ) -> SweetsNeeded {
        let count = (section_end + 1 - section_beginning) as i32;
        SweetsNeeded {
            total: (1..=count).sum(),
            largest: count,
        }
    }

    pub fn count_sweets(ranks: &[i32]) -> i32 {
        let mut total_sweets = 0;
        let mut section_beginning = 0;
        let mut peak: Option<i32> = None;
        for index in 0..ranks.len() {
            if on_plateau(ranks, index) {
                total_sweets += 1;
                section_beginning += 1;
                continue;
            }
            if index == section_beginning {
                continue;
            }
            if index == ranks.len() - 1
                || ranks[section_beginning].cmp(&ranks[index])
                    != ranks[index].cmp(&ranks[index + 1])
            {
                let SweetsNeeded { total, largest } =
                    calculate_sweets_needed_for_section(section_beginning, index);
                total_sweets += total;
                assert!(ranks[section_beginning] != ranks[index]);
                if ranks[section_beginning] < ranks[index] {
                    if is_peak(ranks, index) {
                        assert!(peak.is_none());
                        peak = Some(largest);
                        section_beginning = index;
                    } else {
                        section_beginning = index + 1;
                    }
                } else {
                    assert!(!is_peak(ranks, index));
                    if let Some(last_peak_added) = peak {
                        total_sweets -= cmp::min(last_peak_added, largest);
                        peak = None;
                    }
                    section_beginning = index + 1;
                }
            }
        }
        total_sweets
    }

    #[cfg(test)]
    mod tests {
        use crate::implementation::*;

        fn test_forwards_and_backwards(mut ranks: Vec<i32>, expected_candy: i32) {
            assert_eq!(count_sweets(&ranks), expected_candy);
            ranks.reverse();
            assert_eq!(count_sweets(&ranks), expected_candy);
        }

        #[test]
        fn test_0() {
            test_forwards_and_backwards(vec![1], 1);
        }

        #[test]
        fn test_1() {
            test_forwards_and_backwards(vec![1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 3, 2, 1], 32);
        }
    }
}

fn main() {
    assert_eq!(
        implementation::count_sweets(&[1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 3, 2, 1]),
        32
    );
}