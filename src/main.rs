use core::fmt;
use std::{collections::{HashMap, HashSet}, io, mem, ops::Add, sync::Arc};

struct Solution{

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

    fn analyze_slice<T:fmt::Display>(slice: &[T]) -> (i32, usize) {
        println!("the first element is: {}", slice[0]);
        println!("the length is: {}", slice.len());
        println!("array occupies {} bytes", mem::size_of_val(slice));
        (format!("{}", slice[0]).parse().expect("error!"), slice.len())
    }

    println!("analyze slice: {:?}", analyze_slice(&[1,2,3]));
    let mut array =[1i16;16];
    for i in [12,2,3,4] {
        //let i:i32 = i;
        array[i] = (i+1) as i16;
    }
    println!("analyze slice: {:?}", analyze_slice(&array[1 .. 10]));

}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for i in 0..k {
            //println!("i is {i}");
            if i == 0 {
                for j in 1..(n+1) {
                    let item = Vec::from_iter([j]);
                    result.push(item);
                }
            }else{
                let mut new_result = Vec::new();
                for j in 1..(n+1) {
                    for item in &result {
                        if item[item.len()-1] < j {
                            let mut p = item.clone();
                            p.push(j);
                            new_result.push(p);
                        }
                    }
                }
                result = new_result;
            }
        }
        result

    }
}

fn test_77() {
    println!("{:?}",Solution::combine(4, 2));
    println!("{:?}",Solution::combine(1, 1));
    println!("{:?}",Solution::combine(20, 15));
}

fn test_78() {
    impl Solution {
        pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut nums = nums.clone();
            let mut result = Vec::new();
            let mut buffer: Vec<Vec<i32>> = Vec::new();
            result.push(Vec::new());
            buffer.push(Vec::new());
            nums.sort();
            for i in 0..nums.len() {
                let mut new_buffer: Vec<Vec<i32>> = Vec::new();
                for item in &buffer {
                    for num in &nums {
                        if item.is_empty() {
                            new_buffer.push(Vec::from_iter([*num]));
                        }else if item[item.len()-1] < *num{
                            let mut p = item.clone();
                            p.push(*num);
                            new_buffer.push(p);
                        }
                    }
                }
                buffer = new_buffer.clone();
                result.append(&mut new_buffer);
            }
            result
        }
    }
    println!("{:?}", Solution::subsets(Vec::from_iter([1,2,3])));
    println!("{:?}", Solution::subsets(Vec::from_iter(1..11)));
}

fn custom_types() {

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    #[derive(Debug)]
    struct Point(i32, i32);

    #[derive(Debug)]
    struct Rectangle {
        center: Point,
        user: Person,
    }

    println!("{:?}", Rectangle{center: Point(1,2), user: Person{name:String::from("ab"), age:35}})
}

fn leetcode_79(){
    impl Solution {
        pub fn exist0(board: Vec<Vec<char>>, word: String) -> bool {

            struct Point(i16, i16);

            // Implement From function for Point with i32 as parameter
            impl From<i32> for Point {
                fn from(item: i32) -> Self {
                    Point((item/10000) as i16, (item%10000) as i16)
                }
            }

            //Implement Into function for Point with i32 as parameter
            impl Into<i32> for Point {
                fn into(self) -> i32 {
                    (self.0 as i32)*10000 + self.1 as i32
                }
            }

            //Implement Add function for two points
            impl Add for Point {
                type Output = Point;

                fn add(self, other: Point) -> Point {
                    Point(self.0 + other.0, self.1 + other.1)
                }
            }

            //Implement clone function for struct Point
            impl Clone for Point {
                fn clone(&self) -> Point {
                    Point(self.0, self.1)
                }
            }

            // Implement a struct: Trie Tree
            // Implement a function: insert(word)
            struct TrieTree {
                position: Point,
                children: Option<HashMap<char, TrieTree>>,
            }

            //Implement new function for TrieTree
            impl TrieTree {
                fn new() -> Self {
                    TrieTree {
                        position: Point(0, 0),
                        children: None,
                    }
                }

                //Implement insert function for TrieTree, with char as parameter, and return the child trietree node as return value
                fn insert(&mut self, word: char) -> &mut TrieTree {
                    if self.children.is_none() {
                        self.children = Some(HashMap::new());
                    }
                    self.children.as_mut().unwrap().entry(word).or_insert(TrieTree::new())
                }

                //Get the corresponding TrieTree child node for given parameter: word as char, return the child node as mutable reference
                fn get_child(&mut self, word: char) -> Option<&mut TrieTree> {
                    if self.children.is_none() {
                        return None;
                    }
                    self.children.as_mut().unwrap().get_mut(&word)
                }
            }

            fn iter_node(node: &mut TrieTree, board: &Vec<Vec<char>>, pos: Point, path: &mut HashSet<i32>) {
                let offsets = [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];
                for offset in offsets {
                    let new_point = offset + pos.clone();
                    let (x, y) = (new_point.0, new_point.1);
                    if x >= 0 && x < board.len() as i16 && y >= 0 && y < board[0].len() as i16 {
                        let new_pos = new_point.into();
                        if !path.contains(&new_pos) {
                            path.insert(new_pos);
                            let child = node.insert(board[x as usize][y as usize]);
                            iter_node(child, board, Point::from(new_pos), path);
                            path.remove(&new_pos);
                        }
                    }
                }
            }

            let mut root = TrieTree::new();
            let mut path = HashSet::new();
            for i in 0..board.len() {
                for j in 0..board[i].len() {
                    let pos:i32 = Point(i as i16, j as i16).into();
                    path.insert(pos);
                    let node = root.insert(board[i][j]);
                    iter_node(node, &board, Point::from(pos), &mut path);
                    path.remove(&pos);
                }
            }
            
            let result = true;

            let mut node  = &mut root;
            for c in word.chars() {
                if let Some(child) = node.get_child(c) {
                    node = child;
                }else{
                    return false;
                }
            }

            result
        }
    }

    //imp Into function from Vec<&str> to Vec<Char>
    fn from(item: Vec<&str>) -> Vec<char> {
        let mut result = Vec::new();
        item.into_iter().for_each(|s| {
            result.push(s.clone().chars().nth(0).unwrap());
        });
        result
    }

    let graph = Vec::from_iter([
            Vec::from_iter(['a', 'b', 'c', 'd', 'e', 'f',]),
            Vec::from_iter(['a', 'b', 'c', 'd', 'e', 'f',])]);
    println!("{}", Solution::exist(graph.clone(), String::from("abcddef")));
    println!("{}", Solution::exist(graph.clone(), String::from("aabbccddeed")));
    println!("{}", Solution::exist(graph.clone(), String::from("aabbccddeeffe")));
    let graph : Vec<Vec<char>> = Vec::from_iter([
        from(["A","B","C","E", "F", "G"].to_vec()),
        from(["S","F","C","S", "W", "Z"].to_vec()),
        from(["A","D","E","E", "R", "T"].to_vec()), 
        //from(["A","B","C","E", "F", "G"].to_vec()),
        //from(["S","F","C","S", "W", "Z"].to_vec()),
        from(["A","D","E","E", "R", "T"].to_vec())]);
    println!("{}", Solution::exist(graph.clone(), String::from("ABCCSEFGZWRT")));


}


fn leetcode_79_2() {
    impl Solution {
        pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
            struct Point(i16, i16);
            // Implement From function for Point with i32 as parameter
            impl From<i32> for Point {
                fn from(item: i32) -> Self {
                    Point((item/10000) as i16, (item%10000) as i16)
                }
            }
            //Implement Into function for Point with i32 as parameter
            impl Into<i32> for Point {
                fn into(self) -> i32 {
                    (self.0 as i32)*10000 + self.1 as i32
                }
            }
            //Implement Clone function for Point
            impl Clone for Point {
                fn clone(&self) -> Point {
                    Point(self.0, self.1)
                }
            }
            //Implement Add function for two points
            impl Add for Point {
                type Output = Point;

                fn add(self, other: Point) -> Point {
                    Point(self.0 + other.0, self.1 + other.1)
                }
            }

            fn match_it(board: &Vec<Vec<char>>, word: &String, index:usize, position: Point, path: &mut HashSet<i32>) -> bool {

                if index >= word.len() {
                    return true;
                }

                if board[position.0 as usize][position.1 as usize] != word.chars().nth(index).unwrap() {
                    return false;
                }

                let pos: i32 = position.clone().into();
                if path.contains(&pos) {
                    return false;
                }
                path.insert(pos);

                if index == word.len()-1 {
                    return true;
                }
                let offsets = [Point(-1, 0), Point(1, 0), Point(0, -1), Point(0, 1)];
                for offset in offsets {
                    let mut new_pos = offset + position.clone();
                    let (x, y) = (new_pos.0, new_pos.1);
                    if x>= 0 && x < board.len() as i16 && y >= 0 && y < board[0].len() as i16 {
                        if !path.contains(&new_pos.clone().into()) {
                            if match_it(board, word, index+1, new_pos, path) {
                                return true;
                            }
                        }
                    }
                }
                path.remove(&pos);
                false
            }

            for i in 0..board.len() {
                for j in 0..board[i].len() {
                    let mut path = HashSet::new();
                    if match_it(&board, &word, 0, Point(i as i16, j as i16), &mut path) {
                        return true;
                    }
                }
            }
            false
        }
    }

    //imp Into function from Vec<&str> to Vec<Char>
    fn from(item: Vec<&str>) -> Vec<char> {
        let mut result = Vec::new();
        item.into_iter().for_each(|s| {
            result.push(s.clone().chars().nth(0).unwrap());
        });
        result
    }

    let graph = Vec::from_iter([
            Vec::from_iter(['a', 'b', 'c', 'd', 'e', 'f',]),
            Vec::from_iter(['a', 'b', 'c', 'd', 'e', 'f',])]);
    println!("{}", Solution::exist(graph.clone(), String::from("abcddef")));
    println!("{}", Solution::exist(graph.clone(), String::from("aabbccddeed")));
    println!("{}", Solution::exist(graph.clone(), String::from("aabbccddeeffe")));
    let graph : Vec<Vec<char>> = Vec::from_iter([
        from(["A","B","C","E", "F", "G"].to_vec()),
        from(["S","F","C","S", "W", "Z"].to_vec()),
        from(["A","D","E","E", "R", "T"].to_vec()), 
        from(["A","B","C","E", "F", "G"].to_vec()),
        from(["S","F","C","S", "W", "Z"].to_vec()),
        from(["A","D","E","E", "R", "T"].to_vec())]);
    println!("{}", Solution::exist(graph.clone(), String::from("ABCCSEFGZWRTGZTR")));
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
    //primitives();
    //test_77();
    //test_78();
    //custom_types();
    leetcode_79_2();
}
