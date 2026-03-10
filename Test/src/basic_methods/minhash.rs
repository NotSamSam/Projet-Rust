/*Ici on Implemente "minHash" d'apres Wikipedia : le Minhash est une technique permettant d'estimé
 * rapidement la similarité entre deux ensembles,  Cette méthode a été présentée par Andrei Broder
 * lors d'une conférence en 1997 et initialement utilisée dans le moteur de recherche
 * AltaVista pour détecter les pages web dupliquées et les supprimer des résultats de recherche. Elle
 * a également été appliquée à des problèmes de clustering à grande échelle , comme le
 * regroupement de documents selon la similarité de leurs ensembles de mots.
 *
 * 
 * ici on aura besoin de la Similarité de Jaccard (on le cala dans k-gram)
 * on nous dit que la SdJ vaut 0 si A et B sont disjoint et 1 si ils sont egaux, donc plus deux
 * ensembls sont similaire plus leurs SdJ est proche de 1. Donc le but de MinHash est d'estimé j(A,
 * B) sans pour autant cala leurs Unions et leurs Intersections et ce de maniere RAPIDE.
 *
 *
 */
use probabilistic_collections::similarity::{MinHash, ShingleIterator};
use probabilistic_collections::SipHasherBuilder;

pub fn minhash(txt1 : &str, txt2 : &str) -> f64
{
    let min_hash = MinHash::with_hashers(
    100,
    [SipHasherBuilder::from_seed(0, 0), SipHasherBuilder::from_seed(1, 1)],
    );


    let tokens1: Vec<&str> = txt1.split_whitespace().collect();
    let tokens2: Vec<&str> = txt2.split_whitespace().collect();

    return min_hash.get_similarity(
        ShingleIterator::new(2, tokens1),
        ShingleIterator::new(2, tokens2),);
} 
