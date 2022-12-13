## Library for demonstration

## Usage

```
    // Cargo.toml

    [dependencies]
    hello_lib = "0.1.3"
```

```
    // main.rs

    use hello_lib::hello;

    fn main() {
        println!("{}", hello("Rachel"));
    }

```