use rand::Rng;

pub fn gen_random_color() -> String {
    let mut rng = rand::thread_rng();
    let colors = "0123456789abcdef";
    let r = colors.as_bytes()[rng.gen_range(0..colors.len())];
    let g = colors.as_bytes()[rng.gen_range(0..colors.len())];
    let b = colors.as_bytes()[rng.gen_range(0..colors.len())];
    format!("#{}{}{}", r, g, b)
}
