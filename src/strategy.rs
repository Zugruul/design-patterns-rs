#[cfg(test)]
mod tests {
    trait SortingStrategy {
        fn sort<T: PartialOrd>(list: &mut Vec<T>);
    }

    struct BubbleSorter;

    impl SortingStrategy for BubbleSorter {
        fn sort<T: PartialOrd>(list: &mut Vec<T>) {
            let mut swapped = true;
            while swapped {
                // No swap means array is sorted.
                swapped = false;
                for i in 1..list.len() {
                    if list[i - 1] > list[i] {
                        list.swap(i - 1, i);
                        swapped = true
                    }
                }
            }
        }
    }

    struct AnotherBubbleSorter;

    impl SortingStrategy for AnotherBubbleSorter {
        fn sort<T: PartialOrd>(list: &mut Vec<T>) {
            let mut new_len: usize;
            let mut len = list.len();
            loop {
                new_len = 0;
                for i in 1..len {
                    if list[i - 1] > list[i] {
                        list.swap(i - 1, i);
                        new_len = i;
                    }
                }
                if new_len == 0 {
                    break;
                }
                len = new_len;
            }
        }
    }

    #[test]
    fn bubblesort_works() {
        let mut list = vec![3, 7, 2, 1];

        BubbleSorter::sort(&mut list);

        assert_eq!(list, vec![1, 2, 3, 7]);
    }

    #[test]
    fn another_bubblesort_works() {
        let mut list = vec![3, 7, 2, 1];

        AnotherBubbleSorter::sort(&mut list);

        assert_eq!(list, vec![1, 2, 3, 7]);
    }
}