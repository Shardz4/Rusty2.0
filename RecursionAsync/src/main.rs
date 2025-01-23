/// internally async creates a state machine type containing each sub-future being .awaited. This makes recursive async fns a little tricky, since the resulting state machine type has to contain itself.


async fn foo(){
    step_one().await;
    step_two().await;
}
// generates a type like this

enum Foo{
    first(Stepone),
    second(Steptwo),
}

async fn recursive() {
    recursive().await;
    recursive().await;
}

enum Recursive{
    first(Recursive),
    second(Recursive),
}

/// this won't work as we've created an infinitely sized type. 
/// inorder to allow this we have to introduce an indirection using Box. Unfortunately, compliler limitations mean that just wrapping the calls tp recursive() in Box::pin isn't enough.
/// To make this work we have to make recursive into a nonasync function that returns a .boxed() async block:

use futures::future::{BoxFuture,FutureExt}
fn recursive() -> BoxFuture<' static, ()> {
    async move{
        recursive().await;
        recursive().await;
    }.boxed()
}