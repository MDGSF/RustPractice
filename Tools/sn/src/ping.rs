use actix::prelude::*;

struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[actix::main]
async fn main() {
    let addr = MyActor { count: 5 }.start();

    let res = addr.send(Ping(10)).await;

    println!("result: {}", res.unwrap());

    System::current().stop();
}
