use std::collections::HashSet;


fn clean_code_c(txt: &str) -> String {
    let mut slash_ouvert: bool = false;
    let mut double_slash: bool = false;
    let mut etoile: bool = false;
    let mut fin_interval: bool = false;

    let mut dans_balise: bool = false;

    let mut res: String = String::new();
    
    for c in txt.chars() {

        if c == '[' && !etoile && !double_slash {
            dans_balise = true;
            continue;
        }

        if dans_balise {
            if c == ']' {
                dans_balise = false;
            }
            continue; 
        }

        match c {
            '/' if !slash_ouvert && !etoile && !double_slash => {
                slash_ouvert = true;
            }
            '/' if slash_ouvert && !etoile && !double_slash => {
                double_slash = true;
                slash_ouvert = false;
            }
            '*' if slash_ouvert && !etoile && !double_slash => {
                etoile = true;
                slash_ouvert = false;
            }
            '*' if etoile => {
                fin_interval = true;
            }
            '/' if fin_interval && etoile => {
                fin_interval = false;
                etoile = false;
            }
            _ if etoile && fin_interval && c != '*' => {
                fin_interval = false;
            }
            '\n' if double_slash => {
                double_slash = false;
                res.push(c);
            }
            _ if !etoile && !double_slash => {
                if slash_ouvert {
                    res.push('/');
                    slash_ouvert = false;
                }
                res.push(c);
            }
            _ => {}
        }
    }

    if slash_ouvert {
        res.push('/');
    }

    res
}


pub fn k_grams(txt1: &str, txt2: &str, k: usize, space: bool, is_code : bool) -> f32 {

 
    let process = |t: &str| -> String {

        let cleaned = if is_code {
            clean_code_c(t)
        } 
        else {
            t.to_string()
        };

        if space {
            cleaned.to_lowercase()
        } else {
            cleaned.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase()
        }
    };

    let t1 = process(txt1);
    let t2 = process(txt2);
    println!("{}",t1);
     println!("=================================================");
      println!("{}",t2);


    let get_shingles = |text: &str| -> HashSet<String> {
        let chars: Vec<char> = text.chars().collect();
        let mut shingles = HashSet::new();
        if chars.len() >= k {
            for i in 0..=chars.len() - k {
                let s: String = chars[i..i + k].iter().collect();
                shingles.insert(s);
            }
        }
        shingles
    };

    let set1 = get_shingles(&t1);
    let set2 = get_shingles(&t2);

    if set1.is_empty() && set2.is_empty() {
        return 100.0;
    }

    let intersection = set1.intersection(&set2).count();
    let union = set1.len() + set2.len() - intersection;

    (intersection as f32 / union as f32) * 100.0
}


