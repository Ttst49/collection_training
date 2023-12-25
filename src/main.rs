pub fn vector_training(){

    //create a vector without values
    let mut v : Vec<i64> = Vec::new();

    //shortcut vec! create a vector with values inside
    #[allow(unused_variables)]
        let w = vec![12,27,92];

    // .push to add values to a vector
    v.push(12);
    v.push(736);
    v.push(2);
    v.push(73);
    v.push(01);

    //get element from vector
    let element : &i64 = &v[3];
    println!("Le 3ème élement du vecteur est {}",element);

    match v.get(2) {
        Some(third)=>println!("Le troisieme element est {}",third),
        None=>println!("Il n'y a pas de troisieme element")
    }

    //iterate inside a vector with for loop
    for i in &v {
        println!("{}",i)
    }

    //inside and change value in vector with for loop
    for j in &mut v {
        *j += 50
    }

    //enum usage in order to have different types of data in a single vector
    enum Cell{
        Int(i64),
        Float(f64),
        Text(String),
    }

    #[allow(unused_variables)]
    let line = vec![
        Cell::Int(63),
        Cell::Float(15.92),
        Cell::Text(String::from("coucou"))
    ];


}

pub fn string_training(){

    //create new empty String
    #[allow(unused_variables)]
        let s = String::new();

    //create &str data
    #[allow(unused_variables)]
        let data = "initial content";

    //define a string with data inside it
    #[allow(unused_variables)]
        let s = data.to_string();

    //define a string with &str inside it
    #[allow(unused_variables)]
        let s = "initial content".to_string();

    //define a string with content inside of it with From:: method
    #[allow(unused_variables)]
        let s = String::from("initial content");

    //push_str method to add string after
    let mut s = String::from("foo");
    s.push_str("bar");

    //push_str method to add string after with a variable
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 est {}",s2);

    //push method in order to add a unique letter to a String
    let mut s = String::from("lo");
    s.push('l');

    //concatenation of 2 variables to make a bigger String
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    #[allow(unused_variables)]
        let s3 = s1 + &s2;

    //concatenation using macro format!
    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");

    let concatenate = format!("{}-{}-{}",s1,s2,s3);
    
    //getting char in a string with for loop
    for c in concatenate.chars() {
        println!("{}",c)
    }

    //getting byte in a string with for loop
    for byte in concatenate.bytes() {
        println!("{}",byte)
    }

}


use std::collections::HashMap;
pub fn hash_map_training(){

    //manual definition of hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("red"),7);

    //hash map with collect and zip from vectors
    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10,7];

    #[allow(unused_variables)]
    let scores: HashMap<_,_>  = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //define range

    let field_name = String::from("Favorite color");
    let field_data = String::from("Blue");

    let mut hash = HashMap::new();

    hash.insert(field_name,field_data);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //get a unique value from a variable
    let team_name = String::from("Yellow");
    scores.get(&team_name);

    //get all values from a for loop
    for (key,value) in &scores {
        println!("Your key is {} and it value is {}", key,value)
    }

    //rewrite a value for an existing key

    //scores.insert(String::from("Blue"),25);
    scores.insert(String::from("Blue"),33);

    println!("{:?}", scores);

    //add a value only if there is no already one for a specific key
    let mut scores = HashMap::new();
    scores.insert(String::from("Brown"), 10);

    scores.entry(String::from("Purple")).or_insert(50);
    scores.entry(String::from("Brown")).or_insert(50);

    println!("{:?}", scores);

    //check every word in a sentence and add to counter
    let text = "bonjour le monde magnifique monde";

    let mut table = HashMap::new();

    for mot in text.split_whitespace() {
        let counter = table.entry(mot).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", table);

}



fn main() {
    hash_map_training()
}



