/*ici on commence l'implentation de l'algo "k-grams" 
 * d'apres wikipedia : On se base sur la decoupe d'une chaine en sous ensemble de k elements puis
 * on rassemble tous dans une liste que l'on reduit au max en supprimant les k-uplet identiques
 *
 * La ressemblance est calculé pour un nombre de barceaux donnée pour un doc A et un doc B :
 * ressemblance(A, B) = (l'intersection de leurs Bardeau)/(l'union de leurs bardeaux),
 * grossierement on divise les k-grams en commun par le nombre TOTAL de k-grams c sensé donné un
 * pourcentage/100
 * ps : smehli pour les fautes wolah
 */

pub fn k_grams(txt1 : &str,txt2 : &str, k : usize, space : bool) -> f32
{
    let mut txt1 = txt1.to_string();
    let mut txt2 = txt2.to_string();

    if space == false //Space = prenons nous en compte les espaces ?
    {
        txt1 = txt1.replace(" ", "");
        txt2 = txt2.replace(" ", "");
        println!("Suppression des espaces");
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
    println!("SUCCES");
    println!("Union = {} et Inter = {}", total, same_values);
    return ((same_values as f32)/total)*100.0;
    
}

//FIX : les slices decoute uniquement par byte donc compliqué pour les accents
