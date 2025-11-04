// Luis Alejandro Aguilar Soberanes LICDD


trait Emitir {
    fn hacer_sonido(&self) {
        println!("Emitir Sonido");
    }
}

struct Gato;
struct Grillo;
struct Ballena;

impl Emitir for Gato {
    fn hacer_sonido(&self){
    println!("Miau");
    }
}
impl Emitir for Grillo {}
impl Emitir for Ballena {}

fn reproducir_sonido<T: Emitir>(animal: T) {
    animal.hacer_sonido();
}

fn main(){
    let gato = Gato;
    let grillo = Grillo;
    let ballena = Ballena;

    println!("Sonido grillo");
    reproducir_sonido(grillo);
    println!("Sonido gato");
    reproducir_sonido(gato);
    println!("Sonido ballena");
    reproducir_sonido(ballena);
}