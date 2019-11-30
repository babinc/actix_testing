use actix::{Actor, Context, Handler};
use crate::Ping;

pub struct UiActor {
    pub count: usize,
}

impl Actor for UiActor {
    type Context = Context<Self>;
}

impl actix::Supervised for UiActor {
    fn restarting(&mut self, _ctx: &mut Context<Self>) {
        println!("Restarting UI Actor")
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

