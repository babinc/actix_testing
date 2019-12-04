use actix::{Actor, Context, Handler, Message, AsyncContext};
use std::thread;
use std::time::Duration;

pub enum Error {}

pub enum Request {
    DoValue(i32),
    SetValue(i32),
    GetValue,
}

impl Message for Request {
    type Result = Result<Response, Error>;
}

pub enum Response {
    Empty,
    Value(i32),
}

pub struct AppActor {
    pub value: i32,
}

impl AppActor {
    pub fn new() -> AppActor {
        AppActor {
            value: 0
        }
    }
}

impl Actor for AppActor {
    type Context = Context<Self>;
}

impl Handler<Request> for AppActor {
    type Result = Result<Response, Error>;


    fn handle(&mut self, msg: Request, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Request::DoValue(val) => {
                let addr = ctx.address();
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(5000));
                    addr.do_send(Request::SetValue(val));
                });
                Ok(Response::Empty)
            },
            Request::SetValue(val) => {
                self.value = val;
                Ok(Response::Empty)
            },
            Request::GetValue => {
                self.value += 1;
                Ok(Response::Value(self.value))
            }
        }
    }
}
