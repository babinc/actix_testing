use actix::{Actor, Context, Handler, Message};
use std::thread;
use std::time::Duration;

pub enum Error {}

pub enum Request {
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

    fn long_executing_stuff(&self) {
        thread::sleep(Duration::from_secs(3));
    }
}

impl Actor for AppActor{
    type Context = Context<Self>;
}

impl Handler<Request> for AppActor {
    type Result = Result<Response, Error>;

    fn handle(&mut self, msg: Request, _: &mut Context<Self>) -> Self::Result {
        match msg {
            Request::SetValue(val) => {
                self.value = val;
                self.long_executing_stuff();
                Ok(Response::Empty)
            },
            Request::GetValue => {
                Ok(Response::Value(self.value))
            }
        }
    }
}
