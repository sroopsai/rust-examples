fn main() {
    println!("Hello, world!");
    // Variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular type annotation
    let an_integer = 5i32; // suffix annotation

    // Or a default will be used
    let default_float = 3.0; // 'f64'
    let default_integer = 7; // 'i32'

    // A type can also be inferred from the context
    let mut inferred_type = 12; // Type i64 inferred from next line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed
    let mut mutable = 12; // Mutable 'i32'
    mutable = 21;

    // Error! the type of a variable can't be changed
    // mutable = true;

    /* Compound types - Arrays and Tuples */

    // Array signature consists of type T and length as [T; length]
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple is collection of values of different types
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    
}
