# CPL
Thank you to the [FreeCodeCamp Blog Post](https://www.freecodecamp.org/news/the-programming-language-pipeline-91d3f449c919/) for the inspiration!

I am going to be creating a programming language from scratch. I will be documenting my progress here.
I'll be following along with [this](https://hackernoon.com/building-your-own-programming-language-from-scratch) blog post to get started.

I'll be a compiled language, and I'll be using Rust to write the compiler.

# Syntax
## Keywords
- `let` - Declare a variable (Type is inferred, let x = 5)
- `:` - Declare a variable with a type. (let x: i32 = 5)
- `fn` - Declare a function.
- `if` - Declare an if statement.
- `else if` - Declare an else if statement.
- `else` - Declare an else statement.
- `switch` - Declare a switch statement.
- `while` - Declare a while loop.
- `for` - Declare a for loop.
- `break` - Break out of a loop.
- `continue` - Continue to the next iteration of a loop.
- `return` - Return a value from a function.
- `null` - Null value.
- `import` - Import a module. (Maybe)
- `export` - Export a module. (Maybe)

## Literals
- `true` - Boolean true.
- `false` - Boolean false.
- `0 - 9` - Numbers (Including decimals).
- `"` - String.
- `'` - Character.

## Operators
- `+` - Addition.
- `-` - Subtraction.
- `*` - Multiplication.
- `/` - Division.
- `%` - Modulus.
- `^` - Exponent.
- `=` - Assignment.
- `==` - Equals.
- `!=` - Not equals.
- `>` - Greater than.
- `<` - Less than.
- `>=` - Greater than or equal to.
- `<=` - Less than or equal to.
- `&&` - And.
- `||` - Or.
- `!` - Not.
- `&` - Bitwise and.
- `|` - Bitwise or.
- `~` - Bitwise not.
- `<<` - Bitwise left shift.
- `>>` - Bitwise right shift.
- `&=` - Bitwise and assignment.
- `|=` - Bitwise or assignment.
- `~=` - Bitwise not assignment.
- `<<=` - Bitwise left shift assignment.
- `>>=` - Bitwise right shift assignment.
- `++` - Increment.
- `--` - Decrement.
- `+=` - Addition assignment.
- `-=` - Subtraction assignment.
- `*=` - Multiplication assignment.
- `/=` - Division assignment.
- `%=` - Modulus assignment.
- `^=` - Exponent assignment.
- `->` - Function arrow.

## Types
- `i8` - 8 bit signed integer.
- `i16` - 16 bit signed integer.
- `i32` - 32 bit signed integer.
- `i64` - 64 bit signed integer.
- `i128` - 128 bit signed integer.
- `u8` - 8 bit unsigned integer.
- `u16` - 16 bit unsigned integer.
- `u32` - 32 bit unsigned integer.
- `u64` - 64 bit unsigned integer.
- `u128` - 128 bit unsigned integer.
- `f32` - 32 bit floating point number.
- `f64` - 64 bit floating point number.
- `bool` - Boolean.
- `char` - Character.
- `str` - String.
- `null` - Null.
- `void` - Void.

## Comments
- `//` - Single line comment.
- `/* */` - Multi line comment.
- `///` - Documentation comment. (Maybe)
