trait Sonido {
    fn hacer_sonido(&self) -> String {
        "hacer sonido".to_string()
    }
}

struct Grillo;
struct Gato;
struct Ballena;

impl Sonido for Grillo {
    fn hacer_sonido(&self) -> String {
        "¡Cri Cri!".to_string()
    }
}

impl Sonido for Gato {
    fn hacer_sonido(&self) -> String {
        "¡Miau!".to_string()
    }
}

impl Sonido for Ballena {}

fn imprimir_sonido(animal: &impl Sonido) {
    println!("{}", animal.hacer_sonido());
}

fn main() {
    let grillo = Grillo;
    let gato = Gato;
    let ballena = Ballena;
    
    imprimir_sonido(&grillo);
    imprimir_sonido(&gato);
    imprimir_sonido(&ballena);
}