use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};

fn containss(occ : String , lst :&Vec<String>)->bool
{
    for c in lst
    {
        if occ == *c
        {
            return true;
        }
    }
    return false;
}

pub fn k_grams_only(txt1 : &str, k : usize, space : bool) -> Vec<String>
{
    let mut txt1 = txt1.to_string();
    if space == false
    {
        txt1 = txt1.replace(" ", "");
    }

    let mut shingle_list_1 : Vec<String> = Vec::new();
    let len_txt_1 = txt1.chars().count();

    let txt1 = txt1.to_lowercase();
    for i in 0..=len_txt_1 - k
    {
        let s: String = txt1.chars().skip(i).take(k).collect();
        if !containss( s.clone(), &shingle_list_1)
        {
             shingle_list_1.push(s);
        }
    }
    return shingle_list_1;
}

pub fn minhash_signature(shingles: &[String], num_hashes: usize) -> Vec<u64>
{
    let mut signature = vec![u64::MAX; num_hashes];

    for (i, slot) in signature.iter_mut().enumerate()
    {
        for shingle in shingles.iter()
        {

            let mut hasher = DefaultHasher::new();

            hasher.write_u64(i as u64);
            shingle.hash(&mut hasher);
            let h = hasher.finish();
            if h < *slot
            {
                *slot = h;
            }
        }
    }

    return signature;
}

pub fn lsh_index(signatures: &[Vec<u64>], bands: usize) -> HashMap<String, Vec<usize>> 
{
    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    let rows_per_band = signatures[0].len() / bands;

    for (doc_id, sig) in signatures.iter().enumerate() 
    {
        for b in 0..bands 
        {
            let start = b * rows_per_band;
            let end = start + rows_per_band;
            let band_hash = format!("{:?}", &sig[start..end]);
            index.entry(band_hash).or_default().push(doc_id);
        }
    }

    return index;
}
pub fn find_similar_docs(
    documents: &[String],
    k: usize,
    threshold: f64,
    bands: usize,
    num_hashes: usize
) -> HashMap<usize, HashSet<usize>> 
{
    
    let signatures: Vec<Vec<u64>> = documents
        .iter()
        .map(|doc| minhash_signature(&k_grams_only(doc, k, false), num_hashes))
        .collect();

    
    let lsh = lsh_index(&signatures, bands);
    let mut results: HashMap<usize, HashSet<usize>> = HashMap::new();

    
    for candidate_list in lsh.values() {
        for (i, &doc1) in candidate_list.iter().enumerate() {
            for &doc2 in candidate_list.iter().skip(i + 1) {
                let sig1 = &signatures[doc1];
                let sig2 = &signatures[doc2];

                let intersection = sig1.iter()
                    .zip(sig2.iter())
                    .filter(|(a, b)| a == b)
                    .count();
                let sim = intersection as f64 / sig1.len() as f64;

                if sim >= threshold {
                    results.entry(doc1).or_default().insert(doc2);
                    results.entry(doc2).or_default().insert(doc1);
                }
            }
        }
    }

   return results;
}
