struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        // using a vector, though with a heap its faster, way faster
        KthLargest { k: k, nums: nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums.sort_unstable_by(|a, b| b.cmp(a));
        self.nums[self.k as usize - 1 as usize]
    }
}

// /**
//  * Your KthLargest object will be instantiated and called as such:
//  * let obj = KthLargest::new(k, nums);
//  * let ret_1: i32 = obj.add(val);
//  */
pub fn main() {
    let k = 3;
    let nums = vec![4, 5, 8, 2];
    let obj = KthLargest::new(k, nums);
    let ret_1: i32 = obj.add(3);
    println!("{}", ret_1);
    let ret_2: i32 = obj.add(5);
    println!("{}", ret_2);
    let ret_3: i32 = obj.add(10);
    println!("{}", ret_3);
    let ret_4: i32 = obj.add(9);
    println!("{}", ret_4);
    let ret_5: i32 = obj.add(4);
    println!("{}", ret_5);
}
