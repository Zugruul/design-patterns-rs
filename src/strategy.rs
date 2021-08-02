// Source implementations for bubblesort: https://chercher.tech/rust/bubble-sort-rust
// TODO: Do mergesort, quicksort and others as well!

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

    struct OptimizedBubbleSorter;

    impl SortingStrategy for OptimizedBubbleSorter {
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

        OptimizedBubbleSorter::sort(&mut list);

        assert_eq!(list, vec![1, 2, 3, 7]);
    }

    fn sort_with_strategy<T: SortingStrategy, R: PartialOrd>(list: &mut Vec<R>) {
        T::sort(list);
    }

    #[test]
    fn using_first_bubblesort_strategy() {
        let mut list: Vec<i32> = vec![3, 7, 2, 1];

        sort_with_strategy::<BubbleSorter, i32>(&mut list);

        assert_eq!(list, vec![1, 2, 3, 7]);
    }

    #[test]
    fn using_optimized_bubblesort_strategy() {
        let mut list: Vec<i32> = vec![3, 7, 2, 1];

        sort_with_strategy::<OptimizedBubbleSorter, i32>(&mut list);

        assert_eq!(list, vec![1, 2, 3, 7]);
    }
}