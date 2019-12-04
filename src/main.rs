mod ui_actor;
mod app_actor;

use actix::prelude::*;
use ui_actor::UiActor;
use app_actor::AppActor;

fn main() -> std::io::Result<()> {
    System::run(|| {
        let ui_arbiter = Arbiter::new();
        let app_arbiter = Arbiter::new();

//        let app_addr = SyncArbiter::start(3, || AppActor::new());

        let app_addr = AppActor::start_in_arbiter(
            &app_arbiter,
            move |_ctx: &mut Context<AppActor>| AppActor::new()
        );

        UiActor::start_in_arbiter(
            &ui_arbiter,
            move |_ctx: &mut Context<UiActor>| UiActor::new(app_addr)
        );
    })
}