use actix::{Actor, Context, Handler, ActorStream, ContextFutureSpawner, Addr};
use crate::Ping;
use actix::utils::IntervalFunc;
use std::time::Duration;
use crate::app_actor::AppActor;
use futures::Future;

pub struct UiActor {
    pub count: usize,
    pub app_addr: Addr<AppActor>
}

impl UiActor {
    fn draw(&mut self, _: &mut Context<Self>) {
        let fut = self.app_addr.send(Ping(10));
        let resp = fut.wait();
        match resp {
            Ok(result) => println!("ui result: {}", result),
            Err(e) => println!("ui err: {}", e.to_string()),
        }
    }
}

impl Actor for UiActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        IntervalFunc::new(Duration::from_millis(1000), Self::draw)
            .finish()
            .spawn(ctx);
    }
}

impl Handler<Ping> for UiActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        println!("UI Got Message");
        self.count += msg.0;
        self.count
    }
}
