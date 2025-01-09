fn main() {
    let mut x = 5;
    { //creating a new scope to handle the multiple mutable borrows
        let y = &mut x;
        *y += 1;
    }
    { //creating a new scope to handle the multiple mutable borrows
        let z = &mut x;
        *z += 2; 
    }
    println!("x = {}", x);
}