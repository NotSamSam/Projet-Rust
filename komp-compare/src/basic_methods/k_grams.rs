pub fn k_grams(txt1 : &str,txt2 : &str, k : usize, space : bool) -> f32
{
    let mut txt1 = txt1.to_string();
    let mut txt2 = txt2.to_string();

    if space == false 
    {
        txt1 = txt1.replace(" ", "");
        txt2 = txt2.replace(" ", "");
    } 

    let mut shingle_list_1 : Vec<&str> = Vec::new();
    let mut shingle_list_2 : Vec<&str> = Vec::new();

    let len_txt_1 = txt1.chars().count();
    let len_txt_2 = txt2.chars().count();

    let mut i = 0;
    let mut j = 0;

    while i <= len_txt_1 - k 
    {
         shingle_list_1.push(&txt1[i..=i+k-1]);
         i+=1;
    }
    while j <= len_txt_2 - k 
    {
         shingle_list_2.push(&txt2[j..=j+k-1]);
         j+=1;
    }
    let mut same_values = 0;
    for i in shingle_list_1.iter()
    {
        let no_local_i = i;
        for j in shingle_list_2.iter()
        {
            if no_local_i == j
            {
                same_values += 1;
                break;
            }
        }
    }
    let total : f32 = (shingle_list_1.len() as f32 + shingle_list_2.len() as f32) - same_values as f32;
    return ((same_values as f32)/total)*100.0;
}
