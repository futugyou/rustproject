use actix::{dev::MessageResponse, prelude::*};

#[derive(Message)]
#[rtype(result = "Responses")]
enum Messages {
    Ping,
    Pong,
}

enum Responses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle<R: dev::ResponseChannel<M>>(self, _ctx: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("actor is stopped");
    }
}

impl Handler<Messages> for MyActor {
    type Result = Responses;
    fn handle(&mut self, msg: Messages, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Messages::Ping => Responses::GotPing,
            Messages::Pong => Responses::GotPong,
        }
    }
}

#[actix_rt::main]
async fn main() {
    let addr = MyActor.start();
    let ping_future = addr.send(Messages::Ping).await;
    let pong_future = addr.send(Messages::Pong).await;

    match ping_future {
        Ok(res) => match res {
            Responses::GotPing => println!("ping received"),
            Responses::GotPong => println!("pong received"),
        },
        Err(err) => println!("got error: {}", err),
    }
    match pong_future {
        Ok(res) => match res {
            Responses::GotPing => println!("ping received"),
            Responses::GotPong => println!("pong received"),
        },
        Err(err) => println!("got error: {}", err),
    }
}
