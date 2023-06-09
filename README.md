# CPL

Big thank you to
the [FreeCodeCamp Blog Post](https://www.freecodecamp.org/news/the-programming-language-pipeline-91d3f449c919/) for the
inspiration!

I am going to be creating a programming language from scratch. I will be documenting my progress here.
I'll be following along with [this](https://hackernoon.com/building-your-own-programming-language-from-scratch) blog
post to get started.

I'll be a compiled language, and I'll be using Rust to write it!

# Syntax

## Keywords

- `let` - Declare a variable (Type is inferred, e.g. `let x = 5`)
- `:` - Declare a variable with a type. (e.g. `let x: i32 = 5`)
- `fn` - Declare a function.
- `if` - Declare an "if" statement.
- `elif` - Declare an "else if" statement.
- `else` - Declare an "else" statement.
- `switch` - Declare a "switch" statement.
- `case` - Declare a "case" statement.
- `default` - Declare a "default" statement.
- `while` - Declare a "while" loop.
- `for` - Declare a "for" loop.
- `break` - Break out of a loop.
- `continue` - Continue to the next iteration of a loop.
- `return` - Return a value from a function.
- `none` - Null value.
- `to` - Used in ranges. (e.g. `0 to 10`)
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
- `=` - Assignment.
- `==` - Equals.
- `!=` - Not equals.
- `>` - Greater than.
- `<` - Less than.
- `>=` - Greater than or equal to.
- `<=` - Less than or equal to.
- `&&` - Logical And.
- `||` - Logical Or.
- `!` - Not.
- `&` - Bitwise And.
- `|` - Bitwise Or.
- `^` - Bitwise Xor.
- `<<` - Bitwise left shift.
- `>>` - Bitwise right shift.
- `&=` - Bitwise and assignment.
- `|=` - Bitwise or assignment.
- `^=` - Bitwise xor assignment.
- `<<=` - Bitwise left shift assignment.
- `>>=` - Bitwise right shift assignment.
- `++` - Increment.
- `--` - Decrement.
- `+=` - Addition assignment.
- `-=` - Subtraction assignment.
- `*=` - Multiplication assignment.
- `/=` - Division assignment.
- `%=` - Modulus assignment.
- `->` - Function return indicator.

## Types

- `f32` - 32-bit floating point number.
- `f64` - 64-bit floating point number.
- `i8` - 8-bit signed integer.
- `i16` - 16-bit signed integer.
- `i32` - 32-bit signed integer.
- `i64` - 64-bit signed integer.
- `i128` - 128-bit signed integer.
- `u8` - 8 bit unsigned integer.
- `u16` - 16 bit unsigned integer.
- `u32` - 32 bit unsigned integer.
- `u64` - 64 bit unsigned integer.
- `u128` - 128 bit unsigned integer.
- `none` - Null value.

## Comments

- `//` - Single line comment.
- `/* */` - Multi line comment.
- `///` - Documentation comment. (Maybe)

# Examples

## Hello World

```cpl
fn main() {
    print("Hello World!");
}
```

## Variables

```cpl
let a: i32 = 5;
let b: i32 = 10;
let c = a + b;

print(c);
```

## Functions

```cpl
fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    let c = add(a, b);
    
    print(c);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

## If Statements

```cpl
let a: i32 = 5;
let b: i32 = 10;

if a > b {
    print("a is greater than b.");
} else if a < b {
    print("a is less than b.");
} else {
    print("a is equal to b.");
}
```

## Switch Statements

```cpl
let a: i32 = 5;
    
switch a {
    1 => print("a is 1."),
    2 => print("a is 2."),
    3 => print("a is 3."),
    4 => print("a is 4."),
    5 => print("a is 5."),
    _ => print("a is not 1, 2, 3, 4, or 5."),
}
```

## While Loops

```cpl
let a: i32 = 0;

while a < 10 {
    print(a);
    a += 1;
}
```

## For Loops

```cpl
for i in 0 to 10 {
    print(i);
}
```

## Comments

```cpl
// This is a single line comment.
    
/*
 This is a multi line comment.
 It can span multiple lines. (Obviously)
 */
```
