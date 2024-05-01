use breakpoint::breakpoint_inner;

macro_rules! breakpoint {
    ( $condition: expr ) => {
        breakpoint_inner!(
            "nop",
            $condition
        );
    };
    () => {
        breakpoint_inner!(
            "int 3"
        );
    };
}

fn main() {
    breakpoint!(12==13);
    println!("Hello, world!");
}
