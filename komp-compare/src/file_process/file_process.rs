use std::fs::File;
use std::io::Read;
//use crate::basic_methods::minhash::minhash;
use crate::basic_methods::k_grams::k_grams;
use crate::basic_methods::l_distance::l_distance;

pub fn read_file(file : &str)->Result<String, std::io::Error>
{
    let mut text = File::open(file)?;
    let mut content = String::new();
    text.read_to_string(&mut content)?;
    return Ok(content);

}

pub fn compare_file(file1 : &str, file2 : &str, space : bool, precision : usize) -> f32
{
    if precision > 9 || precision <= 0
    {
        println!("Please enter a precision value between 1 and 8.");
        return 0.0;
    }

    let mut f1 : String = String::new(); 
    let mut f2 : String = String::new(); 
    match read_file(file1) 
    { 
        Ok(contents) => f1.push_str(&contents), 
        Err(_) => { println!("Error reading document 1.");return 0.0;}, 
    } 
    match read_file(file2) 
    { 
        Ok(contents) => f2.push_str(&contents), 
        Err(_) => { println!("Error reading document 2.");return 0.0;}, 
    } 
    let res = k_grams(&f1, &f2, precision, space); 
    return res;
}

pub fn l_distance_file(file1 : &str, file2 : &str)->u32
{
    let mut f1 : String = String::new();
    let mut f2 : String = String::new();
    match read_file(file1)
    {
        Ok(contents) => f1.push_str(&contents),
        Err(_) => { println!("Error reading document 1.");return 0;},
    }
    match read_file(file2)
    {
        Ok(contents) => f2.push_str(&contents),
        Err(_) => { println!("Error reading document 2.");return 0;},
    }
    return l_distance(&f1, &f2);

}

/*pub fn find_occ(file1 : &str, list_file : Vec<&str>)
{
      
} */
