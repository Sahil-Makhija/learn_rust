- Just like `javascript` has `npm` , rust has `cargo` which is used to manage packages and dependencies (called *crates* in rust).

### Creating a new project with Cargo

```shell
$ cargo new my_app
$ cd my_app
```

- Cargo initializes a new directory for our project called *my_app*. Inside this dir,
```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs // contains hello_world fn
└── target
    ├── CACHEDIR.TAG
    └── debug
```

- `Cargo.toml` is like `package.json` in `javascript`. It contains list of packages used/installed in our current project.
```toml
[package] name = "hello_cargo" 
version = "0.1.0" 
edition = "2024" 

[dependencies]
```
- Add a new package in this file:
```shell
// $ cargo add <package_name>@<version>
$ cargo add rand@0.8.5
```

- Now, Install the added packages.

```shell
$ cargo build
// adds new packages added to the cargo.toml file and also installs any other crates required by those packages
```

### Building our project

**For Development / Testing**

```bash
$ cargo build
```

- lesser build time
- not optimized
- build file located inside `./debug/`

**For Production**

```bash
$ cargo build --release
```

- longer build time
- optimized build
- build file located inside `./release/`

### Running our project

```shell
// After building...
$ cargo <build_file_name>

// or directly build & run
$ cargo run
```

#### Check for compilation errors

```shell
$ cargo check // checks for any compilation errors in current project
```

### Version Control System

- By default `cargo` uses `git` as its default vcs.
- To use any other vcs, 
```shell
$ cargo new --vcs=gitlab
```

