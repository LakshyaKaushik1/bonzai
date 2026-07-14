### There are 5 ways to print something
- print! : It will print the standard output without adding a newline
- println! : this will print the standard output with a new line
- eprint! : prints the standard error without a new line
- eprintln : prints the standard error with a new line
- format! : It does not print in the console, instead it write the text into a dynamic string for later use.

### Positional arguments
- println!("{0} is best than {1}. Oh wait, {0} is awesome.", "Rust", "C++")
Output: Rust is better than C++. Oh wait, Rust is awesome

### Variables
- you declare a variale in Rust with the 'let' keywords.
- Syntax: let <type> <variable_name>
- Here type can be mutable (mut) or immutable. By default the variable type is immutable.

### Associated Functions
- It is a type of funtion that is tied to a specific type (string, int, float, etc).
- We define it as TypeName :: <functions_name>()
- We use it for string because they vary in size and can get complex quickly.
- But for int and float, we don't use it as they take exactly 32 bits in the stack memory.
- new() is an associated function of the String type. It create a new empty string (just like a = "" in python)