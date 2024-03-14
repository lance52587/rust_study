// use art::kinds::PrimaryColor;
// use art::utils::mix;
use cratesio2::PrimaryColor;
use cratesio2::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
