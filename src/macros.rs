//! Macro definitions.

macro_rules! index_set_impl_from_iterator {
    ($($Set:tt)*) => {
        impl<S: crate::storage::Storage> FromIterator<usize>
            for $($Set)*<S>
        {
            #[inline]
            fn from_iter<T>(iter: T) -> Self
            where
                T: IntoIterator<Item = usize>
            {
                use crate::IndexSet;

                let mut set = Self::new();

                for item in iter {
                    set.insert(item);
                }

                set
            }
        }
    };
}

macro_rules! index_set_impl_extend {
    ($($Set:tt)*) => {
        impl<S: crate::storage::Storage> Extend<usize> for $($Set)*<S> {
            #[inline]
            fn extend<T>(&mut self, iter: T)
            where
                T: IntoIterator<Item = usize>
            {
                use crate::IndexSet;

                for item in iter {
                    self.insert(item);
                }
            }
        }
    };
}

macro_rules! index_set_impl_from {
    ($($Set:tt)*) => {
        impl<S: crate::storage::Storage> From<$($Set)*<S>>
            for alloc::collections::BTreeSet<usize>
        {
            #[inline]
            fn from(index_set: $($Set)*<S>) -> Self {
                Self::from(&index_set)
            }
        }

        impl<S: crate::storage::Storage> From<&$($Set)*<S>>
            for alloc::collections::BTreeSet<usize>
        {
            fn from(index_set: &$($Set)*<S>) -> Self {
                use crate::IndexSet;

                let mut btree_set = Self::new();

                for index in index_set.iter() {
                    btree_set.insert(index);
                }

                btree_set
            }
        }

        impl<S: crate::storage::Storage> From<$($Set)*<S>>
            for alloc::vec::Vec<usize>
        {
            #[inline]
            fn from(index_set: $($Set)*<S>) -> Self {
                Self::from(&index_set)
            }
        }

        impl<S: crate::storage::Storage> From<&$($Set)*<S>>
            for alloc::vec::Vec<usize>
        {
            fn from(index_set: &$($Set)*<S>) -> Self {
                use crate::IndexSet;

                let mut vec = Self::new();

                for index in index_set.iter() {
                    vec.push(index);
                }

                vec
            }
        }
    };
}

macro_rules! index_set_tests_for {
    ($Set:ty) => {
        #[cfg(test)]
        mod tests {
            use crate::IndexSet;

            /// Test index insert ops.
            #[test]
            fn test_index_set_insert() {
                let mut set = <$Set>::new();
                let mut indices = vec![1, 4, 6, 3, 1, 100, 123, 12, 3];

                // insert some elements into the set
                for i in indices.iter().copied() {
                    set.insert(i);
                }

                // check if the set contains the same elements
                // we inserted, in ascending order
                indices.sort_unstable();
                indices.dedup();

                let set_indices: Vec<_> = set.iter().collect();
                assert_eq!(indices, set_indices);

                // check that the no. of storage elements used is lower
                // than the max no. of bitsets we would otherwise need
                let storage_elements_max =
                    indices[indices.len() - 1] / <u64 as $crate::storage::Storage>::WIDTH;
                assert!(set.bit_sets.len() <= storage_elements_max);
            }

            /// Test index remove ops.
            #[test]
            fn test_index_set_remove() {
                let mut set = <$Set>::new();
                let indices = [1, 4, 6, 3, 1, 100, 123, 12, 3];
                let remove = [100, 6, 100, 12, 123, 3];

                // insert some elements into the set
                for i in indices.iter().copied() {
                    set.insert(i);
                }

                // remove elements from the set
                for i in remove.iter().copied() {
                    set.remove(i);
                }

                let expected: ::std::collections::HashSet<_> = {
                    let indices: ::std::collections::HashSet<_> = indices.into_iter().collect();
                    let remove: ::std::collections::HashSet<_> = remove.into_iter().collect();
                    indices.difference(&remove).copied().collect()
                };
                let got: ::std::collections::HashSet<_> = set.iter().collect();

                assert_eq!(expected, got);
            }

            /// Test creating an index from an iterator.
            #[test]
            fn test_index_set_from_iter() {
                let indices = [1, 4, 6, 3, 1, 100, 123, 12, 3];

                let got: $Set = indices.iter().copied().collect();
                let expected: ::std::collections::BTreeSet<_> = indices.iter().copied().collect();

                assert_eq!(expected, got.into());
            }

            /// Test index set length related ops.
            #[test]
            fn test_index_set_len_and_is_empty() {
                let indices_1 = [1, 4, 6, 3];
                let indices_2 = [2, 100, 123, 12, 5];

                let mut set = <$Set>::new();

                assert!(set.is_empty());

                set.extend(indices_1.iter().copied());
                assert!(!set.is_empty());
                assert_eq!(set.len(), indices_1.len());

                set.extend(indices_2.iter().copied());
                assert!(!set.is_empty());
                assert_eq!(set.len(), indices_1.len() + indices_2.len());

                for item in indices_1.iter().copied() {
                    set.remove(item);
                }
                assert!(!set.is_empty());
                assert_eq!(set.len(), indices_2.len());

                for item in indices_2.iter().copied() {
                    set.remove(item);
                }
                assert!(set.is_empty());
                assert_eq!(set.len(), 0);
            }
        }
    };
}

pub(crate) use index_set_impl_extend;
pub(crate) use index_set_impl_from;
pub(crate) use index_set_impl_from_iterator;
pub(crate) use index_set_tests_for;
