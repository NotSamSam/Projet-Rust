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
    return m[l_1][l_2];
}
