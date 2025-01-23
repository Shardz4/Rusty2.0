/// Pinning an !unpin type to the heap gives our data a stable address so we know that the data we point to can't move after its pinned. in contrast to staxk pinnning, w e know that the data will be pinned for the lifetime for the lifetime of the object

use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test{
a: String,
b: *const String,
_marker: PhantomPinned,
}

impl Test {
    fn new (txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.a;
        unsafe {
            boxed.as_mut(). get_unchecked_mut().b = self_ptr};
            boxed
        }
        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }
        fn b(self: Pin<&Self>) -> *const String {
            self.get_ref().b
        }
    }

    pub fn main() {
        let test1 = Test::new("test1");
        let test2 = Test::new("test2");

        println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
        println!("a: {}, b: {:?}", test2.as_ref().a(), test2.as_ref().b());
    }
