extern crate wasmchat;
extern crate web_logger;
extern crate yew;

use wasmchat::{services::PubnubService, Model};
use yew::prelude::*;

pub struct Context {
    pubnub: PubnubService,
}

impl AsMut<PubnubService> for Context {
    fn as_mut(&mut self) -> &mut PubnubService {
        &mut self.pubnub
    }
}

fn main() {
    web_logger::init();
    yew::initialize();

    let context = Context {
        pubnub: PubnubService::new("pub-c-f1c4eb70-1624-499f-b87e-91a19560ca05",
                                   "sub-c-53de4390-cf6f-11eb-b6ed-fa77d5b6609d"),
    };

    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
