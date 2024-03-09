use crate::parse::parse_csv_file;
mod parse;

fn main() {
    parse_csv_file("input.csv", "output.csv");
}
