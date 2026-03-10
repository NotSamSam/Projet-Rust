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
 * putn c harr
