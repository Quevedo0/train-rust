trait Sonido {
    fn sonido2(&self) {
        println!("¡emitir sonido!");
    }
}

struct Gato;
struct Grillo;
struct Ballena;

impl Sonido for Gato {
    fn sonido2(&self) {
        println!("Miau");
    }
}

impl Sonido for Grillo {}
impl Sonido for Ballena {}

fn reproducir(animal: &impl Sonido) {
    animal.sonido2();
}

fn main() {
    let gato = Gato;
    let grillo = Grillo;
    let ballena = Ballena;

    println!("El sonido del gato:");
    reproducir(&gato);
    println!();

    println!("El sonido del grillo:");
    reproducir(&grillo);
    println!();

    println!("El sonido de la ballena:");
    reproducir(&ballena);
}

