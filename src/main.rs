struct Solution{

}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        result.push(Vec::from_iter([1,2,3]));
        result
    }
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::combine(10, 5));
}
