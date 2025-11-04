//DIEGO SANTOS TREVIÑO CAMACHO

trait Sonidos{
    fn emite_sonido(&self);
}

struct Gato;
struct Grillo;
struct Ballena;

impl Sonidos for Gato{
    fn emite_sonido(&self){
    println!("Miau\n");
    }
}

impl Sonidos for Grillo{
    fn emite_sonido(&self){
    println!("Emite sonido\n");
    }
}

impl Sonidos for Ballena{
    fn emite_sonido(&self){
    println!("Emite sonido");
    }
}

fn poliformismo<T: Sonidos>(ruido: T){
    ruido.emite_sonido();
}

fn main() {

let gato = Gato;
let grillo = Grillo;
let ballena = Ballena;

   poliformismo(gato);
   poliformismo(grillo);
   poliformismo(ballena);

}