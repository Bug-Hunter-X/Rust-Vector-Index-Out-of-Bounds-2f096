fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0);
    println!("First element: {:?}", first);
    let second = vec.get(1);
    println!("Second element: {:?}", second);
    //Safe way to handle the index
    match vec.get(2) {
        Some(value) => println!("Third element: {:?}", value),
        None => println!("Index out of bounds")
    };
    //Alternative approach using unwrap_or
    let third = vec.get(2).unwrap_or(&0);
    println!("Third element (unwrap_or): {:?}", third);
}