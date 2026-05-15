use std::collections::HashSet;

fn clean_code(txt : &str)
{
    todo!()
}

pub fn k_grams(txt1: &str, txt2: &str, k: usize, space: bool, is_code : bool) -> f32 {
    let process = |t: &str| -> String {
        if space {
            t.to_lowercase()
        } else {
            t.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase()
        }
    };

    let t1 = process(txt1);
    let t2 = process(txt2);

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
