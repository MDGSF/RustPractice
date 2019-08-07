fn capitalize(data: &mut [char]) {}

fn main() {
    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..];
    capitalize(slice);
    data.push('d');
}
