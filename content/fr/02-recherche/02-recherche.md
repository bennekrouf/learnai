---
id: "02-02"
titre: "Recherche"
sous_rubriques:
  - id: bm25
    titre: "BM25"
  - id: vectorielle
    titre: "Recherche vectorielle"
  - id: hybride
    titre: "Fusion hybride (RRF)"
sources:
  - titre: "Thakur et al. — BEIR: A Heterogeneous Benchmark for Zero-shot Evaluation of Information Retrieval Models, NeurIPS 2021"
    url: "https://arxiv.org/abs/2104.08663"
  - titre: "Cormack, Clarke, Büttcher — Reciprocal Rank Fusion outperforms Condorcet and individual Rank Learning Methods, SIGIR 2009"
    url: "https://plg.uwaterloo.ca/~gvcormac/cormacksigir09-rrf.pdf"
qcm:
  - id: bm25-1
    sous_rubrique: bm25
    question: "Dans BM25, quel mot de la question pèse le plus ?"
    choix:
      - "Le plus fréquent dans le corpus"
      - "Le plus rare dans le corpus"
      - "Le premier de la question"
      - "Le plus long"
    reponse: 1
    pourquoi: >-
      Un mot présent partout ne distingue aucun document.
      « le », « de », « contrat » apparaissent dans tout un corpus juridique.
      « résiliation anticipée » est rare, donc discriminant.
      BM25 donne plus de poids aux mots rares. C'est l'IDF.

  - id: bm25-2
    sous_rubrique: bm25
    question: "Un mot apparaît 50 fois dans un document. Le score BM25 est-il 50 fois plus haut qu'avec une seule occurrence ?"
    choix:
      - "Oui, c'est proportionnel"
      - "Non, seule la première occurrence compte"
      - "Non, l'effet sature"
      - "Oui, et c'est même plus"
    reponse: 2
    pourquoi: >-
      Répéter un mot ne rend pas un document 50 fois plus pertinent.
      BM25 fait monter le score vite au début, puis de moins en moins.
      Ça limite aussi les documents qui bourrent un mot-clé.

  - id: bm25-3
    sous_rubrique: bm25
    question: "Quelle question BM25 rate-t-il le plus facilement ?"
    choix:
      - "« comment résilier mon abonnement » quand le document dit « mettre fin au contrat »"
      - "« facture INV-2024-0042 »"
      - "« article L1234-5 »"
      - "« Dupont-Moretti »"
    reponse: 0
    pourquoi: >-
      BM25 compte des mots communs.
      « résilier » et « mettre fin » ont le même sens, mais aucun mot en commun.
      BM25 ne voit pas les synonymes.
      Il excelle en revanche sur les identifiants, codes et noms propres.

  - id: bm25-4
    sous_rubrique: bm25
    question: "En français, la question dit « chevaux », le document dit « cheval ». Que faut-il pour que BM25 les relie ?"
    choix:
      - "Rien, il le fait seul"
      - "Une température plus haute"
      - "Un embedding"
      - "Un analyseur de langue qui ramène les mots à leur racine"
    reponse: 3
    pourquoi: >-
      Pour BM25, « chevaux » et « cheval » sont deux mots différents.
      Un analyseur français normalise : racine, accents, mots vides.
      Mal configuré, l'index anglais par défaut rate ces liens.

  - id: vectorielle-1
    sous_rubrique: vectorielle
    question: "Le point fort de la recherche vectorielle ?"
    choix:
      - "Les identifiants exacts"
      - "Les synonymes et les reformulations"
      - "Les numéros de version"
      - "Elle ne coûte rien à indexer"
    reponse: 1
    pourquoi: >-
      L'embedding capte le sens, pas les mots.
      « résilier » et « mettre fin » se retrouvent proches.
      Avec un modèle multilingue, une question en français trouve un texte anglais.

  - id: vectorielle-2
    sous_rubrique: vectorielle
    question: "Le benchmark BEIR (2021) teste des modèles sur des domaines qu'ils n'ont pas vus. Ce qu'il montre ?"
    choix:
      - "La recherche vectorielle bat toujours BM25"
      - "Tous les modèles se valent"
      - "BM25 est obsolète"
      - "BM25 reste une base solide, et des modèles vectoriels font moins bien hors de leur domaine"
    reponse: 3
    pourquoi: >-
      Un modèle d'embedding apprend sur certains types de textes.
      Sur un domaine nouveau, médical ou juridique, il peut perdre pied.
      BM25 n'a rien appris, donc rien à oublier.
      Avant de jeter BM25, mesure sur tes données.

  - id: vectorielle-3
    sous_rubrique: vectorielle
    question: "Certains modèles d'embedding demandent un préfixe différent pour la question et pour le document. Pourquoi ?"
    choix:
      - "Une question courte et un passage long ne se ressemblent pas, même quand l'un répond à l'autre"
      - "Pour la sécurité"
      - "Pour compter les tokens"
      - "Pour changer de langue"
    reponse: 0
    pourquoi: >-
      « Quel est le délai de préavis ? » ne ressemble pas à un paragraphe de contrat.
      Le modèle a appris à rapprocher une question de sa réponse.
      Le préfixe lui dit quel rôle joue chaque texte.
      L'oublier dégrade les résultats sans message d'erreur.

  - id: hybride-1
    sous_rubrique: hybride
    question: "Pourquoi combiner BM25 et recherche vectorielle ?"
    choix:
      - "Pour doubler le nombre de résultats"
      - "Leurs échecs sont différents : l'un rattrape l'autre"
      - "Parce que c'est plus rapide"
      - "Pour réduire le coût"
    reponse: 1
    pourquoi: >-
      BM25 rate les synonymes. Le vectoriel rate les identifiants.
      Une question peut contenir les deux.
      Fusionner donne la chance aux deux types de correspondance.

  - id: hybride-2
    sous_rubrique: hybride
    question: "Pourquoi ne pas simplement additionner le score BM25 et le score cosinus ?"
    choix:
      - "C'est trop lent"
      - "Le cosinus est toujours nul"
      - "Les deux scores n'ont pas la même échelle"
      - "BM25 n'a pas de score"
    reponse: 2
    pourquoi: >-
      Un score BM25 peut valoir 3 ou 30, selon la question.
      Un cosinus reste entre -1 et 1.
      Additionner donnerait tout le pouvoir à BM25.
      On fusionne donc les rangs, pas les scores.

  - id: hybride-3
    sous_rubrique: hybride
    question: "RRF avec k = 60. Document A : 1er dans BM25, absent du vectoriel. Document B : 2e dans les deux. Qui gagne ?"
    choix:
      - "B"
      - "A"
      - "Égalité"
      - "Impossible à dire"
    reponse: 0
    pourquoi: >-
      A : 1/(60+1) ≈ 0,0164.
      B : 1/(60+2) + 1/(60+2) ≈ 0,0323.
      B gagne nettement.
      RRF récompense l'accord entre les deux recherches.

  - id: hybride-4
    sous_rubrique: hybride
    question: "D'où vient la valeur k = 60 de RRF ?"
    choix:
      - "D'une démonstration mathématique"
      - "Du nombre de résultats retournés"
      - "D'un essai pilote en 2009, et le choix s'est révélé peu critique"
      - "D'une norme ISO"
    reponse: 2
    pourquoi: >-
      Cormack, Clarke et Büttcher ont testé plusieurs valeurs sur des données TREC.
      60 était proche du meilleur, sans que le choix soit décisif.
      La valeur est restée par défaut presque partout.
      k amortit l'écart entre le 1er et le 2e rang.
---

## BM25 {#bm25}

BM25 cherche par **mots**. C'est l'héritier des moteurs de recherche des années 1990.

Il favorise trois choses :

- les mots **rares** du corpus (ils distinguent) ;
- les mots **présents** dans le document, avec un effet qui sature ;
- les documents **courts** à score égal.

:::pourquoi
Un mot présent partout ne distingue rien.
Répéter un mot 50 fois ne rend pas 50 fois plus pertinent.
Un long document contient beaucoup de mots par hasard.
:::

Fort sur : identifiants, codes, noms propres, termes techniques.

Faible sur : synonymes, reformulations, autres langues.

:::pourquoi
BM25 compare des mots, pas des idées.
« résilier » et « mettre fin » n'ont aucun mot commun.
:::

En français, configure un analyseur de langue.

:::pourquoi
Sans lui, « chevaux » et « cheval » sont deux mots sans rapport.
L'analyseur ramène les mots à leur racine et gère les accents.
:::

## Recherche vectorielle {#vectorielle}

On encode la question. On cherche les morceaux dont le vecteur est le plus proche.

Fort sur : synonymes, reformulations, multilingue.

Faible sur : identifiants, négation, domaines inconnus du modèle.

:::pourquoi
L'embedding capte le sens général.
Un numéro de facture n'a pas de « sens ».
Un modèle entraîné sur du web généraliste connaît mal ton jargon métier.
:::

En 2021, le benchmark BEIR teste des modèles sur 18 jeux de données. BM25 reste une base solide. Plusieurs modèles vectoriels font moins bien hors de leur domaine.

:::pourquoi
Un modèle appris sur un domaine perd pied sur un autre.
BM25 n'a rien appris, donc rien à oublier.
Mesure sur tes données avant de choisir.
:::

Respecte les consignes du modèle : certains veulent un préfixe pour la question et un autre pour le document.

:::pourquoi
Une question courte ne ressemble pas à un passage long.
Le préfixe dit au modèle quel rôle joue chaque texte.
L'oublier dégrade les résultats en silence.
:::

## Fusion hybride (RRF) {#hybride}

On lance les deux recherches. On fusionne leurs classements.

```mermaid
flowchart LR
  Q["Question"] --> B["BM25"]
  Q --> V["Recherche vectorielle"]
  B -->|"classement 1"| R["Fusion RRF"]
  V -->|"classement 2"| R
  R --> T["Top-k fusionné"]
```

:::pourquoi
Leurs échecs sont différents.
Ce que l'un rate, l'autre le trouve souvent.
:::

On fusionne les **rangs**, pas les scores.

:::pourquoi
Un score BM25 n'a pas d'échelle fixe. Un cosinus reste entre -1 et 1.
Les additionner donnerait tout le pouvoir à BM25.
:::

**RRF** (Reciprocal Rank Fusion, 2009) : chaque document gagne 1/(k + rang) dans chaque liste. On additionne. k vaut souvent 60.

| Document | Rang BM25 | Rang vectoriel | Score RRF |
|---|---|---|---|
| A | 1 | absent | 1/61 ≈ 0,0164 |
| B | 2 | 2 | 2 × 1/62 ≈ 0,0323 |

:::pourquoi
Un document bien classé par les deux recherches est un pari sûr.
RRF récompense cet accord.
k = 60 amortit l'écart entre le 1er et le 2e rang.
:::

## À retenir

- BM25 : mots exacts, identifiants. Rate les synonymes.
- Vectoriel : sens, synonymes. Rate les identifiants et les domaines inconnus.
- Hybride : on fusionne les rangs avec RRF.
- Mesure sur tes données avant d'écarter BM25.
