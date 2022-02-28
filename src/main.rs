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
#[derive(Debug)]
struct Rectangle<T,U> {
    width: T,
    height: U,
}


fn main() {
    let rect =Rectangle{
        width:1u8,
        height:3u16
    };
    println!("rect is {:?}",rect);
}
