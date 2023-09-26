

fn main() { // Define the main function
    let x = 5; // Declare a variable x, assign it the value of 5

    let x = x + 1; // Creates a new variable x by repeating let x =, taking the original value and
                   // adding 1 so that the value of x is then 6. 

    { // Inner scope created by curly brackets
        let x = x * 2; // Within the inner scope, this let statement shadows x and creates a new
                       // variable. 
        println!("The value of x in the inner scope is: {x}"); 
    }
    // When the scope ends the inner shadowing ends, and x returns to being 6. 
    // Using let allows us to perform a few transformations on a value, but have the value be
    // immutable after those transformations have been completed. 
    //
    // If we had not used `let` we would have generated a compilation error. 
    //
    // Shadowing means the second variable is what the compiler will see whan you use the name of
    // the variable. The second variable *overshadows* the first, taking any uses of the variable
    // name to itself until either it itself is shadowed or the scope ends. 
    //
    // The other difference between `mut` and shadowing: We are effectibely creating a new variable
    // when we use the let keyword again, which means we can change the *type* of the value, but
    // reuse the same name. 
    // e.g. 
    // let spaces = "    "; // a string type
    // let spaces = spaces.len(); // a number type
    // If we used `mut` we would get a compile-time error as we are not allowed to mutate a
    // variable's type. 
    //
    println!("The value of x in the outer scope is: {x}"); 
}
