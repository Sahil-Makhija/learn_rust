- programming language known for concurrency, performance and its reliability.
- Statically-typed language , *however* Variable Types are inferred, need not to be declared by the programmer.
```rust
fn main(){

let mut var = 1;

println!("Variable value : {var}");

println!("Changing value to another type...");

var = "Its string now."; //does not work, because type var is number

println!("Variable value : {var}");

}
```

<hr>

**Statically vs Dynamically typed languages**
In statically-typed language, types of variables are checked during compile time, while in dynamically-typed , they are checked at runtime.

<hr>

- Rust can also be used for creating high-performant web servers.
- Its a *systems language*, meaning it is used to create binaries, cli tools for systems.
- therefore, its also used in `exploit development`.

<hr>

## Rust Compiler

- Before running a program, it needs to be *compiled* first, just like in c++. 
- Rust compiler is called `rustc`
```shell
$ rustc main.rs // compiling a program `main.rs`
```

- 