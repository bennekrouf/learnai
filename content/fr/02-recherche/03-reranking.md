---
id: "02-03"
titre: "Reranking"
sous_rubriques:
  - id: encodeurs
    titre: "Bi-encoder et cross-encoder"
  - id: compromis
    titre: "Latence contre précision"
sources:
  - titre: "Nogueira & Cho — Passage Re-ranking with BERT, 2019"
    url: "https://arxiv.org/abs/1901.04085"
  - titre: "Khattab & Zaharia — ColBERT: Efficient and Effective Passage Search via Contextualized Late Interaction over BERT, 2020"
    url: "https://arxiv.org/abs/2004.12832"
  - titre: "Thakur et al. — BEIR, NeurIPS 2021"
    url: "https://arxiv.org/abs/2104.08663"
qcm:
  - id: encodeurs-1
    sous_rubrique: encodeurs
    question: "Un bi-encoder encode la question et le document…"
    choix:
      - "Ensemble, dans une seule passe"
      - "Seulement le document"
      - "Seulement la question"
      - "Séparément, puis compare leurs vecteurs"
    reponse: 3
    pourquoi: >-
      C'est le fonctionnement d'un modèle d'embedding classique.
      Les vecteurs des documents sont calculés une fois, à l'indexation.
      À la recherche, on n'encode que la question.
      C'est rapide, mais les deux textes ne se « voient » jamais.

  - id: encodeurs-2
    sous_rubrique: encodeurs
    question: "Pourquoi un cross-encoder est-il plus précis ?"
    choix:
      - "Il utilise un plus grand vocabulaire"
      - "Il lit la question et le document ensemble, mot à mot"
      - "Il a plus de dimensions"
      - "Il ignore les mots rares"
    reponse: 1
    pourquoi: >-
      Le bi-encoder résume chaque texte en un vecteur, seul.
      Le cross-encoder lit les deux textes côte à côte.
      Il voit si le document répond vraiment à la question.
      Il repère une négation ou une condition que le vecteur a écrasée.

  - id: encodeurs-3
    sous_rubrique: encodeurs
    question: "Pourquoi ne pas utiliser un cross-encoder sur tout le corpus ?"
    choix:
      - "Il est moins précis sur les longs corpus"
      - "Il ne supporte pas le français"
      - "Il faut un passage du modèle par paire question-document, impossible à précalculer"
      - "Il est interdit par les licences"
    reponse: 2
    pourquoi: >-
      Le cross-encoder a besoin de la question pour calculer.
      Rien ne peut être préparé à l'avance.
      Un million de documents, c'est un million de passages par question.
      On le réserve donc à une courte liste de candidats.

  - id: encodeurs-4
    sous_rubrique: encodeurs
    question: "ColBERT (2020) est un compromis. Comment ?"
    choix:
      - "Il garde un vecteur par token, précalculé, et compare token à token"
      - "Il utilise BM25 en interne"
      - "Il encode seulement les titres"
      - "Il ne garde qu'un vecteur par corpus"
    reponse: 0
    pourquoi: >-
      Un vecteur par token garde plus de détails qu'un vecteur par texte.
      Ces vecteurs sont précalculés, comme pour un bi-encoder.
      La comparaison fine se fait à la recherche, token à token.
      Plus précis qu'un bi-encoder, plus rapide qu'un cross-encoder. Mais plus de stockage.

  - id: compromis-1
    sous_rubrique: compromis
    question: "Le reranker reçoit les 50 premiers candidats. Le bon document est 80e. Que se passe-t-il ?"
    choix:
      - "Le reranker le remonte en tête"
      - "Le reranker élargit la recherche"
      - "Le LLM le trouvera quand même"
      - "Il est perdu : le reranker ne voit que ce qu'on lui donne"
    reponse: 3
    pourquoi: >-
      Le reranker trie. Il ne cherche pas.
      Un document absent des candidats n'a aucune chance.
      La première étape doit donc viser le rappel : ne rien rater.
      La seconde vise la précision : bien trier.

  - id: compromis-2
    sous_rubrique: compromis
    question: "Tu passes de 50 à 200 candidats envoyés au reranker. Conséquence ?"
    choix:
      - "Aucun changement"
      - "Meilleur rappel possible, mais latence environ 4 fois plus grande pour le reranking"
      - "Moins de précision"
      - "Latence divisée par 4"
    reponse: 1
    pourquoi: >-
      Plus de candidats, plus de chances d'avoir le bon.
      Mais le cross-encoder fait un passage par candidat.
      Le temps de reranking grandit avec leur nombre.
      Le bon réglage se mesure : rappel gagné contre millisecondes perdues.

  - id: compromis-3
    sous_rubrique: compromis
    question: "Selon BEIR (2021), quels modèles ont les meilleurs résultats en moyenne, hors domaine ?"
    choix:
      - "Les bi-encoders seuls"
      - "Les modèles les plus petits"
      - "BM25 seul"
      - "Les rerankers et les modèles à interaction tardive, au prix d'un calcul élevé"
    reponse: 3
    pourquoi: >-
      Lire la question et le document ensemble généralise mieux.
      Mais ça coûte cher à chaque requête.
      D'où l'entonnoir : rapide et large d'abord, lent et fin ensuite.

  - id: compromis-4
    sous_rubrique: compromis
    question: "Quelle latence surveiller pour le reranking ?"
    choix:
      - "La p95 : la latence que 95 % des requêtes ne dépassent pas"
      - "La moyenne seulement"
      - "La plus rapide"
      - "Aucune, le reranking est instantané"
    reponse: 0
    pourquoi: >-
      La moyenne cache les requêtes lentes.
      Un utilisateur sur vingt peut attendre trois fois plus.
      La p95 montre ce que vivent les cas difficiles.
      C'est le sujet de l'étape 8.
---

## Bi-encoder et cross-encoder {#encodeurs}

Deux façons de mesurer si un document répond à une question.

| | Bi-encoder | Cross-encoder |
|---|---|---|
| Lecture | Question et document séparément | Question et document ensemble |
| Précalcul | Oui, à l'indexation | Impossible |
| Vitesse | Très rapide | Lent |
| Précision | Bonne | Meilleure |
| Usage | Première recherche | Second tri |

:::pourquoi
Le bi-encoder résume chaque texte seul, en un vecteur.
Le cross-encoder voit les deux textes côte à côte.
Il repère qu'un document parle du bon sujet sans répondre à la question.
:::

En 2019, Nogueira et Cho appliquent BERT en cross-encoder pour retrier des passages. Le gain est net sur le benchmark MS MARCO.

:::pourquoi
C'est le moment où le reranking neuronal devient la norme.
L'idée n'a pas changé depuis : un second tri plus fin sur peu de candidats.
:::

**ColBERT** (2020) est un compromis : un vecteur par token, précalculé, comparé token à token.

:::pourquoi
Plus de détail qu'un vecteur par texte.
Précalculable, contrairement au cross-encoder.
Le prix : beaucoup plus de stockage.
:::

## Latence contre précision {#compromis}

On organise la recherche en **entonnoir**.

```mermaid
flowchart LR
  C["Corpus : millions de morceaux"] --> H["Recherche hybride : top 100"]
  H --> R["Reranker : top 10"]
  R --> L["LLM"]
```

Étape 1 : large et rapide. Elle vise le **rappel** : ne rien rater.

Étape 2 : étroite et lente. Elle vise la **précision** : bien trier.

:::pourquoi
Le reranker trie. Il ne cherche pas.
Un document absent des candidats est perdu pour de bon.
:::

Le nombre de candidats est le réglage principal.

:::pourquoi
Plus de candidats, plus de chances d'avoir le bon.
Mais le reranker fait un passage par candidat.
Quatre fois plus de candidats, environ quatre fois plus de temps de reranking.
:::

Surveille la latence p95, pas la moyenne.

:::pourquoi
La moyenne cache les requêtes lentes.
La p95 dit ce que vivent les cas difficiles.
:::

## À retenir

- Bi-encoder : rapide, précalculé, pour chercher.
- Cross-encoder : précis, coûteux, pour retrier.
- Entonnoir : rappel d'abord, précision ensuite.
- Le reranker ne repêche jamais un document absent.
