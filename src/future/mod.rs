use std::future::Future;
use std::pin::Pin;
use std::task::{ Context, Poll };
use std::time::{ Duration, Instant };
use tokio::time::sleep;

struct DelayedFuture {
    when: Instant,
}

impl Future for DelayedFuture {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready("Hello, world!")
        } else {
            // Register the waker to be notified when the future should be polled again
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_delayed_future() {
        let delay = Duration::from_millis(100);
        let when = Instant::now() + delay;
        let future: DelayedFuture = DelayedFuture { when };

        let result = future.await;
        assert_eq!(result, "Hello, world!");
    }
}
