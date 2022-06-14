use std::ptr;

trait InsertVec<T> {
    fn insert_vec(&mut self, vec_to_insert: Vec<T>, index_to_insert: usize);
}

impl<T> InsertVec<T> for Vec<T> {
    fn insert_vec(&mut self, vec_to_insert: Vec<T>, index_to_insert: usize) {
        let len = self.len();

        assert!(index_to_insert <= len);

        let len_to_insert = vec_to_insert.len();

        if len + len_to_insert >= self.capacity() {
            self.reserve(len_to_insert + len - self.capacity());
        }
        let len_to_insert_isize = len_to_insert as isize;
        assert!(len_to_insert_isize >= 0);

        unsafe {
            let p = self.as_mut_ptr().add(index_to_insert);

            ptr::copy(p, p.offset(len_to_insert_isize), len - index_to_insert);

            for (offset, element) in vec_to_insert.into_iter().enumerate() {
                ptr::write(p.add(offset), element);
            }

            self.set_len(len + len_to_insert)
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::InsertVec;

    #[test]
    fn test_insert_vec() {
        let mut new_vec = vec![1, 2, 3, 4];
        new_vec.insert_vec(vec![5, 6, 7, 8], 1);
        assert_eq!(new_vec, vec![1, 5, 6, 7, 8, 2, 3, 4]);
        let mut new_vec = vec![1, 2, 3, 4];
        new_vec.insert_vec(vec![], 1);
        assert_eq!(new_vec, vec![1, 2, 3, 4]);
    }
}
