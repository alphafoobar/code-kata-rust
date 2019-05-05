mod kata02 {
    use std::option::Option;

    fn _chop(key: i32, array: &[i32], offset: usize, length: usize) -> Option<usize> {
        let remaining: usize = length - offset;
        if remaining == 0 {
            return None;
        }

        let point: usize = offset + remaining / 2;
        if array[point] == key {
            return Some(point);
        }
        if array[point] > key {
            return _chop(key, array, offset, point);
        }
        return _chop(key, array, point + 1, length);
    }

    pub fn chop(key: i32, array: &[i32]) -> Option<usize> {
        return _chop(key, array, 0, array.len());
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn search_empty_array() {
            let array: [i32; 0] = [];
            assert_eq!(None, super::chop(1, &array));
        }

        #[test]
        fn search_singleton() {
            let array: [i32; 1] = [1];
            assert_eq!(Some(0), super::chop(1, &array));
            assert_eq!(None, super::chop(3, &array));
        }

        #[test]
        fn search_array_of_three() {
            let array: [i32; 3] = [1, 3, 5];
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
            let array: [i32; 4] = [1, 3, 5, 7];
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