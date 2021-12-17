// mod error;

// use error::self;

// fn main() {
//     error::internal!();
// }

// foo::bar!();  // works

mod foo;

fn main() {
    foo::bar!();
}
