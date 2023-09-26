# Data Types
Every value in Rust is of a certain type, which tells Rust what kind of data is being specified, so it knows how to work with that data. 
Two subsets: Scalar and Compound

Nb. Rust is a *statically typed* language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. 
In situations where many types are possible we must add a type annotation, like
this: 
```rust
let guess: u32 = "42".parse().expect("Not a number!"); 
```


## Scalar Types
Represents a *single* value. 

Four primary types: 
1. Integers
2. Floating-point numbers
3. Booleans
4. Characters


Integer Types: 
An integer is a number without a fractional component. 

Length     Signed      Unsigned
8-bit      i8          u8
16-bit     i16         i16
32-bit     i32         u32
64-bit     i64         u64
128-bit    i128        u128
arch       isize       usize

Each variable can be either signed or unsigned, and has an explicit size. 
Signed means it can be negative. Unsigned means positive only. 
Signed numbers are stored using [two's
complement](https://en.wikipedia.org/wiki/Two's_complement) representation. 

Each signed number can store numbers from -(2^(n-1)) to 2^(n-1) - 1 inclusive,
where *n* is the number of bits the variant uses. 
e.g. an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127. 

(The discrepancy is because it is -1 to -128 (a span of 128 digits) and 0 - 127
(another span of 128 digits))

isize and usize depend on the architecture you're on. 64 bits on a 64 bit
system, 32 on a 32 bit system. 

Number literals     Example
Decimal             98_222 // Note the use of _ to make the number easier to
read 
Hex                 0xff
Octal               0o77
Binary              0b1111_0000
Byte (u8 only)      b'A'

How do you know which type of integer to use? 
Rust's defaults are usually a good place to start. 
Integer types default to i32. 
The primary size in which you'd use isize or usize is when indexing some sort of
collection. 

// Note to self: Do more research into the Integer Overflow issue. 
// wrapping_*   wrapping_add     checked_*    overflowing_*   saturating_*

Floating points
Numeric operations
Boolean type
Character type


Compound types: 
Tuple type

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); 
}
```

```rust
fn main() {
    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup; 

    println!("The value of y is: {y}");
}
```

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; 

    let six_point_four = x.1; 

    let one = x.2;
}
```
Array type
Unlike the tuple type must have the same type. 
Arrays in Rust have a fixed length. 

Arrays are useful when you want your data allocated on the stack rather than the heap, or when you want to ensure you always have a fixed number of elements. 

An array isn't as flexible as the vector type. A vector is a similar collection type provided by the standard library that *is* allowed to grow or shrink in size. If you're unsure whether you should use an array or a vector, chances are you should use a vector. 

let months = ["January", "February", ... , "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5]; 

Here, i32 is the type of each element. 

let a = [3; 5]; // This is the same as writing let a = [3, 3, 3, 3, 3];

a[0]

If you try to access an array through an index that is outside of the index range, Rust will panic. 
