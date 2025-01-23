/// it is similar to future trait but can yield multiple values before completion, similar to the iterator trait. from the standard library.
/// trait Item{
/// the type of the value yielded by the stream.
/// type Item;
/// 
/// Attempts to resolve the next item in the stream.
/// Returns 'Poll::Ready(none) if not ready, 'Poll::Ready(Some(x))' if a value
/// is ready, and 'Poll::Ready(none)' if the stream has completed.
/// 
/// fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
///         -> oll<OPtion<Self::Item>>;
/// }
/// 

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // 'StreamExt::next' is similar to 'Iterator::next' but returns a type that implemnts 'Future<Output = Option <T>>'.
    assert_eq!(some(1), rx.next().await);
    assert_eq!(some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

// The provided Rust code snippet demonstrates the usage of asynchronous streams and channels. In this example, a sender (tx) and receiver (rx) are created using the mpsc::channel function. The sender sends two integers (1 and 2) to the receiver asynchronously using the send method. After sending the integers, the sender is dropped, indicating that no more values will be sent.

// The receiver then uses the StreamExt::next method to retrieve the values from the stream. The next method returns a Future that resolves to an Option<T>. By using the await keyword, the code waits for the next value to be available and then asserts that the received values match the expected ones.