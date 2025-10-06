use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use goofy_animals::generate_name;

fn main() {
    let mut rng = ChaCha20Rng::from_os_rng();

    println!("{}", generate_name(&mut rng));
}
