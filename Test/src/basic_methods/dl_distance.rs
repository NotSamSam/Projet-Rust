/* ici on implemente "Distance de Levenshtein" /!\ different de "Distance de Damerau-Levenshtein"
 * d'ap Wikipedia : le distance de DL est une distance entre deux chaines de caractères,
 * l'evaluation de cette distance consiste a eval le nombre d'operation minim pour transformer un
 * chaines string en une autre, ces operations peuvent etres : 
 * - insertion
 * - suppression
 * - substitution
 * - transposition
 * - permutation
 * Frederick J. Damerau a non seulement distingué ces quatre opérations d'édition, mais a aussi
 * écrit qu'elles correspondent à plus de 80 % des fautes d'orthographe humaines (Parfait dans le
 * cadre du projet)
 *
 * Aller voir : https://dev.to/jmegnidro/distance-de-levenshtein-le-guide-ultime-pour-mesurer-la-similarite-textuelle-3m7f
 */

pub fn mini(a : u32, b : u32, c : u32) -> u32
{
    let mut res = a;
    if b < res
    {
        res = b;
    }
    if c < res 
    {
        res = c;
    }
    return res;

}

pub fn l_distance(txt1 : &str, txt2 : &str) -> u32
{
    let l_1 = txt1.len();
    let l_2 = txt2.len();
    let mut m : Vec<Vec<u32>> = vec![vec![0; (l_2 + 1) as usize]; (l_1 + 1) as usize];
    println!("ETAPE 1 : Matrice M crée");

    let mut i = 0;
    while i < l_1 + 1
    {
        let mut j = 0;
        while j < l_2 + 1
        {
            if i == 0
            {m[i][j] = j as u32;}
            else if j == 0
            {m[i][j]=i as u32;}
            else if txt1.as_bytes()[i-1] == txt2.as_bytes()[j-1]
            {m[i][j]=m[i-1][j-1];}
            else
            {m[i][j] = 1 + mini(m[i-1][j], m[i][j-1], m[i-1][j-1]) as u32; }
            j+=1;
        } 
        i+=1;
    }
    println!("Fin de fonction.");
    return m[l_1][l_2];


}

pub fn l_distance(txt1 : &str, txt2 : &str) -> u32
{

}
//FIX : pareil que dans k-gram ici il faut traité les accent et caractere speciaux  
