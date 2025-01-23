// async{
//     let mut x = [0; 128];
//     let read_into_buf_fut = read_into_buf(&mut x);
//     read_into_buf_fut.await;
//     println!("{:?}", x);
// }

// struct Readintobuf<'a> {
//     buf: &'a mut [u8],
    
// }

// struct AsyncFuture{
//     x: [u8; 128],
//     read_into_buf_fut: Readintobuf<'what_lifetime?>,
// }
// /// Here,  the readinto buf future holds a referenc into other field og our structure, x.
// /// however, if AsyncFuture is moved the location of x will move as well, invalidating the pointer in read_into_buf_fut.buf
// /// 
// /// pinning futures to a particular spot in memory prevents thsi problem, making it safe to create references inside an async block.
// /// 

#[derive(Debug)]
struct Test{
    a: String,
    b: *const String, // pointer to a string
}
impl Test {
    fn new(txt: &str) -> Self{
        Test{
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self){
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }
    fn a(&self) -> &str {
        &self.a
    }
    fn b(&self) -> &String {
            assert!(!self.b.is_null(), "Test::b called without Test::init being callled first");
            unsafe { &*(self.b)}
    }
}

/// Since b is refernce to a we store it as a pointer since the borrowing rules of Rust doesn't allows us to define its lifetime. This is a self referencial struct

fn main() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {},", test1.a(), test1.b());
    println!("a: {}, b: {},", test2.a(), test2.b());
    println!("\n");
    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b: {},", test1.a(), test1.b());
    println!("a: {}, b: {},", test2.a(), test2.b());
    /// the pointer to test2.b still points to the old location which is inside test1 now. the struct is not self referential anymore, it holds a pointer to a field in a different object.
    ///  That means we can't rely on the lifetime of test2.b to be tied to the lifetime of test2 anymore
}

