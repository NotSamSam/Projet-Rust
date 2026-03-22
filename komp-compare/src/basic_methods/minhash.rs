use probabilistic_collections::similarity::{MinHash, ShingleIterator};
use probabilistic_collections::SipHasherBuilder;

pub fn minhash(txt1 : &str, txt2 : &str) -> f64
{
    let min_hash = MinHash::with_hashers(
    100,
    [SipHasherBuilder::from_seed(0, 0), SipHasherBuilder::from_seed(1, 1)],
    );


    let tokens1: Vec<&str> = txt1.split_whitespace().collect();
    let tokens2: Vec<&str> = txt2.split_whitespace().collect();

    return min_hash.get_similarity(
        ShingleIterator::new(5, tokens1),
        ShingleIterator::new(5, tokens2),);
} 
