use Test::basic_methods::k_grams::k_grams;

#[test]
fn test_k_grams()
{
    let txt1 : &str = "je m'appelle ayoub";
    let txt2 : &str = "je m'appelze youba";
    
     k_grams(txt1, txt2, 3);

}
