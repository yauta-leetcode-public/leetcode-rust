use core::fmt;
use std::{io, mem};

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
    let (a, b, _) = tup;
    println!("a is {a}, b is {b}, c is {}", b);
    let arr: [i64;5] = [3;5];
    println!("array is: {:?}", arr);
    let arr = [3;5];
    println!("array is: {:?}", arr);

    let mut line = String::new();
    let result = io::stdin().read_line(&mut line);
    let mut index: String = String::new();
    let result = io::stdin().read_line(&mut index);
    let index:usize = index.trim().parse().expect("not a number");
    
    println!("the index: {index} of line: {line} is {}", line);

}

fn formatted_print() {
    println!("{} days", 31);
    println!("{0} plus {1} is {2}", 1, 2, 1+2);
    println!("{subject} {verb} {object}", subject="hello", verb=1, object=3.14);
    println!("base 10: {}", 9999);
    println!("base 2(Binary): {:b}", 9999);
    println!("base 8(Octal): {:o}", 9999);
    println!("base 16(Hexadecimal): {:x}", 9999);
    println!("base 16(Hexadecimal): {:X}", 9999);
    println!("{:>10}", 9999);
    println!("{:0>10}", 9999);
    println!("{:0<10}", 9999);
    println!("my name is: {1},  {0} {1}", "bob", "james");
    let number:f64 = 1f64/3.0;
    let width:usize = 10;

    println!("{:>width$}", number);

    #[derive(Debug)]
    struct Point(i32, i32);

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> { 
            write!(f, "{0}, {1}", self.0, self.1)
         }
    }
    impl fmt::Binary for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> { 
            write!(f, "{0:>05b}, {1:>05b}", self.0, self.1)
         }
    }
    let a =Point(1,2);
    println!("{:#?}\n{}\n{:b}", a, a, a);
    

    #[derive(Debug)]
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::Display for City{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}({},{})", self.name, self.lat, self.lon)
        }
    }

    for city in [
        City{name: "Dublin", lat:53.123, lon:-6.02},
        City{name: "Ublin", lat:53.123, lon:-6.02},
        City{name: "Blin", lat:53.123, lon:-6.02},
    ] {
        println!("{}", city);
    }

    impl fmt::Display for Color{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RGB ({0}, {1}, {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}", self.red, self.green, self.blue)
        }
    }

    for color in [
        Color{red:128, green:255, blue:90},
        Color{red:0, green:3, blue:254},
        Color{red:0, green:0, blue:0},
    ]{
        println!("{}", color);
    }
    println!("one million can be write: {}", 1_000_000u64);
    let tuple_of_tuple = ((1,2,3), ('a', 'b', "OK"));
    fn reverse(pair: ((i32, i32, i32), (char, char, &str))) -> ((char, char, &str),(i32, i32, i32)){
        return (pair.1, pair.0)
    }
    println!("{tuple_of_tuple:?}\n{:?}", reverse(tuple_of_tuple));
}

fn primitives() {

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "( {0:0>2.1} {1:0>2.1} )\n( {2:0>2.1} {3:0>2.1} )", 
                self.0, self.1, self.2, self.3)
        }
    }

    fn transpose(m:&Matrix)->Matrix{
        Matrix(m.2, m.3, m.0, m.1)
    }

    let m = Matrix(2f32,2f32,3f32,4f32);
    println!("{} transpose: {}", m, transpose(&m));
    println!("{} transpose: {}", m, transpose(&m));

    fn analyze_slice(slice: &[i32]) -> (i32, usize) {
        println!("the first element is: {}", slice[0]);
        println!("the length is: {}", slice.len());
        println!("array occupies {} bytes", mem::size_of_val(slice));
        (slice[0], slice.len())
    }

    println!("analyze slice: {:?}", analyze_slice(&[1,2,3]));

}


fn main() {
    /* 
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
    */
    //data_types();
    //formatted_print();
    primitives();
}
