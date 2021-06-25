use std::usize;

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct OrderShipped(usize);

#[derive(Message)]
#[rtype(result = "()")]
struct Ship(usize);

#[derive(Message)]
#[rtype(result = "()")]
struct Subscribe(pub Recipient<OrderShipped>);

struct OrderEvents {
    subscribers: Vec<Recipient<OrderShipped>>,
}

impl OrderEvents {
    fn new() -> Self {
        OrderEvents {
            subscribers: vec![],
        }
    }

    fn notify(&mut self, order_id: usize) {
        for subscr in &self.subscribers {
            subscr.do_send(OrderShipped(order_id)).unwrap();
        }
    }
}

impl Handler<Subscribe> for OrderEvents {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _ctx: &mut Self::Context) -> Self::Result {
        self.subscribers.push(msg.0);
    }
}

impl Handler<Ship> for OrderEvents {
    type Result = ();

    fn handle(&mut self, msg: Ship, _ctx: &mut Self::Context) -> Self::Result {
        self.notify(msg.0);
        System::current().stop();
    }
}

impl Actor for OrderEvents {
    type Context = Context<Self>;
}

struct EmailSubscribe;
impl Actor for EmailSubscribe {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for EmailSubscribe {
    type Result = ();

    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("emial send for order :{}", msg.0);
    }
}

struct SMSSubscribe;
impl Actor for SMSSubscribe {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for SMSSubscribe {
    type Result = ();

    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("sms send for order :{}", msg.0);
    }
}

fn main() {
    let system = System::new("events");
    let emial_subscribe = Subscribe(EmailSubscribe {}.start().recipient());
    let sms_subscribe = Subscribe(SMSSubscribe {}.start().recipient());
    let order_events = OrderEvents::new().start();
    order_events.do_send(emial_subscribe);
    order_events.do_send(sms_subscribe);
    order_events.do_send(Ship(1));
    system.run().unwrap();
}
