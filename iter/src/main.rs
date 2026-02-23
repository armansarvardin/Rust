fn print_elements(elements: &[String]) {
    elements
        .iter()
        .map(|element| format!("Color: {:#?}", element))
        .for_each(|element| println!("{:#?}", element));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|element| element.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|element| element.to_uppercase())
        .collect()
}

fn main() {
    let mut colors: Vec<String> = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow"),
        String::from("purple"),
    ];

    let uppercase_colors = to_uppercase(&colors);
    println!("{:#?}", uppercase_colors);
}
