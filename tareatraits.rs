
//Elias Cuevas Herrera
    trait Emitir {
        fn sonido(&self){
        println!("!emitir sonido¡");
    }
}
struct Gato;
struct Grillo;
struct Ballena;

impl Emitir for Gato {
    fn sonido(&self) {
        println!("Miau");
    }
}

impl Emitir for Grillo{}
impl Emitir for Ballena{}

fn reproducir<T: Emitir >(animal: T) {
    animal.sonido();
    
}

fn main(){
    let gato=Gato;
    let grillo=Grillo;
    let ballena=Ballena;
 
    println!("el sonido del gato: ");
    reproducir(gato);
    println!();

    println!("el sonido del grillo: ");
    reproducir(grillo);
    println!();

    println!("el sonido de la ballena: ");
    reproducir(ballena);
}