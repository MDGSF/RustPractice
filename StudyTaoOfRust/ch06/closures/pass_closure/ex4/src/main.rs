trait Any {
    fn any(&self, f: &(dyn Fn(u32) -> bool)) -> bool;
}

impl Any for Vec<u32> {
    fn any(&self, f: &(dyn Fn(u32) -> bool)) -> bool {
        for &x in self.iter() {
            if f(x) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let v = vec![1, 2, 3];
    let b = v.any(&|x| x == 3);
    println!("{:?}", b);
}
