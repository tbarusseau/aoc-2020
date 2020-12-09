extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn define_main(attr: TokenStream, _item: TokenStream) -> TokenStream {
    let day = attr.to_string();
    let path = format!("./input/2020/day{}.txt", day);

    format!(
        r#"fn main() {{
        use std::path::Path;
        let path_to_input_file = Path::new("{}");
        let input = std::fs::read_to_string(path_to_input_file).expect("Couldn't read input");

        println!("Part 1: {{:?}}", solve_part1(&input));
        println!("Part 2: {{:?}}", solve_part2(&input));
    }}"#,
        path
    )
    .parse()
    .expect("Invalid main definition")
}
