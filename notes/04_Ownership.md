## What is Ownership ?

- _Ownership_ is a set of rules that govern how a Rust program manages memory. 
- All programs have to manage the way they use a computer’s memory while running. 
- Some languages have **garbage collection** that regularly looks for no-longer-used memory as the program runs; in other languages, **the programmer must explicitly allocate and free the memory**. 
- Rust uses a third approach: memory is managed through **a system of ownership** with a **set of rules** that the compiler checks. 
- If any of the rules are violated, the program won’t compile. 
## Ownership Rules

- Each value in Rust has an _owner_.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped. 

The owner is the variable. The memory is automatically returned once the variable that owns it goes out of scope.

---
### Difference b/w `String literals` & `Strings`
#### **String literals**

String literals are string slices that have a fixed size and are immutable. They're typically created using double quotes and have the type `&str`:

```rust
let literal: &str = "Hello, world!";
// This is stored in the program's binary (read-only memory)
```

Key characteristics:
- **Immutable**: Cannot be modified after creation
- **Fixed size**: Size is known at compile time
- **Stack allocated reference**: The reference itself is on the stack, but points to data in static memory
- **Borrowed**: It's a reference to string data stored elsewhere
- **Lifetime**: String literals have a `'static` lifetime (they live for the entire program)
#### **String Type (`String`)**

`String` is a heap-allocated, `growable`, owned string type:

```rust
let mut string = String::from("Hello");
string.push_str(", world!");
// This is stored on the heap and can be modified
```

**Key characteristics:**
- **Mutable**: Can be modified if declared with `mut`
- **Growable**: Size can change at runtime
- **Heap allocated**: Data is stored on the heap
- **Owned**: The `String` owns its data
- **UTF-8 encoded**: Guaranteed to be valid UTF-8
#### **Why not to use `let mut s = "Hello";` ?**

```rust
let mut s = "hello";

// ✅ This works - reassigning the variable to a different &str
s = "world";  
s = "goodbye";

// ❌ This does NOT work - cannot modify the string content
// s.push_str(" world");  // Error: &str doesn't have push_str method
// s[0] = 'H';  // Error: cannot index into &str with assignment

// The variable is mutable, but it's just a reference to immutable data
println!("{}", s);  // prints: "goodbye"
```

---
## Variable Scope

```rust
fn main(){
	let s = String::from("Hello World!");
}
```

- When `s` goes out of scope, Rust calls a special function for us. 
- This function is called [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), and it’s where the author of `String` can put the code to return the memory. 
- Rust calls `drop` automatically at the closing curly bracket.

### The `String` type

```rust
let s = String::from("hello");
```

#### Copying String variables

```rust
let s1 = String::from("hello");
let s2 = s1;
```

- In this example, the value of the variable (i.e. `"hello"`)  is not being copied, instead the data of the variable `s1` is copied to `s2`.
- This data contains reference to the data in the `heap` , its length and capacity.
- Now, both `s1` and `s2` are pointing towards the same `data` in the heap.

![[Pasted image 20251007062644.png]]

- This is similar to `Shallow copy` in other languages, but in Rust, its called `moving a variable into another,`
#### Double-free error

- Since a variable is only valid until it is in-scope, this creates a problem now that we have to variables pointing towards same data in the heap.
- To solve this, rust invalidates the first variable, meaning rust does not have to free anything if the first variable goes out of scope.
- The program won't compile if we try to use the first variable , now :
```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!"); // X : error
```

#### Re-assigning a variable

```rust
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
```

- At line 1, we create a *mutable* variable of type `String` and assign it a value `"hello"` in the heap.
- At line 2, the variable `s` is *mutated*. 
	- first, The `drop()` function is called for the current state. The value `"hello"` is cleared in the heap.
	- Next, a new string is created `"ahoy"` in the heap, just like in line 1.

#### Creating a Deep copy of a string variable

- To create a deep copy (meaning, copying everything from a variable), we use the `clone()` method.
```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
```

- Here, `s1` and `s2` are two variable pointing to two different locations in the heap.
- Both variable are valid in current scope, and we can use any of them.

#### Stack-Only Data: Copy

```rust
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
```

- This code looks similar to the previous, but at line 2, a deep copy is being created of the variable `x`.
- This is because `integers` have fixed size at compile time so they can be stored completely on the stack.
- In this case, calling the `clone()` method for variable `x` will give us the same output.
