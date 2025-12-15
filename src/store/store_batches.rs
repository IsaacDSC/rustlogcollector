use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Store {
    max_size: usize,
    data: Arc<Mutex<Vec<Vec<Vec<u8>>>>>,
}

impl Store {
    pub fn new(max_size: usize) -> Self {
        if max_size == 0 {
            panic!("max_size must be greater than 0");
        }

        Store {
            max_size,
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, se_data: Vec<u8>) {
        let mut data = self.data.lock().unwrap();

        if data.is_empty() {
            data.push(vec![se_data]);
            return;
        }

        // Procura o primeiro batch que não está cheio
        for batch in data.iter_mut() {
            if batch.len() < self.max_size {
                batch.push(se_data);
                return;
            }
        }

        // Se todos os batches estão cheios, cria um novo
        data.push(vec![se_data]);
    }

    // get first element and remove it from the store
    pub fn retrieve_first(&self) -> Result<Vec<Vec<u8>>, Error> {
        let mut data = self.data.lock().unwrap();
        if data.is_empty() {
            return Err(Error::new(ErrorKind::NotFound, "No data found"));
        }

        // Remove the first element from memory before releasing the lock
        let result = data.remove(0);

        Ok(result)
    }

    pub fn flush_all(&self) -> Result<(), Error> {
        let mut data = self.data.lock().unwrap();

        if data.is_empty() {
            return Err(Error::new(ErrorKind::NotFound, "No data found"));
        }

        data.push(Vec::new());
        Ok(())
    }

    pub fn clone_handle(&self) -> Self {
        Store {
            max_size: self.max_size,
            data: Arc::clone(&self.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_21_items() {
        let store = Store::new(20);
        for i in 0..21 {
            store.add(vec![i as u8]);
        }

        let data = store.data.lock().unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].len(), 20);
        assert_eq!(data[1].len(), 1);
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let store = Store::new(10);
        let mut handles = vec![];

        for i in 0..5 {
            let store_clone = store.clone_handle();
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    store_clone.add(vec![i as u8, j as u8]);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let data = store.data.lock().unwrap();
        let total_items: usize = data.iter().map(|batch| batch.len()).sum();
        assert_eq!(total_items, 50);
    }

    #[test]
    fn test_retrieve_first_removes_from_memory() {
        let store = Store::new(3);

        // Add 7 items: [[0,1,2], [3,4,5], [6]]
        for i in 0..7 {
            store.add(vec![i as u8]);
        }

        // Verify initial state: 3 batches total
        {
            let data = store.data.lock().unwrap();
            assert_eq!(data.len(), 3, "Should have 3 batches initially");
            assert_eq!(data[0].len(), 3, "Batch 0 should have 3 items");
            assert_eq!(data[0][0][0], 0, "Batch 0, item 0 should be 0");
            assert_eq!(data[0][1][0], 1, "Batch 0, item 1 should be 1");
            assert_eq!(data[0][2][0], 2, "Batch 0, item 2 should be 2");
            assert_eq!(data[1].len(), 3, "Batch 1 should have 3 items");
            assert_eq!(data[1][0][0], 3, "Batch 1, item 0 should be 3");
            assert_eq!(data[1][1][0], 4, "Batch 1, item 1 should be 4");
            assert_eq!(data[1][2][0], 5, "Batch 1, item 2 should be 5");
            assert_eq!(data[2].len(), 1, "Batch 2 should have 1 item");
            assert_eq!(data[2][0][0], 6, "Batch 2, item 0 should be 6");
        }

        // Retrieve first batch
        let result = store.retrieve_first().unwrap();

        // Validate the returned batch
        assert_eq!(result.len(), 3, "Result should have 3 items");
        assert_eq!(result[0][0], 0, "First item should be 0");
        assert_eq!(result[1][0], 1, "Second item should be 1");
        assert_eq!(result[2][0], 2, "Third item should be 2");

        // Validate that only 2 batches remain in memory
        {
            let data = store.data.lock().unwrap();
            assert_eq!(data.len(), 2, "Should have 2 batches after first retrieve");
            assert_eq!(data[0].len(), 3, "New batch 0 should have 3 items");
            assert_eq!(data[0][0][0], 3, "New batch 0 should contain value 3");
            assert_eq!(data[0][1][0], 4, "Batch 0 should contain value 4");
            assert_eq!(data[0][2][0], 5, "Batch 0 should contain value 5");
            assert_eq!(data[1].len(), 1, "Batch 1 should have 1 item");
            assert_eq!(data[1][0][0], 6, "Batch 1 should contain value 6");
        }

        // Retrieve second batch
        let result2 = store.retrieve_first().unwrap();

        // Validate the returned batch
        assert_eq!(result2.len(), 3, "Result should have 3 items");
        assert_eq!(result2[0][0], 3, "Item 0 should be 3");
        assert_eq!(result2[1][0], 4, "Item 1 should be 4");
        assert_eq!(result2[2][0], 5, "Item 2 should be 5");

        // Validate that only 1 batch remains
        {
            let data = store.data.lock().unwrap();
            assert_eq!(data.len(), 1, "Should have 1 batch after second retrieve");
            assert_eq!(data[0].len(), 1, "Batch 0 should have 1 item");
            assert_eq!(data[0][0][0], 6, "Batch 0 should contain value 6");
        }

        // Retrieve last batch
        let result3 = store.retrieve_first().unwrap();
        assert_eq!(result3.len(), 1, "Third result should have 1 item");
        assert_eq!(result3[0][0], 6, "Third result should be 6");

        // Verify store is completely empty
        {
            let data = store.data.lock().unwrap();
            assert_eq!(data.len(), 0, "Store should be empty after all retrieves");
        }

        // Try to retrieve from empty store
        let result_empty = store.retrieve_first();
        assert!(
            result_empty.is_err(),
            "Should return error when store is empty"
        );
    }

    #[test]
    fn test_add_fills_batches_sequentially() {
        let store = Store::new(3);

        // Add 10 items
        for i in 0..10 {
            store.add(vec![i as u8]);
        }

        // Should create: [[0,1,2], [3,4,5], [6,7,8], [9]]
        let data = store.data.lock().unwrap();

        assert_eq!(data.len(), 4, "Should have 4 batches");

        // Batch 0: [0, 1, 2]
        assert_eq!(data[0].len(), 3, "Batch 0 should be full with 3 items");
        assert_eq!(data[0][0][0], 0, "Batch 0, item 0 should be 0");
        assert_eq!(data[0][1][0], 1, "Batch 0, item 1 should be 1");
        assert_eq!(data[0][2][0], 2, "Batch 0, item 2 should be 2");

        // Batch 1: [3, 4, 5]
        assert_eq!(data[1].len(), 3, "Batch 1 should be full with 3 items");
        assert_eq!(data[1][0][0], 3, "Batch 1, item 0 should be 3");
        assert_eq!(data[1][1][0], 4, "Batch 1, item 1 should be 4");
        assert_eq!(data[1][2][0], 5, "Batch 1, item 2 should be 5");

        // Batch 2: [6, 7, 8]
        assert_eq!(data[2].len(), 3, "Batch 2 should be full with 3 items");
        assert_eq!(data[2][0][0], 6, "Batch 2, item 0 should be 6");
        assert_eq!(data[2][1][0], 7, "Batch 2, item 1 should be 7");
        assert_eq!(data[2][2][0], 8, "Batch 2, item 2 should be 8");

        // Batch 3: [9]
        assert_eq!(data[3].len(), 1, "Batch 3 should have 1 item");
        assert_eq!(data[3][0][0], 9, "Batch 3, item 0 should be 9");
    }
}
