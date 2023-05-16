fn main() {
    /// define and init a vector
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    /// mutate the vector
    v.push(4);

    /// read its value
    let third: &i32 = &v[2];
    /// !!! cause the program to panic
    /// let does_not_exist = &v[100];
    /// it returns None without panicking
    let does_not_exist = v.get(100);

    /// iterating over the values in vec
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
    println!("{}", third);

    /// Rust has only one string type in the core language, which is the string slice `str`
    /// String type, which is provided by Rust’s standard library rather than coded into the core language
    let mut data = "initial contents";
    let mut s = data.to_string();
    s.push_str(" something else");

    /// string concat and format
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /// fn add(self, s: &str) -> String {
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    /// two methods for iterating
    /// s.chars() s.bytes()


    /// hashmap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}
