use std::fmt;

struct Swagger<T>(pub T);

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "yolo {} swag", self.0)
    }
}

trait SwaggerExt: Sized {
    fn with_swag(self) -> Swagger<Self>;
}

impl<T> SwaggerExt for T {
    fn with_swag(self) -> Swagger<Self> {
        Swagger(self)
    }
}

fn main() {
    let pi = 3.14;

    println!("{}", pi);
    println!("{}", Swagger(pi));
    println!("{}", pi.with_swag());
}
