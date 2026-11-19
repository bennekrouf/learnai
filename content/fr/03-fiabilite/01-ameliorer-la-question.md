---
id: "03-01"
titre: "Améliorer la question"
sous_rubriques:
  - id: reformulation
    titre: "Reformulation"
  - id: decomposition
    titre: "Décomposition"
  - id: hyde
    titre: "HyDE"
sources:
  - titre: "Ma et al. — Query Rewriting for Retrieval-Augmented Large Language Models (Rewrite-Retrieve-Read), 2023"
    url: "https://arxiv.org/abs/2305.14283"
  - titre: "Press et al. — Measuring and Narrowing the Compositionality Gap in Language Models (Self-Ask), 2022"
    url: "https://arxiv.org/abs/2210.03350"
  - titre: "Gao, Ma, Lin, Callan — Precise Zero-Shot Dense Retrieval without Relevance Labels (HyDE), 2022"
    url: "https://arxiv.org/abs/2212.10496"
qcm:
  - id: reformulation-1
    sous_rubrique: reformulation
    question: "L'utilisateur demande « Quels sont les délais de livraison en France ? », puis « Et pour l'Espagne ? ». Que chercher pour la seconde ?"
    choix:
      - "« Délais de livraison en Espagne »"
      - "« Et pour l'Espagne ? »"
      - "« Espagne »"
      - "Toute la conversation"
    reponse: 0
    pourquoi: >-
      « Et pour l'Espagne ? » n'a de sens qu'avec ce qui précède.
      La recherche, elle, ne voit pas l'historique.
      Un LLM réécrit la question pour qu'elle tienne seule.
      Envoyer toute la conversation noierait la recherche.

  - id: reformulation-2
    sous_rubrique: reformulation
    question: "Pourquoi générer plusieurs reformulations d'une même question ?"
    choix:
      - "Pour occuper le modèle"
      - "Pour réduire le coût"
      - "Pour traduire"
      - "Chaque formulation trouve des documents différents, qu'on fusionne ensuite"
    reponse: 3
    pourquoi: >-
      L'utilisateur emploie ses mots. Le document emploie les siens.
      Trois formulations couvrent plus de vocabulaire qu'une seule.
      On fusionne les résultats, par exemple avec RRF.
      Le prix : plusieurs recherches au lieu d'une.

  - id: reformulation-3
    sous_rubrique: reformulation
    question: "L'utilisateur écrit « RTT CDD 2024 ». Que peut faire la reformulation ?"
    choix:
      - "Rien"
      - "Développer les sigles et ajouter les termes du corpus : « réduction du temps de travail », « contrat à durée déterminée »"
      - "Corriger l'orthographe seulement"
      - "Supprimer les sigles"
    reponse: 1
    pourquoi: >-
      Un sigle peut être écrit en entier dans le document.
      La recherche par mots-clés ne fait pas le lien seule.
      Garder le sigle ET sa forme longue couvre les deux cas.

  - id: decomposition-1
    sous_rubrique: decomposition
    question: "« Compare les conditions de remboursement de 2023 et de 2024. » Pourquoi découper ?"
    choix:
      - "La question est trop longue"
      - "Le modèle ne sait pas comparer"
      - "La réponse est dans deux documents distincts : une recherche par sous-question trouve chacun"
      - "Pour économiser des tokens"
    reponse: 2
    pourquoi: >-
      Une seule recherche ramène surtout les documents d'une des deux années.
      Deux sous-questions, deux recherches, deux documents.
      Le modèle compare ensuite ce qu'il a sous les yeux.

  - id: decomposition-2
    sous_rubrique: decomposition
    question: "Self-Ask (2022) parle d'un « écart de composition ». C'est…"
    choix:
      - "L'écart de prix entre deux modèles"
      - "L'écart entre deux langues"
      - "Un modèle répond juste aux sous-questions, mais échoue sur la question qui les combine"
      - "La différence entre BM25 et vectoriel"
    reponse: 2
    pourquoi: >-
      Le modèle sait qui a racheté Instagram.
      Il sait qui dirige Meta.
      Mais il rate « qui dirige l'entreprise qui a racheté Instagram ».
      Poser les sous-questions explicitement réduit cet écart.

  - id: decomposition-3
    sous_rubrique: decomposition
    question: "Quand ne pas décomposer ?"
    choix:
      - "Quand la question est simple : on ajoute du coût et de la latence pour rien"
      - "Jamais, il faut toujours décomposer"
      - "Quand la question est en français"
      - "Quand le corpus est grand"
    reponse: 0
    pourquoi: >-
      Chaque sous-question coûte un appel de LLM et une recherche.
      « Quel est le délai de préavis ? » n'a qu'une partie.
      Un simple classifieur peut décider s'il faut découper.

  - id: hyde-1
    sous_rubrique: hyde
    question: "Le principe de HyDE ?"
    choix:
      - "Cacher la question à l'utilisateur"
      - "Traduire la question"
      - "Chercher seulement par mots-clés"
      - "Faire écrire une réponse imaginaire, et chercher avec elle"
    reponse: 3
    pourquoi: >-
      HyDE veut dire Hypothetical Document Embeddings.
      Le LLM rédige une réponse plausible, même inexacte.
      On encode cette réponse, pas la question.
      On cherche les vrais documents qui lui ressemblent.

  - id: hyde-2
    sous_rubrique: hyde
    question: "Pourquoi une réponse imaginaire trouve-t-elle mieux qu'une question ?"
    choix:
      - "Elle est plus courte"
      - "Elle ressemble davantage aux documents : même forme, même vocabulaire"
      - "Elle contient la vraie réponse"
      - "Elle est en anglais"
    reponse: 1
    pourquoi: >-
      Une question et un paragraphe de réponse ne se ressemblent pas.
      Deux paragraphes de réponse, si.
      Même si les faits imaginés sont faux, le style et le vocabulaire guident la recherche.

  - id: hyde-3
    sous_rubrique: hyde
    question: "La réponse imaginaire de HyDE peut-elle être montrée à l'utilisateur ?"
    choix:
      - "Oui, c'est la réponse finale"
      - "Oui, avec un avertissement"
      - "Oui, si elle est courte"
      - "Non, elle sert seulement à chercher : elle peut être inventée"
    reponse: 3
    pourquoi: >-
      Elle est écrite sans aucun document.
      Elle contient souvent des faits inventés.
      Elle ne sert que de sonde pour la recherche.
      La réponse finale s'appuie sur les vrais documents retrouvés.
---

## Reformulation {#reformulation}

La question de l'utilisateur n'est pas toujours une bonne requête de recherche.

Elle dépend de l'historique. Elle utilise ses mots, pas ceux des documents.

```mermaid
sequenceDiagram
  participant U as Utilisateur
  participant A as Application
  participant L as LLM
  participant R as Recherche
  U->>A: Et pour l'Espagne ?
  A->>L: Historique + question : réécris-la pour qu'elle tienne seule
  L-->>A: Délais de livraison en Espagne
  A->>R: Délais de livraison en Espagne
  R-->>A: Documents
```

:::pourquoi
La recherche ne voit pas l'historique.
« Et pour l'Espagne ? » seul ne trouve rien d'utile.
:::

On peut générer plusieurs reformulations et fusionner les résultats.

:::pourquoi
Chaque formulation attrape d'autres documents.
On couvre plus de vocabulaire.
Le prix : plusieurs recherches.
:::

Développe les sigles. Garde aussi la forme courte.

:::pourquoi
Le document peut utiliser l'une ou l'autre.
BM25 ne fait pas le lien seul.
:::

## Décomposition {#decomposition}

Une question complexe cache plusieurs questions simples.

« Compare 2023 et 2024 » → « Règles 2023 ? » + « Règles 2024 ? »

:::pourquoi
Une seule recherche favorise un des deux sujets.
Une recherche par sous-question trouve chaque morceau.
:::

En 2022, Self-Ask mesure un **écart de composition** : le modèle répond aux sous-questions, mais rate la question combinée.

:::pourquoi
Combiner deux faits en silence est difficile.
Écrire les étapes rend chaque fait explicite.
:::

Ne découpe pas les questions simples.

:::pourquoi
Chaque sous-question coûte un appel et une recherche.
:::

## HyDE {#hyde}

HyDE (2022) inverse le problème. On ne cherche pas avec la question. On cherche avec une **réponse imaginée**.

```mermaid
flowchart LR
  Q["Question"] --> L["LLM : écris une réponse plausible"]
  L --> H["Réponse imaginaire"]
  H --> E["Embedding"]
  E --> R["Recherche vectorielle"]
  R --> D["Vrais documents"]
  D --> F["Réponse finale, appuyée sur les vrais documents"]
```

:::pourquoi
Une question ne ressemble pas à un paragraphe de réponse.
Une réponse imaginée, si.
Même fausse, elle a le bon vocabulaire et la bonne forme.
:::

La réponse imaginaire n'est **jamais** montrée.

:::pourquoi
Elle est écrite sans source.
Elle sert de sonde, pas de réponse.
:::

## À retenir

- Réécris la question pour qu'elle tienne seule.
- Plusieurs formulations, résultats fusionnés.
- Découpe les questions composées, pas les simples.
- HyDE : chercher avec une réponse imaginée, jamais la montrer.
