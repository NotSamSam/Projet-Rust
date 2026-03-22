use komp_compare::file_process::file_process::compare_file;

pub fn main()
{
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 1));   
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 2));
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 3));
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 4));
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 5));
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 6));
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 7));
    println!("returned value : {}", compare_file("example1.txt", "example2.txt", false, 8));

//        println!("returned value : {}", l_distance_file("example1.txt", "example2.txt"));    

}
