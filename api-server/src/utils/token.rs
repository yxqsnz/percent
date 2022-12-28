use rand::{thread_rng, Rng, SeedableRng};
use rand_hc::Hc128Rng;

pub fn generate() -> String {
    let mut rng = Hc128Rng::from_rng(thread_rng()).unwrap();
    let mut target = [0u8; 32];
    rng.fill(&mut target);
    base64::encode(&target)
}
