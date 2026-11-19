---
id: "02-01"
titre: "Préparer le corpus"
sous_rubriques:
  - id: decoupage
    titre: "Découpage"
  - id: metadonnees
    titre: "Métadonnées"
  - id: indexation
    titre: "Indexation"
sources:
  - titre: "Anthropic — Introducing Contextual Retrieval, septembre 2024"
    url: "https://www.anthropic.com/engineering/contextual-retrieval"
  - titre: "Malkov & Yashunin — Efficient and robust approximate nearest neighbor search using HNSW graphs"
    url: "https://arxiv.org/abs/1603.09320"
qcm:
  - id: decoupage-1
    sous_rubrique: decoupage
    question: "Pourquoi découper les documents en morceaux avant de les indexer ?"
    choix:
      - "Pour économiser de l'espace disque"
      - "Parce que les modèles d'embedding refusent les longs textes"
      - "Un vecteur pour 100 pages mélange tout, et on ne veut injecter que le passage utile"
      - "Pour traduire plus facilement"
    reponse: 2
    pourquoi: >-
      Un seul vecteur pour un long document fait la moyenne de tous ses sujets.
      Il ne ressemble plus à aucune question précise.
      Et injecter 100 pages pour une phrase utile gaspille le budget.
      Un morceau, c'est une idée, un vecteur, un passage injectable.

  - id: decoupage-2
    sous_rubrique: decoupage
    question: "Un morceau dit : « Le chiffre d'affaires a progressé de 3 % sur le trimestre. » Le problème ?"
    choix:
      - "On ne sait plus de quelle entreprise ni de quel trimestre il parle"
      - "Il est trop long"
      - "Il contient un chiffre"
      - "Aucun"
    reponse: 0
    pourquoi: >-
      Le nom de l'entreprise et la date étaient plus haut dans le document.
      Le découpage les a séparés.
      La question « chiffre d'affaires d'ACME au T2 » ne trouvera pas ce morceau.
      C'est l'exemple d'Anthropic pour présenter Contextual Retrieval.

  - id: decoupage-3
    sous_rubrique: decoupage
    question: "Contextual Retrieval (Anthropic, 2024) ajoute à chaque morceau…"
    choix:
      - "Sa traduction en anglais"
      - "Un résumé de tout le corpus"
      - "Le document entier"
      - "Quelques phrases générées par un LLM qui situent le morceau dans son document"
    reponse: 3
    pourquoi: >-
      Un LLM lit le document et le morceau.
      Il écrit 50 à 100 tokens de contexte : entreprise, période, section.
      On colle ce contexte devant le morceau avant de l'indexer.
      Avec la recherche par mots-clés aussi, les échecs baissent de 49 %.

  - id: decoupage-4
    sous_rubrique: decoupage
    question: "À quoi sert le chevauchement entre deux morceaux ?"
    choix:
      - "À doubler le nombre de résultats"
      - "À ne pas couper une idée en deux sans qu'aucun morceau la contienne entière"
      - "À accélérer la recherche"
      - "À rien"
    reponse: 1
    pourquoi: >-
      Une coupure tombe parfois au milieu d'une explication.
      Chaque moitié devient incompréhensible.
      Répéter quelques phrases d'un morceau au suivant limite ce risque.
      Le coût : un peu plus de stockage et de doublons.

  - id: decoupage-5
    sous_rubrique: decoupage
    question: "Le meilleur endroit pour couper un document ?"
    choix:
      - "Tous les 500 caractères exactement"
      - "Au milieu des phrases"
      - "Au hasard"
      - "Aux frontières naturelles : titres, paragraphes, fonctions"
    reponse: 3
    pourquoi: >-
      L'auteur a déjà structuré son document par idées.
      Couper sur cette structure garde des morceaux qui ont un sens.
      Un tableau coupé en deux perd ses en-têtes.
      Une fonction coupée en deux ne veut plus rien dire.

  - id: metadonnees-1
    sous_rubrique: metadonnees
    question: "Google a fini par filtrer la satire de ses résumés IA. Sur quelle information ?"
    choix:
      - "Une métadonnée : la nature de la source"
      - "Le texte de la page"
      - "La longueur de la page"
      - "La langue de la page"
    reponse: 0
    pourquoi: >-
      Le texte d'une blague ressemble au texte d'un conseil.
      Ni un embedding ni BM25 ne voient la différence.
      L'information « site satirique » ou « forum » est une métadonnée.
      Il faut l'enregistrer à l'indexation pour pouvoir filtrer.

  - id: metadonnees-2
    sous_rubrique: metadonnees
    question: "Un employé n'a pas accès au dossier RH. Où appliquer ce droit ?"
    choix:
      - "Dans le prompt : « ne révèle pas les données RH »"
      - "Au moment de la recherche, en filtrant sur les droits d'accès"
      - "Après la réponse, en relisant le texte"
      - "Nulle part, le modèle est discret"
    reponse: 1
    pourquoi: >-
      Un document injecté dans le contexte est lu par le modèle.
      Une consigne peut être contournée.
      Une relecture après coup arrive trop tard.
      Le seul filtre sûr : le document n'est jamais retrouvé pour cet utilisateur.

  - id: metadonnees-3
    sous_rubrique: metadonnees
    question: "Tu cherches les 10 meilleurs morceaux, puis tu gardes ceux de 2024. Il n'en reste aucun. Pourquoi ?"
    choix:
      - "Il n'y a pas de document de 2024"
      - "L'index est cassé"
      - "Le filtre appliqué après la recherche a éliminé les 10 résultats"
      - "BM25 ignore les dates"
    reponse: 2
    pourquoi: >-
      Les 10 meilleurs étaient peut-être tous de 2023.
      Filtrer après coup ne repêche rien.
      Filtre avant ou pendant la recherche.
      La plupart des bases vectorielles le permettent.

  - id: indexation-1
    sous_rubrique: indexation
    question: "HNSW, utilisé par beaucoup de bases vectorielles, est une recherche…"
    choix:
      - "Approximative et rapide"
      - "Exacte et lente"
      - "Par mots-clés"
      - "Aléatoire"
    reponse: 0
    pourquoi: >-
      Comparer la question à des millions de vecteurs, un par un, est trop lent.
      HNSW construit un graphe de voisins pour sauter vers la bonne zone.
      Il rate parfois un voisin. C'est le prix de la vitesse.
      On règle ce compromis avec ses paramètres.

  - id: indexation-2
    sous_rubrique: indexation
    question: "Un document est mis à jour. Que faire de ses anciens morceaux ?"
    choix:
      - "Les garder, plus il y a de données mieux c'est"
      - "Les marquer comme anciens dans le texte"
      - "Les supprimer de tous les index avant d'ajouter les nouveaux"
      - "Rien, l'index se met à jour seul"
    reponse: 2
    pourquoi: >-
      Sinon l'ancienne version reste trouvable.
      Le modèle peut citer un tarif périmé.
      Donne un identifiant stable à chaque document.
      À chaque mise à jour, tous ses morceaux sont remplacés.

  - id: indexation-3
    sous_rubrique: indexation
    question: "Que stocker avec chaque morceau, en plus de son vecteur ?"
    choix:
      - "Rien"
      - "La question qui l'a trouvé"
      - "Le document entier"
      - "Son texte, l'identifiant du document, sa position et ses métadonnées"
    reponse: 3
    pourquoi: >-
      Le texte, pour l'injecter dans le contexte.
      L'identifiant, pour citer la source et mettre à jour.
      La position, pour aller chercher les morceaux voisins.
      Les métadonnées, pour filtrer.
---

## Découpage {#decoupage}

On ne cherche pas dans des documents. On cherche dans des **morceaux** (*chunks*).

:::pourquoi
Un vecteur pour un long document fait la moyenne de ses sujets.
Il ne ressemble plus à aucune question précise.
Un morceau porte une idée. Il se retrouve et s'injecte.
:::

La taille est un compromis.

| Trop petit | Trop grand |
|---|---|
| Perd son contexte | Mélange plusieurs idées |
| « Il a progressé de 3 % » : qui ? | Gaspille le budget de tokens |

Coupe aux frontières naturelles : titres, paragraphes, fonctions. Ne coupe jamais un tableau.

:::pourquoi
L'auteur a déjà structuré son texte par idées.
Un tableau sans ses en-têtes n'a plus de sens.
:::

Ajoute un **chevauchement** : quelques phrases répétées d'un morceau au suivant.

:::pourquoi
Une coupure peut tomber au milieu d'une explication.
Le chevauchement garde l'idée entière dans au moins un morceau.
:::

En 2024, Anthropic publie **Contextual Retrieval**. Un LLM écrit 50 à 100 tokens de contexte pour chaque morceau. On les colle devant avant d'indexer.

| Méthode | Échecs dans les 20 premiers |
|---|---|
| Base | 5,7 % |
| + contexte, embeddings | 3,7 % |
| + contexte, embeddings et BM25 | 2,9 % |
| + reranking | 1,9 % |

:::pourquoi
« Le chiffre d'affaires a progressé de 3 % » devient « ACME, T2 2023 : le chiffre d'affaires a progressé de 3 % ».
Le morceau redevient trouvable.
Ce sont les chiffres d'Anthropic, sur ses propres données. Chez toi, ils seront différents.
:::

## Métadonnées {#metadonnees}

Chaque morceau porte une fiche : source, date, auteur, type, langue, version, droits d'accès.

:::pourquoi
Le texte ne dit pas tout.
Une blague et un conseil se ressemblent. Seule la source les distingue.
C'est ce que Google a dû ajouter après la colle sur la pizza.
:::

Les droits d'accès se filtrent **pendant** la recherche.

:::pourquoi
Un document retrouvé est lu par le modèle.
Une consigne « ne le révèle pas » peut être contournée.
Le seul filtre sûr : il n'est jamais retrouvé.
:::

Filtre avant ou pendant la recherche. Pas après.

:::pourquoi
Les 10 meilleurs résultats peuvent tous être éliminés par le filtre.
Il ne reste alors rien, alors que de bons documents existaient.
:::

## Indexation {#indexation}

Chaque morceau va dans deux index.

```mermaid
flowchart LR
  D["Documents"] --> C["Découpeur"]
  C --> X["LLM : contexte du morceau"]
  X --> E["Modèle d'embedding"]
  E --> V["Index vectoriel"]
  X --> B["Index BM25"]
  M["Métadonnées"] --> V
  M --> B
```

L'index vectoriel utilise une recherche **approximative**, souvent HNSW.

:::pourquoi
Comparer la question à des millions de vecteurs un par un est trop lent.
HNSW saute de voisin en voisin vers la bonne zone.
Il rate parfois un voisin. C'est le prix de la vitesse.
:::

Stocke avec chaque morceau : son texte, l'identifiant du document, sa position, ses métadonnées.

:::pourquoi
Texte pour l'injecter. Identifiant pour citer et mettre à jour.
Position pour lire les voisins. Métadonnées pour filtrer.
:::

Un document change : remplace **tous** ses morceaux.

:::pourquoi
Un ancien morceau oublié reste trouvable.
Le modèle citera un tarif périmé, en toute confiance.
:::

## À retenir

- Des morceaux qui portent une idée, coupés sur la structure.
- Un morceau sans contexte devient introuvable. Ajoute-le.
- Les métadonnées filtrent ce que le texte ne dit pas, dont les droits d'accès.
- Mise à jour : on remplace tous les morceaux du document.
