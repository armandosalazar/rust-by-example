use std::fmt;

fn main() {
    // hello_world();
    // formatted_print();
    // debug();
    // display();
    primitives();
}

// Hello World
fn hello_world() {
    println!("Hello World!");
    // Comments
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

fn formatted_print() {
    // https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
    let x: u8 = 20;
    let str_format = format!("x = {}", x);
    println!("{}", str_format);
    eprintln!("Error");
}

fn debug() {
    #[derive(Debug)]
    struct Human(u8);

    println!("{:?}", Human(12));
    println!("{:#?}", Human(12))
}

fn display() {
    // Default
    #[derive(Debug)]
    struct Struct(u8);

    impl fmt::Display for Struct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("{:?}", Struct(1));
    // Personal
    #[derive(Debug)]
    struct Point {
        x: u8,
        y: u8,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    println!("{}", Point { x: 0, y: 1 });
    println!("{:?}", Point { x: 0, y: 1 })
}

fn primitives() {
    println!("{}", 12u8)
}
