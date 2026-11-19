---
id: "02-05"
titre: "GraphRAG"
sous_rubriques:
  - id: entites
    titre: "Entités et relations"
  - id: multi-sauts
    titre: "Parcours multi-sauts"
  - id: communautes
    titre: "Résumés de communautés"
  - id: cout
    titre: "Quand ça ne vaut pas le coût"
sources:
  - titre: "Edge et al. (Microsoft) — From Local to Global: A Graph RAG Approach to Query-Focused Summarization, 2024"
    url: "https://arxiv.org/abs/2404.16130"
  - titre: "Microsoft Research — LazyGraphRAG: Setting a new standard for quality and cost, 25 novembre 2024"
    url: "https://www.microsoft.com/en-us/research/blog/lazygraphrag-setting-a-new-standard-for-quality-and-cost/"
qcm:
  - id: entites-1
    sous_rubrique: entites
    question: "Dans un graphe de connaissances, une relation s'écrit souvent comme…"
    choix:
      - "Un vecteur"
      - "Un triplet : sujet, relation, objet"
      - "Un morceau de texte"
      - "Un score"
    reponse: 1
    pourquoi: >-
      « Facebook — a racheté — Instagram ».
      Deux entités, reliées par une relation nommée.
      Les entités sont les nœuds. Les relations sont les arêtes.
      On peut ensuite suivre les arêtes, ce qu'un index de vecteurs ne sait pas faire.

  - id: entites-2
    sous_rubrique: entites
    question: "Qui extrait les entités et les relations dans GraphRAG ?"
    choix:
      - "Un humain"
      - "Le modèle d'embedding"
      - "BM25"
      - "Un LLM, morceau par morceau"
    reponse: 3
    pourquoi: >-
      Un LLM lit chaque morceau.
      Il liste les personnes, organisations, lieux, et leurs liens.
      C'est un appel de LLM par morceau.
      C'est précis, mais c'est ce qui rend l'indexation chère.

  - id: entites-3
    sous_rubrique: entites
    question: "« M. Dupont », « Jean Dupont » et « le PDG » désignent la même personne. Le défi ?"
    choix:
      - "La résolution d'entités : fusionner les mentions en un seul nœud"
      - "La traduction"
      - "Le découpage"
      - "Le reranking"
    reponse: 0
    pourquoi: >-
      Sans fusion, le graphe a trois nœuds pour une seule personne.
      Les relations se dispersent entre eux.
      Le parcours rate des liens.
      Fusionner trop, à l'inverse, mélange deux homonymes.

  - id: multi-sauts-1
    sous_rubrique: multi-sauts
    question: "« Qui dirige l'entreprise qui a racheté Instagram ? » Pourquoi la recherche vectorielle peine-t-elle ?"
    choix:
      - "La question est trop courte"
      - "La réponse est dans deux morceaux différents, et le second ne ressemble pas à la question"
      - "Instagram est un nom propre"
      - "Elle ne gère pas les questions"
    reponse: 1
    pourquoi: >-
      Morceau 1 : « Facebook rachète Instagram en 2012. »
      Morceau 2 : « Mark Zuckerberg dirige Meta, ex-Facebook. »
      La question ressemble au premier, pas au second.
      Il faut d'abord trouver Facebook, puis chercher qui le dirige. Deux sauts.

  - id: multi-sauts-2
    sous_rubrique: multi-sauts
    question: "Comment un graphe répond-il à une question multi-sauts ?"
    choix:
      - "Il cherche le morceau le plus proche"
      - "Il résume tout le corpus"
      - "Il part des entités de la question et suit les relations, saut par saut"
      - "Il demande à l'utilisateur"
    reponse: 2
    pourquoi: >-
      On repère « Instagram » dans la question.
      On suit « racheté par » : Facebook.
      On suit « dirigé par » : Mark Zuckerberg.
      Chaque arête renvoie aux morceaux sources, qu'on injecte comme preuves.

  - id: multi-sauts-3
    sous_rubrique: multi-sauts
    question: "Pourquoi limiter le parcours à 1 ou 2 sauts ?"
    choix:
      - "Le nombre de voisins explose à chaque saut"
      - "Les graphes n'ont que 2 niveaux"
      - "C'est une règle de Microsoft"
      - "Pour la sécurité"
    reponse: 0
    pourquoi: >-
      Une entité a 10 voisins. Chacun en a 10.
      Deux sauts : 100 nœuds. Trois sauts : 1 000.
      Le contexte se remplit de liens lointains et inutiles.
      Peu de sauts, et on garde les arêtes pertinentes.

  - id: communautes-1
    sous_rubrique: communautes
    question: "« Quels sont les grands thèmes de ces 10 000 documents ? » Pourquoi le RAG vectoriel échoue-t-il ?"
    choix:
      - "La question est mal écrite"
      - "Il manque un reranker"
      - "La réponse n'est dans aucun morceau : elle demande une vue d'ensemble"
      - "Le corpus est trop petit"
    reponse: 2
    pourquoi: >-
      Le RAG vectoriel trouve des passages qui ressemblent à la question.
      Aucun passage ne liste les thèmes de tout le corpus.
      C'est une question globale.
      Microsoft a conçu GraphRAG pour ce type de question.

  - id: communautes-2
    sous_rubrique: communautes
    question: "Dans GraphRAG, une « communauté », c'est…"
    choix:
      - "Un groupe d'utilisateurs"
      - "Un cluster de serveurs"
      - "Un ensemble de documents de même date"
      - "Un groupe d'entités très liées entre elles dans le graphe"
    reponse: 3
    pourquoi: >-
      Un algorithme comme Leiden découpe le graphe en groupes denses.
      Par exemple : toutes les entités liées à un même procès.
      Le découpage est hiérarchique : grands groupes, sous-groupes.
      Chaque groupe correspond à peu près à un thème.

  - id: communautes-3
    sous_rubrique: communautes
    question: "Comment GraphRAG répond-il à une question globale ?"
    choix:
      - "Il lit tout le corpus à chaque question"
      - "Il utilise des résumés de communautés préparés à l'indexation, puis les combine"
      - "Il prend le premier document"
      - "Il demande à BM25"
    reponse: 1
    pourquoi: >-
      À l'indexation, un LLM résume chaque communauté.
      À la question, chaque résumé produit une réponse partielle.
      Les réponses partielles sont combinées en une réponse finale.
      C'est un schéma map-reduce.

  - id: cout-1
    sous_rubrique: cout
    question: "Le principal défaut de GraphRAG dans sa version d'origine ?"
    choix:
      - "Il ne gère pas le français"
      - "Il est lent à la question"
      - "L'indexation coûte très cher en appels de LLM"
      - "Il ne cite pas ses sources"
    reponse: 2
    pourquoi: >-
      Un appel de LLM par morceau pour extraire le graphe.
      Puis un appel par communauté pour la résumer.
      Sur un gros corpus, c'est beaucoup d'argent.
      Et il faut recommencer quand le corpus change.

  - id: cout-2
    sous_rubrique: cout
    question: "LazyGraphRAG (Microsoft, novembre 2024) annonce un coût d'indexation de…"
    choix:
      - "0,1 % de celui de GraphRAG, autant que le RAG vectoriel"
      - "10 fois celui de GraphRAG"
      - "Le même que GraphRAG"
      - "Zéro"
    reponse: 0
    pourquoi: >-
      LazyGraphRAG ne résume rien à l'avance.
      Il repousse le travail du LLM au moment de la question.
      L'indexation coûte alors autant qu'un RAG vectoriel.
      Le coût se déplace : moins à l'indexation, un peu plus à chaque question.

  - id: cout-3
    sous_rubrique: cout
    question: "Ton corpus est un support client : « comment changer mon mot de passe ? ». Faut-il GraphRAG ?"
    choix:
      - "Oui, toujours"
      - "Non, il faut un LLM plus grand"
      - "Oui, pour le multilingue"
      - "Non : ce sont des questions locales, un RAG hybride avec reranking suffit"
    reponse: 3
    pourquoi: >-
      La réponse ressemble à la question et tient dans un passage.
      C'est le cas idéal du RAG vectoriel et hybride.
      Un graphe ajouterait du coût sans gain.
      Commence simple, mesure, et n'ajoute un graphe que si des questions multi-sauts ou globales échouent.
---

## Entités et relations {#entites}

Le RAG classique stocke des morceaux isolés. GraphRAG stocke aussi **les liens** entre eux.

Un LLM lit chaque morceau et en extrait :

- des **entités** : personnes, organisations, lieux, produits ;
- des **relations** : « Facebook — a racheté — Instagram ».

:::pourquoi
Un index de vecteurs sait dire « ce morceau ressemble à la question ».
Il ne sait pas dire « ce morceau est lié à celui-là ».
Le graphe garde ce lien.
:::

Le plus dur : reconnaître qu'un même acteur porte plusieurs noms.

:::pourquoi
« M. Dupont », « Jean Dupont », « le PDG » : trois nœuds au lieu d'un.
Les relations se dispersent, le parcours rate des liens.
C'est la résolution d'entités.
:::

## Parcours multi-sauts {#multi-sauts}

Certaines questions demandent de relier deux faits.

« Qui dirige l'entreprise qui a racheté Instagram ? »

```mermaid
flowchart LR
  I["Instagram"] -->|"racheté par (2012)"| F["Facebook / Meta"]
  F -->|"dirigé par"| Z["Mark Zuckerberg"]
  Q["Question"] -.->|"entité repérée"| I
```

:::pourquoi
Le morceau sur le rachat ressemble à la question.
Le morceau sur le dirigeant, non.
La recherche vectorielle trouve le premier et rate le second.
Le graphe suit l'arête et trouve les deux.
:::

On limite le parcours à 1 ou 2 sauts.

:::pourquoi
À chaque saut, le nombre de voisins est multiplié.
Au-delà, le contexte se remplit de liens lointains.
:::

## Résumés de communautés {#communautes}

En 2024, Microsoft publie GraphRAG pour une autre famille de questions : les questions **globales**.

« Quels sont les grands thèmes de ce corpus ? »

:::pourquoi
Aucun passage ne contient la réponse.
Elle demande une vue d'ensemble.
Le RAG vectoriel n'a rien de ressemblant à retrouver.
:::

La méthode :

```mermaid
flowchart LR
  M["Morceaux"] --> X["LLM : extraction"]
  X --> G["Graphe"]
  G --> C["Leiden : communautés"]
  C --> S["LLM : un résumé par communauté"]
  Q["Question globale"] --> P["Réponses partielles, une par résumé"]
  S --> P
  P --> F["Réponse finale combinée"]
```

:::pourquoi
Une communauté est un groupe d'entités très liées : souvent un thème.
Résumer chaque thème à l'avance prépare la vue d'ensemble.
À la question, on combine les résumés au lieu de relire le corpus.
:::

## Quand ça ne vaut pas le coût {#cout}

GraphRAG coûte cher à construire.

:::pourquoi
Un appel de LLM par morceau pour extraire.
Un appel par communauté pour résumer.
Tout est à refaire quand le corpus change beaucoup.
:::

Microsoft le reconnaît lui-même. En novembre 2024, il publie **LazyGraphRAG** : indexation au même coût qu'un RAG vectoriel, soit 0,1 % de GraphRAG.

:::pourquoi
LazyGraphRAG ne résume rien à l'avance.
Il fait travailler le LLM au moment de la question.
Le coût se déplace de l'indexation vers la requête.
:::

Règle de décision :

| Type de question | Exemple | Outil |
|---|---|---|
| Locale | « Comment changer mon mot de passe ? » | RAG hybride + reranking |
| Multi-sauts | « Qui dirige l'entreprise qui a racheté X ? » | Graphe, 1 à 2 sauts |
| Globale | « Quels sont les grands thèmes ? » | Résumés de communautés |

:::pourquoi
Selon Microsoft, le RAG vectoriel excelle quand la réponse ressemble à la question.
Commence par lui. Mesure.
Ajoute un graphe seulement si des questions multi-sauts ou globales échouent.
:::

## À retenir

- Un graphe stocke les liens que les vecteurs ignorent.
- Multi-sauts : on suit les relations, 1 ou 2 sauts.
- Questions globales : résumés de communautés, combinés.
- Coût d'indexation élevé. Commence simple, ajoute un graphe si la mesure le justifie.
