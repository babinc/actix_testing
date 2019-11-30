mod ui_actor;
mod app_actor;

use actix::prelude::*;
use futures::Future;
use std::thread;
use std::time::Duration;
use ui_actor::UiActor;
use app_actor::AppActor;

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

fn main() -> std::io::Result<()> {
    // start system, this is required step
    System::run(|| {
        let ui_arbiter = Arbiter::new();
        let app_arbiter = Arbiter::new();

        let ui_addr = Supervisor::start_in_arbiter(&ui_arbiter, move |_| {
            UiActor { count: 10 }
        });

        let app_addr = Supervisor::start_in_arbiter(&app_arbiter, move |_| {
            AppActor {
                value: 10,
                ui_ref: ui_addr
            }
        });

        loop {
            let fut = app_addr.send(Ping(10));
            let resp = fut.wait();
            match resp {
                Ok(result) => println!("app result: {}", result),
                Err(e) => println!("app err: {}", e.to_string()),
            }
            thread::sleep(Duration::from_secs(1));
        }
    })
}