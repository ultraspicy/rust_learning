use core::hash;
use std::hash::Hash;

fn main() {
    // Vec
    // define and init a vector
    // mutate the vector
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);

    // read its value
    // if read value directly, it may panic
    // let does_not_exist = &v[100];
    // use get will return a reference to the element or subslice
    let third: &i32 = &v[2];
    let does_not_exist = v.get(100);
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
    println!("{}", third);

    // String
    // Rust has only one string type in the core language, which is the string slice `str`
    // String type, which is provided by Rust’s standard library rather than coded into the core language
    // `str` is unsized, String = sized (24 bytes on stack + variable heap data)
    // So we can `let a :String = ...` but no `let a: str = ...`
    let mut data = "initial contents";
    let mut s = data.to_string();
    s.push_str(" something else");

    // string concat and format
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // fn add(self, s: &str) -> String {
    // first para is self, will take the ownership. Second para is borrow
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s_new_empty = String::new(); // create empty string
    let s_from_literal = String::from("string_literal"); // create string from literal
    let s_from_literal_2 = "string_literal".to_string(); // create sstring from literal

    let mut s_to_append = String::from("hello");
    s_to_append.push_str(" world"); // push &str, this method is &mut self
    s_to_append.push('c'); // also &mut self, but push a single char

    println!("s_to_append len = {}", s_to_append.len()); // size
    println!("s_to_append is_empty = {}", s_to_append.is_empty()); // empty

    let s_to_trim = String::from("  s_to   trim.   "); // trim heading and trailing whitespaces
    println!("after trim: {}", s_to_trim.trim());

    let s_to_split_comma = String::from("a,b,c,d,e");
    let after_split_comma = s_to_split_comma.split(',');
    for sub in after_split_comma {
        println!("after split: {}", sub);
    }

    let s_to_format = format!("format: {} dollars and {} cents", 5, 3);
    println!("{}", s_to_format);
    // other methods 
    // contains(), starts_with(), ends_with(), replace(), to_lowercase(), to_uppercase()


    // hashmap
    // get(&key) - Returns Option<&V>, immutable reference to value
    // get_mut(&key) - Returns Option<&mut V>, mutable reference to value
    // get_key_value(&key) - Returns Option<(&K, &V)>, both key and value

    // contains_key(&key) - Returns bool, checks if key exists
    // remove(&key) - Returns Option<V>, removes and returns the value
    // remove_entry(&key) - Returns Option<(K, V)>, removes and returns both key and value
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // entry() returns an Entry enum, which has two variants:
    // Occupied(OccupiedEntry) - key exists in the map
    // Vacant(VacantEntry) - key doesn't exist

    // entry() method
    // or_insert(default_value): Insert if key doesn't exist, return mutable reference either way
    // very useful when you need insert default value and immediately manipulate the V
    let mut hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
    hashmap.entry(1).or_insert(Vec::new()).push(0);

    // similar or_insert_with(Fn)
    hashmap.entry(2).or_insert_with(|| {
        let mut v: Vec<i32> = vec![];
        v.push(1);
        v.push(2);
        v
    });
}
