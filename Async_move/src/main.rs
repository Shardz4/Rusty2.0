use std::future::Future;

#[tokio(main)]
async fn blocks() {
    let my_string = "foo".to_string();
    let future_one = async {
        println!("{my_string}");

    };
    let future_two = async {
        println!("{my_string}");
    };

    let ((), ()) = futures::join!(future_one, future_two);


}   

fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        println!("{my_string}");
    }
}


async fn main() {
    blocks().await;
    println!("Finished");

    let future_block = move_block();
    future_block.await;
    println!("Finished");
}