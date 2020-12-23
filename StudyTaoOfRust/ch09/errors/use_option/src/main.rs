fn get_shortest(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
        let mut shortest = names[0];
        for name in names.iter() {
            if name.len() < shortest.len() {
                shortest = *name;
            }
        }
        Some(shortest)
    } else {
        None
    }
}

fn show_shortest(names: Vec<&str>) -> &str {
    match get_shortest(names) {
        Some(shortest) => shortest,
        None => "Not Found",
    }
}

fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
    match get_shortest(names) {
        Some(shortest) => Some(shortest.len()),
        None => None,
    }
}

fn get_shortest_length_2(names: Vec<&str>) -> Option<usize> {
    get_shortest(names).map(|name| name.len())
}

fn main() {
    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");

    assert_eq!(get_shortest_length(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);

    assert_eq!(get_shortest_length_2(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length_2(Vec::new()), None);
}
