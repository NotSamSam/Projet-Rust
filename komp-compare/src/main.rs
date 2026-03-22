use komp_compare::basic_methods::minhash::find_similar_docs;
use std::fs;
use std::collections::{HashMap, HashSet};


pub fn main()
{
    let mut documents: Vec<String> = Vec::new();
    for i in 1..=50 
    {
        let filename = format!("example_stack/doc{}.txt", i);
        let content = fs::read_to_string(filename).expect("Impossible de lire le fichier");
        documents.push(content);
    }
    let k = 5;             
    let num_hashes = 100;  
    let bands = 20;        
    let threshold = 0.8;

    let similar_docs: HashMap<usize, HashSet<usize>> =
    find_similar_docs(&documents, k, threshold, bands, num_hashes);

     for (doc, similars) in similar_docs.iter() 
     {
        println!("Document {} est similaire avec {:?}", doc, similars);
    }
}
