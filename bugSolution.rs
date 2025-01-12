fn main() {
    let mut x = 5;
    { // y is only borrowed while in the scope
        let y = &mut x;
        *y += 1;
    }
    println!("x = {}", x);
    let z = &x; // Now it is safe to create an immutable reference
    println!("x = {}", *z);
}