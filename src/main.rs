mod ui_actor;
mod app_actor;

use actix::prelude::*;
use ui_actor::UiActor;
use app_actor::AppActor;
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

fn main() -> std::io::Result<()> {
//    let mut cmd = Command::new("/Users/carmanbabin/projects/stock_trader_node/scanner/test.js")
//        .stdout(Stdio::piped())
//        .spawn()
//        .unwrap();
//
//    let stdout = cmd.stdout.as_mut().unwrap();
//    let stdout_reader = BufReader::new(stdout);
//    let stdout_lines = stdout_reader.lines();
//
//    for line in stdout_lines {
//        println!("Read: {}", line.unwrap());
//    }
//
//    cmd.wait().unwrap();
//
//    Ok(())

    System::run(|| {
        let ui_arbiter = Arbiter::new();
        let app_arbiter = Arbiter::new();

        let app_addr = AppActor::start_in_arbiter(
            &app_arbiter,
            move |_ctx: &mut Context<AppActor>| AppActor { value: 0 }
        );

        UiActor::start_in_arbiter(
            &ui_arbiter,
            move |_ctx: &mut Context<UiActor>| UiActor { count: 0, app_addr, }
        );
    })
}