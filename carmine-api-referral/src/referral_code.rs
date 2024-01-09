use rand::Rng;

use crate::building_blocks::{ADJECTIVES, ANIMALS, COLORS};

pub fn generate_referral_code() -> String {
    let mut rng = rand::thread_rng();

    // Generate a random index and use it to get a string from the array
    let animal = ANIMALS[rng.gen_range(0..ANIMALS.len())];
    let color = COLORS[rng.gen_range(0..COLORS.len())];
    let adjective = ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())];

    let referral_code = format!("{}-{}-{}", adjective, color, animal);

    referral_code
}
