use std::sync::RwLock;

pub fn process_data(data: &RwLock<Vec<u8>>) -> Vec<u8> {
    let result;
    {
        let mut write = data.write().unwrap();
        write.push(42);
    }
    {
        let read = data.read().unwrap();
        result = read.clone();
    }
    result
}
