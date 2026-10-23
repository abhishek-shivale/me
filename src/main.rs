mod utils;
mod pages;
mod components;

fn main() {
    let re = utils::resume::Resume::parse();
    println!("{:?}", re);
}