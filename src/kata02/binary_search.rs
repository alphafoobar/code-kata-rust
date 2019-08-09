mod kata02 {
    use std::option::Option;

    pub fn chop(key: i64, array: &[i64]) -> Option<usize> {
        let mut low: usize = 0;
        let mut high: usize = array.len();

        while low < high {
            let remaining: usize = high - low;
            let point: usize = low + remaining / 2;
            let value: i64 = array[point];

            if value == key {
                return Some(point);
            }

            if value > key {
                high = point;
            } else {
                low = point + 1;
            }
        }
        return None;
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn search_empty_array() {
            let array: [i64; 0] = [];
            assert_eq!(None, super::chop(1, &array));
        }

        #[test]
        fn search_singleton() {
            let array: [i64; 1] = [1];
            assert_eq!(Some(0), super::chop(1, &array));
            assert_eq!(None, super::chop(3, &array));
        }

        #[test]
        fn search_array_of_three() {
            let array: [i64; 3] = [1, 3, 5];
            assert_eq!(Some(0), super::chop(1, &array));
            assert_eq!(Some(1), super::chop(3, &array));
            assert_eq!(Some(2), super::chop(5, &array));
            assert_eq!(None, super::chop(0, &array));
            assert_eq!(None, super::chop(2, &array));
            assert_eq!(None, super::chop(4, &array));
            assert_eq!(None, super::chop(6, &array));
        }

        #[test]
        fn search_array_or_four() {
            let array: [i64; 4] = [1, 3, 5, 7];
            assert_eq!(Some(0), super::chop(1, &array));
            assert_eq!(Some(1), super::chop(3, &array));
            assert_eq!(Some(2), super::chop(5, &array));
            assert_eq!(Some(3), super::chop(7, &array));
            assert_eq!(None, super::chop(0, &array));
            assert_eq!(None, super::chop(2, &array));
            assert_eq!(None, super::chop(4, &array));
            assert_eq!(None, super::chop(6, &array));
            assert_eq!(None, super::chop(8, &array));
        }
    }
}
