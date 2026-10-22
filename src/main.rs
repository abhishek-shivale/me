mod utils;

fn main() {
    let re = utils::resume::Resume::parse();
    println!("{:?}", re);
}