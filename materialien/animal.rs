trait Animal {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Wuff {}", self.name);
    }
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

/// There are basically three different ways you can write a
/// function.
///
/// ```
/// fn foo<T: Bar>(x: T)            // version A
/// fn foo(x: &Bar)                 // version B
/// fn foo<T: Bar + ?Sized>(x: T)   // version C
/// ```
///
/// - A: static dispatch only, monomorphization
/// - B: dynamic dispatch only, accepting a trait object
/// - C: static dispatch, but passing a trait object is allowed;
///      this is useful if you want to allow both
///
fn speak_twice<T: Animal + ?Sized>(a: &T) {
    a.speak();
    a.speak();
}

/// We can't return `Animal` directly, because it is unsised.
/// So we need to hide it behind a pointer. But `&Animal` doesn't
/// work, because we would reference variables from the function.
/// To return an owned trait object, we use `Box<Animal>`.
fn get_user_animal() -> Box<Animal> {
    println!("If you want a dog, give me a name:");
    let mut dog_name = String::new();
    std::io::stdin().read_line(&mut dog_name).expect("oh noe!");

    let new_len = dog_name.trim_right().len();
    dog_name.truncate(new_len);


    if dog_name.is_empty() {
        Box::new(Cat)
    } else {
        Box::new(Dog { name: dog_name })
    }
}

fn main() {
    let a = get_user_animal();

    speak_twice(&*a);
}
