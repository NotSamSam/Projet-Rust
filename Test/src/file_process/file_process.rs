use std::fs::File;
use std::io::{self, Read};
use crate::basic_methods::minhash::minhash;
use crate::basic_methods::k_grams::k_grams;

pub fn read_file(file : &str)->Result<String, std::io::Error>
{
    let mut text = File::open(file)?;
    let mut content = String::new();
    text.read_to_string(&mut content)?;
    return Ok(content);

}

pub fn return_file(file1 : &str, file2 : &str) -> bool
{
    let mut f1 : String = String::new();
    let mut f2 : String = String::new();

    match read_file(file1) 
    {
        Ok(contents) => f1 = contents,
        Err(e) => return false,
    }
    match read_file(file2)
    {
        Ok(contents) => f2 = contents,
        Err(e) => return false,
    }
    println!("DEBUG LA SIMILARITE EST : {}% D'APRES MINHASH", minhash(&f1, &f2)*100.0);
    println!("DEBUG LA SIMILARITE EST : {}% D'APRES KGRAMS", k_grams(&f1, &f2, 4, false));
    //println!("=== DEBUG : f1 = {} ",f1);
    //println!("=== DEBUG : f2 = {} ",f2);
    return true;
}
