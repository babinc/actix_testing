use actix::{Actor, Context, Handler, Message};
use crate::Ping;

pub enum Request {
    Test,
}

impl Message for Request {
    type Result = i32;
}

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

impl Handler<Request> for AppActor {
    type Result = i32;

    fn handle(&mut self, msg: Request, _: &mut Context<Self>) -> Self::Result {
        match msg {
            Request::Test => 32
        }
    }
}
