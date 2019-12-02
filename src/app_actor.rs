use actix::{Actor, Context, Handler};
use crate::Ping;

pub struct AppActor {
    pub value: usize,
}

impl Actor for AppActor{
    type Context = Context<Self>;
}

impl Handler<Ping> for AppActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        println!("App Got Message");
//        let fut = self.ui_ref.send(Ping(10));
//        let resp = fut.wait();
//        match resp {
//            Ok(result) => println!("result: {}", result),
//            Err(e) => println!("err: {}", e.to_string()),
//        }

        self.value += msg.0;
        self.value
    }
}
