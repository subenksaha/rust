
#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}
// Dangling pointers are not allowed
pub fn dropIt(){
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);
    println!("{:?}", grains)
}