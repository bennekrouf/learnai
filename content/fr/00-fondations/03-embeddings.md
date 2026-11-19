---
id: "00-03"
titre: "Embeddings"
sous_rubriques:
  - id: vecteur
    titre: "Un texte devient un vecteur"
  - id: cosinus
    titre: "La similarité cosinus"
  - id: limites
    titre: "Ce qu'un embedding capte, et ce qu'il rate"
sources:
  - titre: "Mikolov et al. — Efficient Estimation of Word Representations in Vector Space (word2vec), 2013"
    url: "https://arxiv.org/abs/1301.3781"
  - titre: "Nissim, van Noord, van der Goot — Fair is Better than Sensational: Man is to Doctor as Woman is to Doctor, Computational Linguistics, 2020"
    url: "https://arxiv.org/abs/1905.09866"
qcm:
  - id: vecteur-1
    sous_rubrique: vecteur
    question: "Un embedding, c'est…"
    choix:
      - "Une liste de nombres de taille fixe qui représente le sens"
      - "Un résumé du texte"
      - "La liste des tokens du texte"
      - "Un index de mots-clés"
    reponse: 0
    pourquoi: >-
      Un modèle d'embedding lit un texte.
      Il sort une liste de nombres, toujours de la même longueur.
      Trois mots ou trois paragraphes : même taille.
      Des textes de sens proche donnent des listes proches.

  - id: vecteur-2
    sous_rubrique: vecteur
    question: "Tu changes de modèle d'embedding. Que faire des vecteurs déjà stockés ?"
    choix:
      - "Rien, ils restent compatibles"
      - "Recalculer seulement les nouveaux documents"
      - "Les recalculer tous"
      - "Les normaliser"
    reponse: 2
    pourquoi: >-
      Chaque modèle a son propre espace.
      Les nombres d'un modèle n'ont aucun sens pour un autre.
      Mélanger les deux donne des résultats aléatoires.
      Changer de modèle, c'est réindexer tout le corpus.

  - id: vecteur-3
    sous_rubrique: vecteur
    question: "La question et les documents doivent être encodés…"
    choix:
      - "Avec deux modèles différents, pour la diversité"
      - "Avec le même modèle d'embedding"
      - "La question par le LLM, les documents par l'embedding"
      - "Peu importe"
    reponse: 1
    pourquoi: >-
      On compare des positions dans un même espace.
      Deux modèles, deux espaces.
      La comparaison n'aurait aucun sens.

  - id: cosinus-1
    sous_rubrique: cosinus
    question: "La similarité cosinus mesure…"
    choix:
      - "La distance en nombre de mots"
      - "La longueur des vecteurs"
      - "L'angle entre deux vecteurs"
      - "Le nombre de tokens communs"
    reponse: 2
    pourquoi: >-
      Elle regarde si deux vecteurs pointent dans la même direction.
      Même direction : 1. Perpendiculaires : 0.
      La longueur du vecteur est ignorée.
      Seule compte la direction, c'est-à-dire le sens.

  - id: cosinus-2
    sous_rubrique: cosinus
    question: "Vecteurs normalisés (longueur 1). La similarité cosinus est égale à…"
    choix:
      - "Le produit scalaire"
      - "La distance euclidienne"
      - "Zéro"
      - "La somme des composantes"
    reponse: 0
    pourquoi: >-
      Le cosinus, c'est le produit scalaire divisé par les longueurs.
      Si les longueurs valent 1, il ne reste que le produit scalaire.
      C'est plus rapide à calculer.
      Beaucoup de bases vectorielles normalisent pour cette raison.

  - id: cosinus-3
    sous_rubrique: cosinus
    question: "Un document obtient un score de 0,82. Est-il pertinent ?"
    choix:
      - "Oui, au-dessus de 0,8 c'est toujours pertinent"
      - "Oui, tout score positif est pertinent"
      - "Non, il faut 0,95"
      - "Ça dépend du modèle et de tes données"
    reponse: 3
    pourquoi: >-
      Chaque modèle a sa propre échelle.
      Chez l'un, 0,82 est excellent. Chez l'autre, banal.
      Un seuil ne se copie pas d'un projet à l'autre.
      Il se calibre sur des exemples à toi.

  - id: limites-1
    sous_rubrique: limites
    question: "« J'aime ce film » et « Je n'aime pas ce film ». Leurs embeddings sont…"
    choix:
      - "Opposés"
      - "Très proches"
      - "Perpendiculaires"
      - "Impossibles à comparer"
    reponse: 1
    pourquoi: >-
      Les deux phrases parlent du même sujet, avec les mêmes mots.
      L'embedding capte surtout le thème.
      La négation pèse peu.
      Une recherche par embedding peut donc renvoyer l'avis contraire.

  - id: limites-2
    sous_rubrique: limites
    question: "Tu cherches la facture « INV-2024-0042 » par embedding. Le risque ?"
    choix:
      - "Trouver d'autres factures proches mais pas celle-là"
      - "Aucun"
      - "Une erreur de l'API"
      - "Trop de tokens"
    reponse: 0
    pourquoi: >-
      Un embedding capte le sens, pas les caractères exacts.
      Toutes les factures se ressemblent pour lui.
      Les identifiants, codes et noms rares se cherchent par mots-clés.
      D'où la recherche hybride de l'étape 2.

  - id: limites-3
    sous_rubrique: limites
    question: "« roi − homme + femme ≈ reine » (word2vec, 2013). Ce qu'on omet souvent ?"
    choix:
      - "Le résultat était « princesse »"
      - "Ça ne marchait qu'en anglais"
      - "Le code exclut les mots de la question des réponses possibles"
      - "Le calcul se faisait sur des tokens"
    reponse: 2
    pourquoi: >-
      Le code d'origine interdit de répondre roi, homme ou femme.
      Sans ce filtre, la réponse est souvent l'un des mots de départ.
      Des chercheurs l'ont montré en 2020.
      Belle démonstration, mais moins magique qu'annoncée.
---

## Un texte devient un vecteur {#vecteur}

Un **modèle d'embedding** transforme un texte en liste de nombres.

Cette liste a toujours la même taille, quel que soit le texte.

On l'appelle un **vecteur**. C'est une position dans un espace du sens.

:::pourquoi
Un ordinateur compare mal des phrases.
Il compare très bien des nombres.
Le vecteur rend le sens calculable.
:::

Deux textes de sens proche ont des vecteurs proches.

```mermaid
flowchart LR
  D["Documents"] --> E["Modèle d'embedding"]
  Q["Question"] --> E
  E --> V["Vecteurs"]
  V --> B["Base vectorielle"]
  B --> R["Documents les plus proches"]
```

:::pourquoi
Question et documents passent par **le même** modèle.
Chaque modèle a son propre espace. Deux espaces ne se comparent pas.
Changer de modèle oblige à tout recalculer.
:::

## La similarité cosinus {#cosinus}

Pour comparer deux vecteurs, on mesure l'angle entre eux.

Même direction : 1. Sans rapport : proche de 0.

:::pourquoi
La direction porte le sens.
La longueur dépend d'autres choses, comme la longueur du texte.
Le cosinus ignore la longueur.
:::

Si les vecteurs sont normalisés, le cosinus est un simple produit scalaire.

:::pourquoi
Moins de calcul, même résultat.
Sur des millions de documents, ça compte.
:::

Un score n'a pas de sens absolu.

:::pourquoi
Chaque modèle a son échelle.
Un seuil se calibre sur tes propres exemples.
:::

## Ce qu'un embedding capte, et ce qu'il rate {#limites}

Il capte bien le **thème**. Il rate souvent la **précision**.

Il capte mal :

- la négation : « j'aime » et « je n'aime pas » sont voisins ;
- les identifiants : numéros de facture, références, codes ;
- les noms rares et les chiffres exacts.

:::pourquoi
L'embedding résume un texte en une seule position.
Ce qui change peu le sujet change peu la position.
Une négation ou un chiffre changent peu le sujet.
:::

Pour ces cas, on combine avec une recherche par mots-clés. C'est l'étape 2.

Le célèbre « roi − homme + femme ≈ reine » de 2013 a une petite astuce. Le code exclut les mots de départ des réponses possibles.

:::pourquoi
Sans ce filtre, la réponse est souvent l'un des mots de départ.
Les embeddings ont une vraie structure. Mais les démos l'embellissent.
:::

## À retenir

- Un embedding est une position dans l'espace du sens.
- Même modèle pour la question et les documents.
- Le cosinus compare des directions. Les seuils se calibrent.
- Bon sur le thème, faible sur la négation et les identifiants.
