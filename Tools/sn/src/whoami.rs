use actix::prelude::*;

struct WhoAmI;

impl Message for WhoAmI {
    type Result = Result<actix::Addr<MyActor>, ()>;
}

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<WhoAmI> for MyActor {
    type Result = Result<actix::Addr<MyActor>, ()>;

    fn handle(&mut self, _msg: WhoAmI, ctx: &mut Context<Self>) -> Self::Result {
        Ok(ctx.address())
    }
}

#[actix::main]
async fn main() {
    let addr = MyActor { }.start();
    let who_addr = addr.send(WhoAmI{}).await.unwrap().unwrap();
    assert_eq!(addr, who_addr);
}
