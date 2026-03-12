//use testo::basic_methods::k_grams::k_grams;
//use testo::basic_methods::l_distance::l_distance;
//use testo::basic_methods::minhash::minhash;
//use testo::file_process::file_process::read_file;
use testo::file_process::file_process::return_file;
#[test]
fn tst()
{
    /*let t1 : &str = "On ne voit bien qu'avec le coeur. L'essentiel est invisible pour les yeux. Il faut le temps que l'on perde pour bien connaitre les choses.";
    let t2 : &str = "On ne voit bien qu'avec le coeur. L'essentiel reste invisible aux yeux. Il faut du temps pour bien connaitre les choses.";
    println!("==== Debut K-grams ====");
    let pourc = k_grams(t1,t2,5,false);
    println!("Les chaines se ressembles à {}%",pourc);
    println!("==== fin K-grams ====");


    println!("");
    println!("");
    println!("");
    println!("");


    let ti1 : &str = "neural networks are widely used for image recognition natural language processing and complex pattern detection";
    let ti2 : &str = "neural networks are commonly used for image recognition natural language processing and complex pattern detection";
    println!("==== Debut distance DL ====");
    let dist = dl_distance(ti1, ti2);
    println!("La distance est : {}", dist);
    println!("==== fin distance DL ====");*/

/*    let e1 : &str = "the cat sat on a mat";
    let e2 : &str = "the cat sat on the mat";
    let res = minhash(e1,e2);
    println!("Le Resulta ===> {}",res);
*/

    
   return_file("example1.txt","example2.txt");


}
