use enum_all_variants::AllVariants;

#[derive(AllVariants, Debug)]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

fn main() {
    println!("{:?}", Direction::all_variants());
}
