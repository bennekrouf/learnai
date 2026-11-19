---
id: "02-04"
titre: "Mesurer la recherche"
sous_rubriques:
  - id: metriques
    titre: "recall@k, MRR, nDCG"
  - id: jeu-de-test
    titre: "Construire un jeu de test"
sources:
  - titre: "Anthropic — Introducing Contextual Retrieval (mesure : échecs dans les 20 premiers)"
    url: "https://www.anthropic.com/engineering/contextual-retrieval"
  - titre: "Järvelin & Kekäläinen — Cumulated gain-based evaluation of IR techniques (nDCG), 2002"
    url: "https://dl.acm.org/doi/10.1145/582415.582418"
qcm:
  - id: metriques-1
    sous_rubrique: metriques
    question: "3 documents sont pertinents. 2 apparaissent dans les 10 premiers résultats. Quel est le recall@10 ?"
    choix:
      - "0,2"
      - "0,67"
      - "1"
      - "0,3"
    reponse: 1
    pourquoi: >-
      Le recall@k compte la part des pertinents trouvés dans les k premiers.
      2 trouvés sur 3 existants : 2/3 ≈ 0,67.
      Il ne regarde pas l'ordre à l'intérieur des 10.

  - id: metriques-2
    sous_rubrique: metriques
    question: "Pour un RAG, pourquoi le recall@k est-il la mesure clé de la première étape ?"
    choix:
      - "Il est le plus simple à calculer"
      - "Il mesure la qualité de la réponse"
      - "Ce qui n'est pas dans les k morceaux envoyés au LLM ne peut pas servir à répondre"
      - "Il mesure la latence"
    reponse: 2
    pourquoi: >-
      Le LLM ne voit que les k morceaux qu'on lui donne.
      Un bon document au rang k+1 n'existe pas pour lui.
      Anthropic mesure d'ailleurs ses gains en échecs dans les 20 premiers.
      C'est 1 moins le recall@20.

  - id: metriques-3
    sous_rubrique: metriques
    question: "Trois questions. Le premier bon résultat est au rang 1, puis 2, puis 4. Quel est le MRR ?"
    choix:
      - "≈ 0,58"
      - "≈ 2,33"
      - "0,25"
      - "1"
    reponse: 0
    pourquoi: >-
      On prend l'inverse du rang du premier bon résultat.
      1/1 = 1. 1/2 = 0,5. 1/4 = 0,25.
      Moyenne : 1,75 / 3 ≈ 0,58.
      Le MRR punit fort un bon résultat qui n'arrive pas en tête.

  - id: metriques-4
    sous_rubrique: metriques
    question: "Quand préférer nDCG à MRR ?"
    choix:
      - "Quand il n'y a qu'une seule bonne réponse"
      - "Quand on ne connaît pas les documents pertinents"
      - "Quand plusieurs documents sont pertinents, à des degrés différents"
      - "Jamais"
    reponse: 2
    pourquoi: >-
      Le MRR ne regarde que le premier bon résultat.
      nDCG note chaque document : très pertinent, un peu, pas du tout.
      Il récompense les plus pertinents placés en haut.
      Il vaut 1 quand l'ordre est parfait.

  - id: metriques-5
    sous_rubrique: metriques
    question: "Dans nDCG, que veut dire le « D », pour discounted ?"
    choix:
      - "Les documents récents comptent plus"
      - "Les doublons sont retirés"
      - "Les documents longs sont pénalisés"
      - "Un document pertinent compte moins quand il est classé plus bas"
    reponse: 3
    pourquoi: >-
      Un bon document au rang 1 vaut plus qu'au rang 8.
      On divise son gain par une fonction qui grandit avec le rang.
      Le classement compte, pas seulement la présence.

  - id: jeu-de-test-1
    sous_rubrique: jeu-de-test
    question: "Un jeu de test pour la recherche, c'est…"
    choix:
      - "Une liste de documents"
      - "Des questions, chacune avec les morceaux qui y répondent"
      - "Des réponses rédigées par le LLM"
      - "Les logs de latence"
    reponse: 1
    pourquoi: >-
      Pour mesurer un rappel, il faut savoir ce qu'on aurait dû trouver.
      Chaque question porte donc la liste de ses morceaux pertinents.
      Sans ça, aucune des mesures précédentes n'est calculable.

  - id: jeu-de-test-2
    sous_rubrique: jeu-de-test
    question: "Tu fais générer les questions par un LLM à partir de chaque morceau. Le piège ?"
    choix:
      - "Les questions seront en anglais"
      - "Il y aura trop de questions"
      - "Les questions reprennent les mots du morceau, ce qui flatte BM25 et surestime la recherche"
      - "Aucun"
    reponse: 2
    pourquoi: >-
      Un LLM qui lit un morceau reprend ses mots.
      Les vrais utilisateurs, eux, ne connaissent pas ces mots.
      Le test devient trop facile.
      Mélange avec de vraies questions d'utilisateurs, et reformule.

  - id: jeu-de-test-3
    sous_rubrique: jeu-de-test
    question: "Quels cas ne doivent pas manquer dans ton jeu de test ?"
    choix:
      - "Identifiants, synonymes, questions multi-documents, et questions sans réponse"
      - "Seulement les questions faciles"
      - "Seulement les questions longues"
      - "Seulement les questions en anglais"
    reponse: 0
    pourquoi: >-
      Les identifiants testent BM25. Les synonymes testent le vectoriel.
      Les questions multi-documents testent le rappel.
      Les questions sans réponse testent l'abstention, à l'étape 3.
      Un test fait de cas faciles ne te dira rien.

  - id: jeu-de-test-4
    sous_rubrique: jeu-de-test
    question: "Tu règles la taille des morceaux jusqu'à avoir le meilleur score sur ton jeu de test. Le risque ?"
    choix:
      - "Aucun"
      - "Le score baisse"
      - "Le jeu de test devient trop grand"
      - "Sur-ajuster au jeu de test : le score monte, la vraie qualité non"
    reponse: 3
    pourquoi: >-
      À force de régler sur les mêmes questions, on optimise pour elles.
      Les vraies questions, elles, sont différentes.
      Garde une partie du jeu de côté, jamais utilisée pour régler.
      C'est elle qui dit la vérité.
---

## recall@k, MRR, nDCG {#metriques}

Sans mesure, on règle à l'aveugle.

Trois mesures, trois questions.

| Mesure | Question | Usage |
|---|---|---|
| recall@k | Les bons documents sont-ils dans les k premiers ? | Première étape d'un RAG |
| MRR | À quel rang arrive le premier bon document ? | Une seule bonne réponse |
| nDCG | Les plus pertinents sont-ils tout en haut ? | Pertinence graduée |

**recall@k** : pertinents trouvés dans les k premiers, divisé par pertinents existants.

:::pourquoi
Le LLM ne voit que les k morceaux qu'on lui donne.
Ce qui est au rang k+1 n'existe pas pour lui.
Anthropic mesure Contextual Retrieval en échecs dans les 20 premiers : 1 moins le recall@20.
:::

**MRR** : moyenne de 1/rang du premier bon résultat.

:::pourquoi
Rang 1 vaut 1. Rang 2 vaut 0,5. Rang 10 vaut 0,1.
Il punit fort un bon résultat qui n'arrive pas en tête.
:::

**nDCG** : chaque document a une note de pertinence. Son gain diminue avec le rang. On divise par le score d'un ordre parfait.

:::pourquoi
Certains documents sont très pertinents, d'autres un peu.
nDCG récompense les meilleurs placés en haut.
1 veut dire ordre parfait.
:::

## Construire un jeu de test {#jeu-de-test}

Un jeu de test : des questions, et pour chacune, les morceaux qui y répondent.

:::pourquoi
Pour mesurer ce qu'on a raté, il faut savoir ce qu'on aurait dû trouver.
:::

D'où viennent les questions ?

- De vrais utilisateurs : les meilleures, mais il faut les annoter.
- Générées par un LLM à partir des morceaux : rapides, mais trop faciles.

:::pourquoi
Un LLM qui lit un morceau reprend ses mots.
La recherche par mots-clés les trouve trop bien.
Le score devient flatteur.
:::

Couvre les cas difficiles :

- identifiants et codes ;
- synonymes et reformulations ;
- réponses réparties sur plusieurs documents ;
- questions **sans** réponse dans le corpus.

:::pourquoi
Chaque cas teste une faiblesse différente.
Un jeu fait de cas faciles affiche 95 % et ne t'apprend rien.
:::

Commence petit : 50 à 100 questions. Agrandis au fil des échecs réels.

Garde une partie du jeu de côté. Ne règle jamais sur elle.

:::pourquoi
À force de régler sur les mêmes questions, on optimise pour elles.
La partie mise de côté dit si le gain est réel.
:::

Relance la mesure à chaque changement : découpage, modèle, filtres, reranker.

```mermaid
flowchart LR
  Q["Jeu de test"] --> S["Système de recherche"]
  S --> R["Résultats classés"]
  R --> M["Calcul : recall@k, MRR, nDCG"]
  Q -->|"morceaux attendus"| M
  M --> D["Décision : garder ou annuler le changement"]
```

## À retenir

- recall@k pour la première étape. MRR ou nDCG pour l'ordre.
- Un jeu de test : questions et morceaux attendus.
- Questions générées par LLM : trop faciles seules.
- Une partie du jeu mise de côté, jamais utilisée pour régler.
