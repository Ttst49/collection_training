fn vector_training(){

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
        Some(troisieme)=>println!("Le troisieme element est {}",troisieme),
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

    let line = vec![
        Cell::Int(63),
        Cell::Float(15.92),
        Cell::Text(String::from("coucou"))
    ];


}



fn main() {
    vector_training()
}



