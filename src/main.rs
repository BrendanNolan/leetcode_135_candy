// There is a wring assumption throughout - that a section must have length > 1 .
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
        let last_index = ranks.len() - 1;
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

    fn calculate_sweets_needed_for_section(
        ranks: &[i32],
        section_beginning: usize,
        section_end: usize,
    ) -> SweetsNeeded {
        let count = (section_end + 1 - section_beginning) as i32;
        let beginning_rank = ranks[section_beginning];
        let ending_rank = ranks[section_end];
        if beginning_rank > ending_rank
            || section_beginning == 0
            || beginning_rank == ranks[section_beginning - 1]
        {
            SweetsNeeded {
                total: (1..=count).sum(),
                largest: count,
            }
        } else {
            assert!(ending_rank > beginning_rank || section_beginning == section_end);
            // Need to add a bump of one to every sweet award because the prrevious section
            // ended at a lower rank.
            SweetsNeeded {
                total: (1..=count).map(|x| x + 1).sum(),
                largest: count + 1,
            }
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
            if index == section_beginning && index != ranks.len() - 1 {
                if ranks[section_beginning] == ranks[section_beginning + 1] {
                    if section_beginning == 0
                        || ranks[section_beginning - 1] == ranks[section_beginning]
                    {
                        total_sweets += 1;
                    } else {
                        total_sweets += 2;
                    }
                    section_beginning += 1;
                }
                continue;
            }
            if index == ranks.len() - 1
                || ranks[section_beginning].cmp(&ranks[index])
                    != ranks[index].cmp(&ranks[index + 1])
            {
                let SweetsNeeded { total, largest } =
                    calculate_sweets_needed_for_section(ranks, section_beginning, index);
                total_sweets += total;
                if ranks[section_beginning] < ranks[index] {
                    if is_peak(ranks, index) {
                        assert!(peak.is_none());
                        peak = Some(largest);
                        section_beginning = index;
                    } else {
                        section_beginning = index + 1;
                    }
                } else {
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

        #[test]
        fn test_2() {
            test_forwards_and_backwards(vec![1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 3, 2, 1, 2, 5, 7], 41);
        }

        #[test]
        fn test_3() {
            test_forwards_and_backwards(vec![1, 2, 3, 3, 1], 9);
        }

        #[test]
        fn test_4() {
            test_forwards_and_backwards(vec![1, 2, 3, 3, 3, 1], 10);
        }

        #[test]
        fn test_5() {
            test_forwards_and_backwards(vec![3, 2, 1, 1, 3, 1], 10);
        }

        #[test]
        fn test_6() {
            test_forwards_and_backwards(vec![3, 2, 1, 1, 1, 3, 1], 11);
        }

        #[test]
        fn test_7() {
            test_forwards_and_backwards(vec![1, 0, 3], 5);
        }

        #[test]
        fn test_8() {
            test_forwards_and_backwards(vec![1, 2, 1, 2, 1], 7);
        }

        #[test]
        fn test_9() {
            test_forwards_and_backwards(vec![1, 2, 1, 2, 1, 2], 9);
        }

        #[test]
        fn test_10() {
            test_forwards_and_backwards(vec![2, 1, 2, 1, 2, 1, 2], 11);
        }

        #[test]
        fn test_11() {
            test_forwards_and_backwards(
                vec![
                    58, 21, 72, 77, 48, 9, 38, 71, 68, 77, 82, 47, 25, 94, 89, 54, 26, 54, 54, 99,
                    64, 71, 76, 63, 81, 82, 60, 64, 29, 51, 87, 87, 72, 12, 16, 20, 21, 54, 43, 41,
                    83, 77, 41, 61, 72, 82, 15, 50, 36, 69, 49, 53, 92, 77, 16, 73, 12, 28, 37, 41,
                    79, 25, 80, 3, 37, 48, 23, 10, 55, 19, 51, 38, 96, 92, 99, 68, 75, 14, 18, 63,
                    35, 19, 68, 28, 49, 36, 53, 61, 64, 91, 2, 43, 68, 34, 46, 57, 82, 22, 67, 89,
                ],
                208,
            );
        }

        #[test]
        fn test_12() {
            assert_eq!(
                count_sweets(&[1, 2, 2, 3]),
                6,
            );
        }

        #[test]
        fn test_13() {
            assert_eq!(
                count_sweets(&[1, 2, 3, 3, 2, 1]),
                12,
            );
        }
    }
}

fn main() {
    assert_eq!(
        implementation::count_sweets(&[1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 3, 2, 1]),
        32
    );
}
