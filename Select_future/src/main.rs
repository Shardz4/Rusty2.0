use futures::future::{Fuse, FusedFuture, FutureExt};
use futures::stream::{FusedStream, Stream, StreamExt};
use futures::pin_mut;
use futures::select;

async fn get_new_num() -> u8 {5}
async fn run_on_new_num(_: u8) {}

async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin, starting_num: u8,
){
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::termianted();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);  
    loop{
        select! {
            () = interval_time.select_next_some() => {
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num - get_new_num_fut => {
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            () = run_on_new_num_fut => {},
            compolete=>panic!("'Interva_timer' completed unexpectedly")
        }
    }
}

