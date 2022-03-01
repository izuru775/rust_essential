// Variables in rust are immutable by default

/*
    Local variables : snake_case
    Modules : snake_case
    Types : UpperCamelCase
    Traits : UpperCamelCase
    Enum variants : UpperCamelCase
    functions : snake_case
    Methods : snake_case
*/

// Boxes are smart pointers which points to a memory location in the heap


fn sum_boxes<T: std::ops::Add<Output = T>>(a:Box<T>,b:Box<T>)->Box<T>{
    Box::new(*a + *b)
}



fn main() {
    
    let one = Box::new(1);
    let two = Box::new(2);

    assert_eq!(*sum_boxes(one, two),3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e),5.85987);

    println!("Test Passed!");
}
