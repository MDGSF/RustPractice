pub struct Letter {
    text: String,
}

pub struct EmptyEnvelope {}

pub struct ClosedEnvelope {
    letter: Letter,
}

pub struct PickupLorryHandle {
    done: bool,
}

impl Letter {
    pub fn new(text: String) -> Self {
        Letter { text: text }
    }
}

impl EmptyEnvelope {
    pub fn wrap(self, letter: Letter) -> ClosedEnvelope {
        ClosedEnvelope { letter: letter }
    }
}

pub fn buy_prestamped_envelop() -> EmptyEnvelope {
    EmptyEnvelope {}
}

impl PickupLorryHandle {
    pub fn pickup(&mut self, envelop: ClosedEnvelope) {}
    pub fn done(self) {}
}

impl Drop for PickupLorryHandle {
    fn drop(&mut self) {
        println!("sent");
    }
}

pub fn order_pickup() -> PickupLorryHandle {
    PickupLorryHandle { done: false }
}

fn main() {
    let letter = Letter::new(String::from("Dear RustFest"));
    let envelop = buy_prestamped_envelop();
    let closed_envelop = envelop.wrap(letter);
    let mut lorry = order_pickup();
    lorry.pickup(closed_envelop)
}
