## Library for demonstration

## Usage

```
    // Cargo.toml

    [dependencies]
    hello_lib = "0.1.4"
```

```
    // main.rs

    use hello_lib::hello;

    fn main() {
        println!("{}", hello("Rachel"));
        println!("{}", hello(31);
        println!("{}", hello(3.14);
        println!("{}", hello(true));
        println!("{}", hello('G'));
    }

```