fn main() {
    let mut v = vec![1, 2, 3];
    let value = v[0]; //Safe copy the value from the vector
    v[0] = 4;
    println!("v: {:?}", v);
    println!("value: {}", value);
}