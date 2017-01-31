use std::cell::RefCell;
use std::rc::Rc;

/// A helper type to visualise drops on the terminal. Isn't it cute? :3
struct HappyDrop;

impl Drop for HappyDrop {
    fn drop(&mut self) {
        println!("❤ I have been dropped! ╰( ◕ ᗜ ◕ )╯");
    }
}

/// We need another wrapper type to avoid having infinitely nested types. This
/// is also useful to include the drop-indicator.
struct Node {
    /// Drop-indicator. This could also be sensitive data that requires
    /// its destructor to be executed (you shouldn't rely on that for safety
    /// purposes!).
    ///
    /// Silence warnings, because the `drop()` *is* used.
    #[allow(dead_code)]
    happy: HappyDrop,
    /// An optional shared reference to another node, which can mutated through
    /// an immutable reference.
    rc: RefCell<Option<Rc<Node>>>,
}

fn main() {
    // We create our first object, without a reference.
    let one = Rc::new(Node {
        happy: HappyDrop,
        rc: RefCell::new(None),
    });

    // This is the second node which now also owns the first node.
    let two = Rc::new(Node {
        happy: HappyDrop,
        // `two` is owner of `one` now. Note that we `clone()` the Rc around
        // one to create a new owner, instead of moving the ownership from the
        // stack frame into two. After this, `two` and the `main` stackframe
        // are owner of `one`.
        rc: RefCell::new(Some(one.clone())),
    });

    // This next line closes the cycle. We can access `one` here, because our
    // stackframe is owner too! We use the RefCell to mutate the optional
    // reference to another node and make `one` owner of `two`.
    *one.rc.borrow_mut() = Some(two);
}
