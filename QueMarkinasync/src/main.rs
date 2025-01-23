use std::rc::Rc;
#[derive(Debug)]
struct NotSend(Rc<()>)

async fn bar(){}
async fn foo(){
    let not_send = Rc::new(());
    bar().await;
    // The following line causes a compilation error
    // not_send.send(());
}

fn require_send(_: impl Send){}


fn main() {
    require_send(foo());
}
///  if we store x into a variable, it won't be dropped until after the .await, at which point the async fn may be running on a different thread. Since Rc is not Send, allowing it to travel across threads would be unsound. One simple solution to this would be to drop the Rc before the .await, but unfortunately that does not work today.