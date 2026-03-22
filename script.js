// On récupère le bouton et la zone de texte
const btnMore = document.getElementById('btn-more');
const content = document.getElementById('cahier-des-charges');

// Quand on clique sur le bouton...
btnMore.addEventListener('click', () => {
    // Si le contenu est caché, on l'affiche. S'il est affiché, on le cache.
    content.classList.toggle('show');

    // On change le texte du bouton selon l'état
    if (content.classList.contains('show')) {
        btnMore.innerText = "Réduire le cahier des charges";
    } else {
        btnMore.innerText = "En savoir plus (Cahier des charges)";
    }
});