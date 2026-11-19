---
id: "09-04"
titre: "Publication"
sous_rubriques:
  - id: analyse-architecture
    titre: "Analyse de son architecture"
sources:
  - titre: "Anthropic — How we built our multi-agent research system, 13 juin 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, 12 juin 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, 2 mai 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
qcm:
  - id: analyse-architecture-1
    sous_rubrique: analyse-architecture
    question: "Pourquoi publier l'analyse de son architecture ?"
    choix:
      - "Pour montrer son raisonnement : les choix, les compromis, les mesures et les échecs"
      - "Pour montrer qu'on utilise les outils à la mode"
      - "Pour cacher les problèmes"
      - "Pour remplacer le code"
    reponse: 0
    pourquoi: >-
      Le code montre ce qui a été fait.
      L'analyse montre pourquoi, et à quel prix.
      C'est ce qu'un recruteur ou un pair veut évaluer.
      Le jugement se voit dans les choix, pas dans la liste d'outils.

  - id: analyse-architecture-2
    sous_rubrique: analyse-architecture
    question: "Quel plan suit une bonne analyse d'architecture ?"
    choix:
      - "Le code complet copié dans l'article"
      - "Liste des technologies utilisées"
      - "Captures d'écran de l'interface uniquement"
      - "Problème et contraintes, architecture, décisions et options écartées, mesures, échecs, ce que je changerais"
    reponse: 3
    pourquoi: >-
      Les contraintes expliquent les choix.
      Les options écartées montrent qu'il y a eu un choix.
      Les mesures prouvent.
      Les échecs et le « je changerais » montrent le recul.

  - id: analyse-architecture-3
    sous_rubrique: analyse-architecture
    question: "Anthropic annonce +90,2 % pour son multi-agent. Qu'est-ce qui rend l'article crédible ?"
    choix:
      - "Le chiffre est très élevé"
      - "Il donne aussi le coût (environ 15 fois plus de tokens) et les tâches où l'approche convient mal"
      - "Il ne parle que des succès"
      - "Il est long"
    reponse: 1
    pourquoi: >-
      Un gain sans prix ressemble à de la publicité.
      Un gain avec son prix et ses limites ressemble à de l'ingénierie.
      Le lecteur peut décider si le compromis vaut pour lui.

  - id: analyse-architecture-4
    sous_rubrique: analyse-architecture
    question: "Faut-il raconter ses échecs dans une analyse publiée ?"
    choix:
      - "Seulement en note de bas de page"
      - "Non, jamais"
      - "Seulement s'ils sont drôles"
      - "Oui : un échec expliqué enseigne plus qu'un succès et montre qu'on a testé"
    reponse: 3
    pourquoi: >-
      Cognition construit son article sur un échec : Flappy Bird.
      OpenAI publie ce qu'il a raté avec GPT-4o.
      Un échec montre qu'on a mesuré, et ce qu'on a appris.
      Un article sans échec fait douter des tests.

  - id: analyse-architecture-5
    sous_rubrique: analyse-architecture
    question: "Que faut-il retirer avant de publier ?"
    choix:
      - "Les diagrammes"
      - "Clés d'API, données de clients, noms internes sensibles, et prompts qui révèlent des secrets"
      - "Les chiffres"
      - "Les options écartées"
    reponse: 1
    pourquoi: >-
      Une clé dans un extrait de code se vole en minutes.
      Une trace d'agent peut contenir des données personnelles.
      On publie le raisonnement, pas les secrets.
      L'étape 6 s'applique aussi à un article.

  - id: analyse-architecture-6
    sous_rubrique: analyse-architecture
    question: "Comment présenter un chiffre dans l'article ?"
    choix:
      - "Avec sa méthode : jeu de test, nombre de cas et de passages, marge d'erreur, version du modèle"
      - "En gras, sans contexte"
      - "Arrondi au chiffre le plus flatteur"
      - "Sans unité"
    reponse: 0
    pourquoi: >-
      « 92 % de réussite » ne dit rien seul.
      Sur 20 cas ? Sur 2 000 ? Un passage ou cinq ?
      La méthode permet au lecteur de juger.
      C'est la fiche de la rubrique Benchmarks.

  - id: analyse-architecture-7
    sous_rubrique: analyse-architecture
    question: "Quelle section montre le mieux le recul de l'auteur ?"
    choix:
      - "Le titre"
      - "La liste des remerciements"
      - "« Ce que je changerais », avec les raisons"
      - "La date de publication"
    reponse: 2
    pourquoi: >-
      Elle prouve qu'on a regardé son travail de façon critique.
      Elle montre ce qu'on a appris.
      Et elle répond d'avance aux critiques des lecteurs.

  - id: analyse-architecture-8
    sous_rubrique: analyse-architecture
    question: "Comment rendre l'analyse vérifiable par le lecteur ?"
    choix:
      - "Demander au lecteur de faire confiance"
      - "Lier le dépôt, les ADR, les diagrammes et les résultats d'évaluation"
      - "Ne donner aucun lien"
      - "Publier seulement une vidéo"
    reponse: 1
    pourquoi: >-
      L'article résume. Le dépôt prouve.
      Les ADR donnent le détail des décisions.
      Les résultats permettent de refaire la mesure.
      Tout le chapitre converge ici.

  - id: analyse-architecture-9
    sous_rubrique: analyse-architecture
    question: "Quel titre convient le mieux à une analyse d'architecture ?"
    choix:
      - "« Utilisation de LangChain, Pinecone et GPT »"
      - "« Mon super projet IA révolutionnaire »"
      - "« Projet final »"
      - "« Un agent unique plutôt que trois : 2,8 fois moins cher, et ce que ça nous a coûté »"
    reponse: 3
    pourquoi: >-
      Le titre annonce une décision et son compromis.
      Il dit au lecteur ce qu'il va apprendre.
      Une liste d'outils ne dit rien du raisonnement.
      Un superlatif fait fuir les lecteurs techniques.

  - id: analyse-architecture-10
    sous_rubrique: analyse-architecture
    question: "Avant publication, qui doit relire l'article ?"
    choix:
      - "Seulement le modèle qui l'a aidé à l'écrire"
      - "Personne"
      - "Un pair pour la technique, et une relecture sécurité pour les fuites"
      - "Seulement l'auteur, une fois de plus"
    reponse: 2
    pourquoi: >-
      Un pair voit les trous dans le raisonnement.
      Une relecture sécurité voit la clé oubliée ou la donnée client.
      Ce qui est publié ne se retire jamais complètement.
---

## Analyse de son architecture {#analyse-architecture}

Le code montre **ce qui** a été fait. L'analyse montre **pourquoi**, et **à quel prix**.

:::pourquoi
Un recruteur ou un pair veut voir ton jugement.
Le jugement se voit dans les choix et les compromis, pas dans la liste d'outils.
:::

Le plan :

| Section | Contenu |
|---|---|
| Problème et contraintes | Ce qu'il fallait résoudre, avec quel budget, quelle latence, quelles données |
| Architecture | Flux de données et topologie d'agents |
| Décisions | Les choix clés, les options écartées, liens vers les ADR |
| Mesures | Qualité, coût, latence, avec la méthode |
| Échecs | Ce qui n'a pas marché, et pourquoi |
| Ce que je changerais | Avec les raisons |

:::pourquoi
Les contraintes expliquent les choix.
Les options écartées prouvent qu'il y a eu un choix.
Les échecs et le « je changerais » montrent le recul.
:::

Donne le **prix** avec le **gain**.

:::pourquoi
Anthropic annonce +90,2 % pour son multi-agent.
Il écrit aussi : environ 15 fois plus de tokens, et des tâches où ça convient mal.
C'est ce qui rend l'article crédible.
:::

Raconte les **échecs**.

:::pourquoi
Cognition bâtit son article sur Flappy Bird raté.
OpenAI publie ce qu'il a manqué avec GPT-4o.
Un article sans échec fait douter des tests.
:::

Chaque chiffre vient avec sa **méthode** : jeu de test, nombre de cas, passages, marge d'erreur, version du modèle.

:::pourquoi
« 92 % » sur 20 cas et un passage ne vaut pas « 92 % » sur 2 000 cas et cinq passages.
Le lecteur doit pouvoir juger.
:::

Avant de publier, deux relectures :

```mermaid
sequenceDiagram
  participant A as Auteur
  participant P as Pair technique
  participant S as Relecture sécurité
  participant L as Lecteurs
  A->>P: Brouillon + dépôt + résultats
  P-->>A: Trous dans le raisonnement, chiffres sans méthode
  A->>S: Brouillon corrigé
  S-->>A: Clé d'API dans un extrait, nom de client dans une trace
  A->>A: Retire les secrets, anonymise
  A->>L: Publie avec liens vers dépôt, ADR, diagrammes, résultats
```

:::pourquoi
Un pair voit ce que l'auteur ne voit plus.
Une clé publiée se vole en minutes.
Ce qui est publié ne se retire jamais complètement.
:::

On retire : **clés d'API**, **données de clients**, **noms internes sensibles**, **prompts** qui révèlent des secrets.

:::pourquoi
Une trace d'agent peut contenir des données personnelles.
On publie le raisonnement, pas les secrets.
:::

Un **titre** qui annonce une décision et son compromis.

| Moins bien | Mieux |
|---|---|
| « Mon super projet IA » | « Un agent unique plutôt que trois : 2,8 fois moins cher » |
| « Utilisation de LangChain et Pinecone » | « Pourquoi notre cache sémantique renvoyait la mauvaise réponse » |

:::pourquoi
Le titre dit au lecteur ce qu'il va apprendre.
Une liste d'outils ne dit rien du raisonnement.
:::

Tout le chapitre converge ici : l'article **résume**, le dépôt **prouve**.

:::pourquoi
Les ADR donnent les décisions.
Les diagrammes donnent la structure.
Les résultats d'évaluation permettent de refaire la mesure.
:::

## À retenir

- Problème, architecture, décisions, mesures, échecs, ce que je changerais.
- Le prix avec le gain. Les échecs avec les succès.
- Chaque chiffre avec sa méthode.
- Relecture technique et sécurité. Liens vers le dépôt, les ADR, les résultats.
