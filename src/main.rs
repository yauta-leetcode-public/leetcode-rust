struct Solution{

}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        result.push(Vec::from_iter([1,2,3, 4]));
        result
    }
}

fn data_types() {
    let guess:f64 = "43".parse().expect("not a mumber!");
    println!("guess is: {guess}");
    let heart = '😻';
    println!("heart is: {heart}");
    let tup: (i32, i64, f64) = (1,2,3.0);
    println!("tup is: {:?}", tup);
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::combine(10, 5));

    let x = 5;
    println!("the value of x is: {x}");
    let x = 6;
    println!("the value of x is: {x}");
    let x = 7;
    println!("the value of x is: {x}");
    {
        let x = x*2;
        println!("inside the value of x is: {x}");
    }
    println!("outsize the value of x is: {x}");

    data_types();
}
