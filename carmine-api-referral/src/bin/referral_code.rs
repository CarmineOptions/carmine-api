use carmine_api_referral::referral_code::generate_referral_code;

fn main() {
    for _ in 0..10 {
        let code = generate_referral_code();
        println!("{}", code);
    }
}
