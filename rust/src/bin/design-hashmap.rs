struct MyHashMap {
    key: Vec<i32>,
    value: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        MyHashMap {
            key: Vec::new(),
            value: Vec::new(),
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.key.contains(&key) {
            let index = self.key.iter().position(|&x| x == key).unwrap();
            self.value[index] = value;
        } else {
            self.key.push(key);
            self.value.push(value);
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if self.key.contains(&key) {
            let index = self.key.iter().position(|&x| x == key).unwrap();
            return self.value[index];
        } else {
            return -1;
        }
    }
    
    fn remove(&mut self, key: i32) {
        if self.key.contains(&key) {
            let index = self.key.iter().position(|&x| x == key).unwrap();
            self.key.remove(index);
            self.value.remove(index);
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */