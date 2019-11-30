mod ui_actor;
mod app_actor;

use actix::prelude::*;
use futures::Future;
use std::thread;
use std::time::Duration;
use ui_actor::UiActor;
use app_actor::AppActor;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

fn main() -> std::io::Result<()> {
//    let stdout = Command::new("/Users/carmanbabin/projects/stock_trader_node/scanner/test.js")

    let mut cmd = Command::new("/Users/carmanbabin/projects/stock_trader_node/scanner/test.js")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }

    cmd.wait().unwrap();

    Ok(())

    // start system, this is required step
//    System::run(|| {
//        let ui_arbiter = Arbiter::new();
//        let app_arbiter = Arbiter::new();
//
//        let ui_addr = Supervisor::start_in_arbiter(&ui_arbiter, move |_| {
//            UiActor { count: 10 }
//        });
//
//        let app_addr = Supervisor::start_in_arbiter(&app_arbiter, move |_| {
//            AppActor {
//                value: 10,
//                ui_ref: ui_addr
//            }
//        });
//
//        loop {
//            let fut = app_addr.send(Ping(10));
//            let resp = fut.wait();
//            match resp {
//                Ok(result) => println!("app result: {}", result),
//                Err(e) => println!("app err: {}", e.to_string()),
//            }
//            thread::sleep(Duration::from_secs(1));
//        }
//    })
}