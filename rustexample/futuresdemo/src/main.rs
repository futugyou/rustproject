use std::cell::RefCell;
thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));
mod task {
    use crate::NOTIFY;

    pub struct Context<'a> {
        waker: &'a Waker,
    }

    impl<'a> Context<'a> {
        pub fn from_waker(waker: &'a Waker) -> Self {
            Context { waker }
        }

        pub fn waker(&self) -> &'a Waker {
            &self.waker
        }
    }

    pub struct Waker;

    impl Waker {
        pub fn wake(&self) {
            NOTIFY.with(|f| *f.borrow_mut() = true)
        }
    }
}
use crate::task::*;

mod future {
    use crate::task::*;

    pub trait TryFuture {
        type Ok;
        type Error;
        fn try_poll(&mut self, ctx: &Context) -> Poll<Result<Self::Ok, Self::Error>>;
        fn and_then<Fut, F>(self, f: F) -> AndThen<Self, F>
        where
            F: FnOnce(Self::Ok) -> Fut,
            Fut: Future,
            Self: Sized,
        {
            AndThen {
                future: self,
                f: Some(f),
            }
        }
        fn map_err<E, F>(self, f: F) -> MapErr<Self, F>
        where
            F: FnOnce(Self::Error) -> E,
            Self: Sized,
        {
            MapErr {
                future: self,
                f: Some(f),
            }
        }
    }
    pub struct MapErr<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, F, E> Future for MapErr<Fut, F>
    where
        Fut: TryFuture,
        F: FnOnce(Fut::Error) -> E,
    {
        type Output = Result<Fut::Ok, E>;
        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(ctx) {
                Poll::Ready(result) => {
                    let f = self.f.take().unwrap();
                    Poll::Ready(result.map_err(f))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
    pub struct AndThen<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, NextFut, F> Future for AndThen<Fut, F>
    where
        Fut: TryFuture,
        NextFut: TryFuture<Error = Fut::Error>,
        F: FnOnce(Fut::Ok) -> NextFut,
    {
        type Output = Result<NextFut::Ok, Fut::Error>;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.try_poll(ctx) {
                Poll::Ready(Ok(val)) => {
                    let f = self.f.take().unwrap();
                    f(val).try_poll(ctx)
                }
                Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                Poll::Pending => Poll::Pending,
            }
        }
    }
    impl<F, T, E> TryFuture for F
    where
        F: Future<Output = Result<T, E>>,
    {
        type Ok = T;
        type Error = E;
        fn try_poll(&mut self, ctx: &Context) -> Poll<F::Output> {
            self.poll(ctx)
        }
    }
    pub enum Poll<T> {
        Ready(T),
        Pending,
    }

    pub trait Future {
        type Output;

        fn poll(&mut self, cx: &Context) -> Poll<Self::Output>;

        fn map<U, F>(self, f: F) -> Map<Self, F>
        where
            F: FnOnce(Self::Output) -> U,
            Self: Sized,
        {
            Map {
                future: self,
                f: Some(f),
            }
        }

        fn then<Fut, F>(self, f: F) -> Then<Self, F>
        where
            F: FnOnce(Self::Output) -> Fut,
            Fut: Future,
            Self: Sized,
        {
            Then {
                future: self,
                f: Some(f),
            }
        }
    }

    pub struct Map<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, F, T> Future for Map<Fut, F>
    where
        Fut: Future,
        F: FnOnce(Fut::Output) -> T,
    {
        type Output = T;

        fn poll(&mut self, ctx: &Context) -> Poll<T> {
            match self.future.poll(ctx) {
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap();
                    Poll::Ready(f(val))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    pub struct Then<Fut, F> {
        future: Fut,
        f: Option<F>,
    }

    impl<Fut, NextFut, F> Future for Then<Fut, F>
    where
        Fut: Future,
        NextFut: Future,
        F: FnOnce(Fut::Output) -> NextFut,
    {
        type Output = NextFut::Output;
        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
            match self.future.poll(ctx) {
                Poll::Ready(val) => {
                    let f = self.f.take().unwrap();
                    f(val).poll(ctx)
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
    pub struct Ready<T>(Option<T>);

    impl<T> Future for Ready<T> {
        type Output = T;

        fn poll(&mut self, _: &Context) -> Poll<Self::Output> {
            Poll::Ready(self.0.take().unwrap())
        }
    }

    pub fn ready<T>(val: T) -> Ready<T> {
        Ready(Some(val))
    }
}
use crate::future::*;

fn block_on<F>(mut f: F) -> F::Output
where
    F: Future,
{
    NOTIFY.with(|n| loop {
        if *n.borrow() {
            *n.borrow_mut() = false;
            let ctx = Context::from_waker(&Waker);
            if let Poll::Ready(val) = f.poll(&ctx) {
                return val;
            }
        }
    })
}

#[derive(Default)]
struct MyFuture {
    count: u32,
}

impl Future for MyFuture {
    type Output = i32;
    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.count {
            3 => Poll::Ready(3),
            _ => {
                self.count += 1;
                ctx.waker().wake();
                Poll::Pending
            }
        }
    }
}

struct AddOneFuture<T>(T);

impl<T> Future for AddOneFuture<T>
where
    T: Future,
    T::Output: std::ops::Add<i32, Output = i32>,
{
    type Output = i32;
    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.0.poll(ctx) {
            Poll::Ready(count) => Poll::Ready(count + 1),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn main() {
    // use futures::prelude::Future;
    // let future1 = future::ok::<u32, u32>(1)
    //     .map(|x| x + 3)
    //     .map_err(|e| println!("error :{:?}", e))
    //     .and_then(|x| Ok(x - 3))
    //     .then(|res| match res {
    //         Ok(val) => Ok(val + 3),
    //         err => err,
    //     });
    // let joined_future = future::join(future1, future::err::<u32, u32>(2));
    // let val = block_on(joined_future);
    // assert_eq!(val, (Ok(4), Err(2)));

    let my_future = future::ready(1)
        .map(|x| x + 1)
        .then(|x| future::ready(x + 1))
        .map(Ok::<i32, ()>)
        .and_then(|p| future::ready(Ok(p + 1)))
        .map_err(|_: ()| 5);
    //println!("output: {}", run(my_future));
    println!("output: {:?}", block_on(my_future));
}
