use komp-compare::basic_methods::k_grams::k_grams;

#[test]
pub fn tst()
{
    println!("La similarité entre les docs 1 et 2 est de {}",  k_grams("example1.txt","example2.txt", 3, false));
    println!("La similarité entre les docs 1 et 2 est de {}",  k_grams("example1.txt","example2.txt", 4, false));
    println!("La similarité entre les docs 1 et 2 est de {}",  k_grams("example1.txt","example2.txt", 5, false));
    println!("La similarité entre les docs 1 et 2 est de {}",  k_grams("example1.txt","example2.txt", 6, false));
    println!("La similarité entre les docs 1 et 2 est de {}",  k_grams("example1.txt","example2.txt", 7, false));
    println!("La similarité entre les docs 1 et 2 est de {}",  k_grams("example1.txt","example2.txt", 8, false));
}
