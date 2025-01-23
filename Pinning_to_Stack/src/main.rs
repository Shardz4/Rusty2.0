// if we required a pinned pointer instead

use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a:String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test{
    fn new(txt: &str) -> Self {
        Test{
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type '!Unpin;

        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut()};
        this.b = self_ptr;
    }
    fn a(self: Pin<&Self>) -> String{
        self.get_ref().a
    }
    fn b<'a> (self: Pin<&'a Self>) -> &'a String {
        assert!(!self.b.is_null(), "Test:: b called without Test::init being called first");
        unsafe { &*self.b }
    }
}

pub fn main() {
    let mut test1 = Test::new("test1");
    let mut test1 = unsafe{ Pin::new_unchecked(&mut test1)};
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe{ Pin::new_unchecked(&mut test2)};
    Test::init(test2.as_mut());

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    /// if we add this it will gove us a compile time error
    /// std::mem::swap(test1.get_mut, test2.get_mut());
    println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
    // Panic: Test::b called without Test::init being called first
}

/// its importatnt to note that stack piining will alwats rely on guarantees oyu give when writing unsafe.
/// while w weknow that the pointee of &'a mut T is pinned for the lifetime of 'a we can't know that the data &'a mut T points to isn't movef after 'a ends. if it does ot will violate the pIn contract.


