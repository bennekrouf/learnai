---
id: "03-02"
titre: "Recherche en plusieurs étapes"
sous_rubriques:
  - id: boucler
    titre: "Boucler"
  - id: arreter
    titre: "Savoir s'arrêter"
sources:
  - titre: "Trivedi et al. — Interleaving Retrieval with Chain-of-Thought Reasoning (IRCoT), 2022"
    url: "https://arxiv.org/abs/2212.10509"
  - titre: "Asai et al. — Self-RAG: Learning to Retrieve, Generate, and Critique through Self-Reflection, 2023"
    url: "https://arxiv.org/abs/2310.11511"
  - titre: "Yan et al. — Corrective Retrieval Augmented Generation (CRAG), 2024"
    url: "https://arxiv.org/abs/2401.15884"
qcm:
  - id: boucler-1
    sous_rubrique: boucler
    question: "Pourquoi une seule recherche ne suffit-elle pas toujours ?"
    choix:
      - "Les index sont lents"
      - "Ce qu'on lit dans un premier document dit souvent quoi chercher ensuite"
      - "Les modèles oublient vite"
      - "Une recherche est limitée à un mot"
    reponse: 1
    pourquoi: >-
      « Qui dirige l'entreprise qui a racheté Instagram ? »
      Le premier document révèle : Facebook.
      C'est seulement maintenant qu'on sait quoi chercher : le dirigeant de Facebook.
      La seconde requête dépend de la première lecture.

  - id: boucler-2
    sous_rubrique: boucler
    question: "Dans un RAG agentique, qui décide de lancer une nouvelle recherche ?"
    choix:
      - "Le modèle, via un outil de recherche qu'il appelle quand il en a besoin"
      - "L'utilisateur"
      - "La base vectorielle"
      - "Un minuteur"
    reponse: 0
    pourquoi: >-
      La recherche devient un outil, comme au chapitre 0.
      Le modèle l'appelle, lit, et décide s'il lui en faut plus.
      Il peut changer de requête à chaque tour.
      C'est la boucle d'outils, appliquée à la recherche.

  - id: boucler-3
    sous_rubrique: boucler
    question: "CRAG (2024) ajoute un évaluateur des documents retrouvés. S'il les juge mauvais ?"
    choix:
      - "On répond quand même avec eux"
      - "On arrête tout"
      - "On les écarte et on cherche ailleurs, par exemple sur le web"
      - "On les donne au reranker"
    reponse: 2
    pourquoi: >-
      Répondre à partir de mauvais documents mène à une mauvaise réponse.
      L'évaluateur note chaque lot : correct, ambigu ou incorrect.
      Incorrect : on jette et on cherche une autre source.
      C'est la correction avant la génération.

  - id: boucler-4
    sous_rubrique: boucler
    question: "Self-RAG (2023) apprend au modèle à décider…"
    choix:
      - "S'il faut chercher, et si sa réponse est soutenue par ce qu'il a lu"
      - "Quelle langue utiliser"
      - "Combien de tokens écrire"
      - "Quel modèle d'embedding choisir"
    reponse: 0
    pourquoi: >-
      Toutes les questions n'ont pas besoin de recherche.
      Self-RAG entraîne le modèle à émettre des jetons de réflexion.
      Ils disent : chercher ou non, passage utile ou non, réponse soutenue ou non.
      Le modèle critique son propre travail en cours de route.

  - id: arreter-1
    sous_rubrique: arreter
    question: "Quel critère d'arrêt est le plus sain ?"
    choix:
      - "Quand le modèle se dit satisfait, sans autre limite"
      - "Jamais : plus on cherche, mieux c'est"
      - "Après exactement 10 recherches"
      - "Toutes les sous-questions ont une preuve, OU un maximum de tours est atteint"
    reponse: 3
    pourquoi: >-
      Le but est d'avoir une preuve pour chaque partie de la question.
      Mais un modèle peut chercher sans fin.
      Le maximum de tours protège le coût et la latence.
      Les deux critères ensemble : arrêt quand c'est fini, ou quand ça dure trop.

  - id: arreter-2
    sous_rubrique: arreter
    question: "La nouvelle recherche renvoie les mêmes documents que la précédente. Que faire ?"
    choix:
      - "Relancer la même requête"
      - "S'arrêter ou changer franchement de stratégie : on tourne en rond"
      - "Augmenter la température"
      - "Doubler k"
    reponse: 1
    pourquoi: >-
      Aucune information nouvelle n'est entrée.
      Continuer coûte sans rien apporter.
      Soit on a tout ce que le corpus contient, soit la requête est mal posée.
      Dans le premier cas, on répond ou on s'abstient.

  - id: arreter-3
    sous_rubrique: arreter
    question: "Pourquoi accumuler tous les documents de toutes les étapes est-il risqué ?"
    choix:
      - "Ça n'a aucun risque"
      - "Les documents expirent"
      - "Le contexte se remplit de distracteurs et de doublons, et la qualité baisse"
      - "Le modèle refuse plus de 5 documents"
    reponse: 2
    pourquoi: >-
      C'est le Context Rot du chapitre 1.
      Chaque étape ajoute des passages, pertinents ou non.
      On déduplique, et on ne garde que ce qui sert la réponse.
---

## Boucler {#boucler}

Un RAG simple fait : chercher, puis répondre. Une fois.

Un RAG en plusieurs étapes fait : chercher, lire, décider, chercher encore.

```mermaid
flowchart LR
  Q["Question"] --> L["LLM"]
  L -->|"requête"| R["Recherche"]
  R --> E["Évaluateur des documents"]
  E -->|"bons"| L
  E -->|"mauvais : chercher ailleurs"| W["Autre source"]
  W --> L
  L -->|"assez de preuves"| F["Réponse"]
```

:::pourquoi
Ce qu'on lit change ce qu'il faut chercher.
Le premier document révèle souvent le nom à chercher ensuite.
:::

Plusieurs familles d'approches :

| Approche | Idée |
|---|---|
| IRCoT (2022) | Alterner une étape de raisonnement et une recherche |
| Self-RAG (2023) | Le modèle décide s'il faut chercher, et juge s'il est soutenu |
| CRAG (2024) | Un évaluateur note les documents, et cherche ailleurs s'ils sont mauvais |
| RAG agentique | La recherche est un outil que le modèle appelle librement |

:::pourquoi
Toutes partagent une idée : la recherche n'est plus une étape fixe.
Elle devient une décision prise en cours de route.
:::

## Savoir s'arrêter {#arreter}

Une boucle sans fin coûte cher. Et elle n'améliore pas la réponse.

Arrête quand :

- chaque partie de la question a une preuve ;
- **ou** le maximum de tours est atteint ;
- **ou** les recherches ne ramènent plus rien de nouveau.

:::pourquoi
Le modèle peut toujours trouver une raison de chercher encore.
La limite protège le budget et le temps de réponse.
Des résultats identiques signalent qu'on tourne en rond.
:::

Ne garde pas tout ce que les étapes ont ramené.

:::pourquoi
Chaque étape ajoute des passages.
Doublons et distracteurs font baisser la qualité.
Déduplique. Garde ce qui sert la réponse.
:::

Si la boucle s'arrête sans preuve suffisante, on ne devine pas. On s'abstient.

:::pourquoi
Une boucle qui échoue est une information utile.
Elle dit que le corpus ne contient pas la réponse.
C'est l'objet de la rubrique Abstention.
:::

## À retenir

- Ce qu'on lit dit quoi chercher ensuite.
- La recherche devient une décision, prise par le modèle ou par un évaluateur.
- Arrêt : preuves complètes, maximum de tours, ou plus rien de neuf.
- Pas de preuve à la fin : on s'abstient.
