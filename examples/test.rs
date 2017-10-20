extern crate rust_wordnik;

fn main() {
    for definition in  rust_wordnik::get_definitions("house","enter you apikey here") {
        println!("{:?}", definition);
    }
}
