use std::any::Any;

trait HouseElement {
    fn accept(&self, visitor: &dyn HouseElementVisitor);
    fn as_any(&self) -> &dyn Any;
}

trait HouseElementVisitor {
    fn visit(&self, element: &dyn HouseElement);
}

struct House {
    components: Vec<Box<dyn HouseElement>>,
}

impl House {
    fn new() -> Self {
        House {
            components: vec![Box::new(LivingRoom::new())],
        }
    }
}

impl HouseElement for House {
    fn accept(&self, visitor: &dyn HouseElementVisitor) {
        for component in self.components.iter() {
            component.accept(visitor);
        }
        visitor.visit(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct LivingRoom;

impl LivingRoom {
    fn new() -> Self {
        LivingRoom
    }
}

impl HouseElement for LivingRoom {
    fn accept(&self, visitor: &dyn HouseElementVisitor) {
        visitor.visit(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct HouseElementListVisitor;
impl HouseElementListVisitor {
    fn new() -> Self {
        HouseElementListVisitor
    }
}

impl HouseElementVisitor for HouseElementListVisitor {
    fn visit(&self, element: &dyn HouseElement) {
        match element.as_any() {
            house if house.is::<House>() => println!("Visiting the house..."),
            living if living.is::<LivingRoom>() => println!("Visiting the Living room..."),
            _ => {}
        }
    }
}

struct HouseElementDemolishVisitor;
impl HouseElementDemolishVisitor {
    fn new() -> Self {
        HouseElementDemolishVisitor
    }
}

impl HouseElementVisitor for HouseElementDemolishVisitor {
    fn visit(&self, element: &dyn HouseElement) {
        match element.as_any() {
            house if house.is::<House>() => println!("annihilating the house...!!!"),
            living if living.is::<LivingRoom>() => println!("Bombing the Living room...!!!"),
            _ => {}
        }
    }
}

fn main() {
    let house = House::new();
    house.accept(&HouseElementListVisitor::new());
    house.accept(&HouseElementDemolishVisitor::new());
}
