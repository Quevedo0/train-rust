 /* jose david sicairos fonseca */

trait SonidoDeAnimal {
    fn sonido_que_hace(&self) -> String;
}

struct Grillo;
struct Ballena;
struct Gato;

impl SonidoDeAnimal for Ballena {
    fn sonido_que_hace(&self) -> String {
        "phwargh eso dice".to_string()
    }
}

impl SonidoDeAnimal for Grillo {
    fn sonido_que_hace(&self) -> String {
        "cri-cri-cri".to_string()
    }
}

impl SonidoDeAnimal for Gato {
    fn sonido_que_hace(&self) -> String {
        "Miauuuuu, miauuuu".to_string()
    }
}

fn escuchar_sonido_animal<T: SonidoDeAnimal>(animal: &T) {
    println!("El animal hace {}", animal.sonido_que_hace());
}

fn main() {
    let grilloo = Grillo;
    let gato = Gato;
    let balle = Ballena;

    escuchar_sonido_animal(&grilloo);
    escuchar_sonido_animal(&gato);
    escuchar_sonido_animal(&balle);
}
