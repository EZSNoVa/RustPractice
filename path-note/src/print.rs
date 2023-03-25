mod Print {

    pub trait Printable {

        // make the functon accept a reference to self
        fn print(&self) -> String;

    }

    // create an universal function print which takes a Printable trait
    pub fn print<T: Printable>(object: T) {
        println!("{}", object.print());
    }
}

pub use Print::*;
