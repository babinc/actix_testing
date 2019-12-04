use actix::{Actor, Context, ActorStream, ContextFutureSpawner, Addr};
use actix::utils::IntervalFunc;
use std::time::Duration;
use crate::app_actor::{AppActor, Request, Response};
use futures::Future;

pub struct UiActor {
    count: i32,
    pub app_addr: Addr<AppActor>
}

impl UiActor {
    pub fn new(app_addr: Addr<AppActor>) -> UiActor {
        UiActor {
            count: 0,
            app_addr
        }
    }

    fn draw(&mut self, _: &mut Context<Self>) {
        if self.count == 2 {
            self.app_addr.do_send(Request::SetValue(20));
        }

        let fut = self.app_addr.send(Request::GetValue);
        let resp = fut.wait().unwrap().unwrap_or(Response::Empty);
        if let Response::Value(res) = resp {
            println!("Value: {}", res);
        }

        self.count += 1;
    }
}

impl Actor for UiActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        IntervalFunc::new(Duration::from_millis(300), Self::draw)
            .finish()
            .spawn(ctx);
    }
}
