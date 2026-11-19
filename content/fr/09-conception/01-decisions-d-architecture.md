---
id: "09-01"
titre: "Décisions d'architecture"
sous_rubriques:
  - id: adr
    titre: "ADR"
  - id: compromis
    titre: "Justifier un compromis"
sources:
  - titre: "Michael Nygard — Documenting Architecture Decisions, 15 novembre 2011"
    url: "https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions"
  - titre: "Anthropic — How we built our multi-agent research system, 13 juin 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, 12 juin 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
qcm:
  - id: adr-1
    sous_rubrique: adr
    question: "Qu'est-ce qu'un ADR ?"
    choix:
      - "La documentation de l'API"
      - "Un diagramme de classes"
      - "Un ticket de bug"
      - "Un court document qui consigne une décision d'architecture, son contexte et ses conséquences"
    reponse: 3
    pourquoi: >-
      ADR veut dire Architecture Decision Record.
      Michael Nygard a proposé ce format en 2011.
      Une décision par fichier, une ou deux pages au plus.
      Il répond à la question « pourquoi c'est fait comme ça ? ».

  - id: adr-2
    sous_rubrique: adr
    question: "Quelles sections compte l'ADR proposé par Michael Nygard ?"
    choix:
      - "Résumé, Code, Tests"
      - "Titre, Contexte, Décision, Statut, Conséquences"
      - "Problème, Solution, Prix"
      - "Auteur, Date, Signature"
    reponse: 1
    pourquoi: >-
      Le contexte dit les forces en jeu.
      La décision dit ce qu'on fait.
      Le statut dit si elle tient toujours.
      Les conséquences disent ce qu'elle coûte et rapporte.

  - id: adr-3
    sous_rubrique: adr
    question: "Que doit contenir la section Conséquences ?"
    choix:
      - "Les effets positifs, négatifs et neutres de la décision"
      - "Seulement les avantages"
      - "Le nom de celui qui a décidé"
      - "Le code de la solution"
    reponse: 0
    pourquoi: >-
      Une décision qui n'a que des avantages n'est pas une décision.
      Les inconvénients acceptés sont l'information la plus utile.
      Ils disent au lecteur futur ce qui était connu.
      Et ce qui doit être surveillé.

  - id: adr-4
    sous_rubrique: adr
    question: "Une décision est remplacée six mois plus tard. Que faire de l'ancien ADR ?"
    choix:
      - "Le réécrire avec la nouvelle décision"
      - "Le supprimer"
      - "Le garder, le marquer « remplacé » et pointer vers le nouvel ADR"
      - "Le laisser tel quel sans rien indiquer"
    reponse: 2
    pourquoi: >-
      L'historique explique le chemin parcouru.
      On sait pourquoi on avait choisi autrement, et pourquoi on a changé.
      Réécrire efface cette trace.
      Les numéros ne sont jamais réutilisés.

  - id: adr-5
    sous_rubrique: adr
    question: "Où ranger les ADR ?"
    choix:
      - "Dans la tête de l'architecte"
      - "Dans le dépôt du projet, en Markdown, à côté du code"
      - "Dans un e-mail"
      - "Sur un tableau blanc"
    reponse: 1
    pourquoi: >-
      Dans le dépôt, l'ADR suit le code.
      Il se relit en PR, comme le code.
      Il se retrouve avec une recherche.
      Nygard proposait doc/arch/adr-NNN.md.

  - id: adr-6
    sous_rubrique: adr
    question: "Quelle décision mérite un ADR ?"
    choix:
      - "Changer la couleur d'un bouton"
      - "Renommer une variable locale"
      - "Corriger une faute dans un commentaire"
      - "Choisir un agent unique plutôt qu'un système multi-agent"
    reponse: 3
    pourquoi: >-
      Un ADR est pour ce qui coûte cher à changer.
      La topologie d'agents touche tout : coût, latence, fiabilité.
      Renommer une variable se défait en une minute.
      Trop d'ADR noie les importants.

  - id: compromis-1
    sous_rubrique: compromis
    question: "Qu'est-ce que justifier un compromis ?"
    choix:
      - "Choisir la technologie la plus récente"
      - "Dire que la solution n'a pas de défaut"
      - "Dire ce qu'on gagne, ce qu'on perd, et pourquoi ce prix est acceptable ici"
      - "Copier ce que fait une grande entreprise"
    reponse: 2
    pourquoi: >-
      Toute architecture paie quelque chose.
      Le compromis dit quoi, et contre quoi.
      Le « ici » compte : le même prix peut être inacceptable ailleurs.

  - id: compromis-2
    sous_rubrique: compromis
    question: "Anthropic gagne 90,2 % avec son multi-agent de recherche. Pourquoi ne pas l'adopter partout ?"
    choix:
      - "Il consomme environ 15 fois plus de tokens, et il convient mal aux tâches à contexte partagé"
      - "Parce qu'il est interdit"
      - "Parce qu'il est plus lent dans tous les cas"
      - "Il n'y a aucune raison, il faut l'adopter partout"
    reponse: 0
    pourquoi: >-
      Le gain a un prix : le volume de tokens.
      Anthropic dit que la tâche doit valoir ce prix.
      Et que les tâches avec beaucoup de dépendances, comme le code, s'y prêtent mal.
      Le gain dépend du contexte.

  - id: compromis-3
    sous_rubrique: compromis
    question: "Cognition et Anthropic arrivent à des conclusions opposées. Qu'est-ce qui explique la différence ?"
    choix:
      - "Ils utilisent des langages différents"
      - "L'un des deux s'est trompé"
      - "Leurs tâches : du code aux parties liées pour l'un, de la recherche en pistes indépendantes pour l'autre"
      - "Le hasard"
    reponse: 2
    pourquoi: >-
      Le code a des dépendances : le décor et l'oiseau doivent aller ensemble.
      La recherche se découpe en pistes indépendantes.
      Même question, contextes différents, réponses différentes.
      C'est pourquoi le contexte s'écrit dans l'ADR.

  - id: compromis-4
    sous_rubrique: compromis
    question: "Comment rendre un compromis vérifiable ?"
    choix:
      - "Le chiffrer : coût, latence, qualité mesurés sur le même jeu de test pour chaque option"
      - "L'écrire en majuscules"
      - "Le faire valider par le modèle"
      - "Ne pas l'écrire"
    reponse: 0
    pourquoi: >-
      « Plus fiable » ne se discute pas.
      « 12 % de réussite en plus pour 3 fois le coût » se discute.
      Les chiffres viennent des étapes 7 et 8.
      Un lecteur peut refaire la mesure.

  - id: compromis-5
    sous_rubrique: compromis
    question: "Pourquoi écrire les options écartées dans l'ADR ?"
    choix:
      - "Ce n'est pas utile"
      - "Pour allonger le document"
      - "Pour critiquer leurs auteurs"
      - "Pour qu'on ne les repropose pas sans savoir pourquoi elles ont perdu"
    reponse: 3
    pourquoi: >-
      Dans six mois, quelqu'un proposera l'option écartée.
      Avec l'ADR, il voit les raisons du refus.
      Si le contexte a changé, il peut rouvrir la décision.
      Sinon, on gagne une discussion.

  - id: compromis-6
    sous_rubrique: compromis
    question: "Qu'est-ce qu'une condition de révision dans un ADR ?"
    choix:
      - "La date de la prochaine réunion"
      - "Le signal qui doit faire rouvrir la décision, par exemple « si le coût par tâche dépasse 0,50 € »"
      - "Le nombre de relecteurs"
      - "Une règle de formatage"
    reponse: 1
    pourquoi: >-
      Une décision vaut tant que son contexte tient.
      Écrire ce qui la rendrait fausse, c'est préparer le prochain choix.
      Un nouveau modèle, un nouveau prix, un nouveau volume peuvent tout changer.
---

## ADR {#adr}

Un **ADR** (Architecture Decision Record) consigne une décision d'architecture. Une décision par fichier.

:::pourquoi
Dans six mois, quelqu'un demandera « pourquoi c'est fait comme ça ? ».
Sans trace, la réponse est perdue avec la personne qui est partie.
:::

Le format de Michael Nygard (2011) :

| Section | Contenu |
|---|---|
| Titre | Un numéro et une phrase courte |
| Contexte | Les forces en jeu, sans parti pris |
| Décision | « Nous allons… », à la voix active |
| Statut | Proposé, accepté, déprécié, remplacé |
| Conséquences | Effets positifs, négatifs et neutres |

:::pourquoi
Le contexte explique. La décision tranche.
Les conséquences disent le prix accepté.
Une ou deux pages : sinon, personne ne lit.
:::

Un exemple :

```markdown
# ADR 007 : Un agent unique pour le support client

Statut : accepté

## Contexte
Les tâches de support enchaînent des étapes liées :
identité, commande, remboursement. Chaque étape dépend de la précédente.

## Décision
Nous allons utiliser un agent unique avec un historique partagé.

## Options écartées
Superviseur et trois sous-agents : 2,8 fois plus de tokens,
et deux incohérences sur 50 cas de test.

## Conséquences
+ Contexte continu, moins d'incohérences.
- Pas de parallélisme : latence p95 de 14 s.
- L'historique grossit : résumé nécessaire au-delà de 30 étapes.

## Condition de révision
Si la latence p95 dépasse 20 s, ou si des tâches indépendantes apparaissent.
```

Un ADR remplacé n'est **jamais supprimé**. Il est marqué « remplacé par ADR 012 ».

:::pourquoi
L'historique montre le chemin parcouru.
On voit pourquoi on avait choisi autrement, et ce qui a changé.
:::

Les ADR vivent **dans le dépôt**, à côté du code.

:::pourquoi
Ils se relisent en PR, comme le code.
Ils se retrouvent avec une recherche.
:::

Un ADR pour ce qui **coûte cher à changer** : topologie d'agents, modèle, base vectorielle, stratégie d'évaluation.

:::pourquoi
Trop d'ADR noie les importants.
Renommer une variable ne mérite pas de document.
:::

## Justifier un compromis {#compromis}

Toute architecture **paie** quelque chose. Le compromis dit quoi, contre quoi, et pourquoi c'est acceptable **ici**.

:::pourquoi
Anthropic gagne 90,2 % avec son multi-agent de recherche.
Il paie environ 15 fois plus de tokens qu'une conversation.
Pour de la recherche à forte valeur, le prix se justifie.
:::

Le même choix peut être bon ici et mauvais là.

| | Cognition | Anthropic |
|---|---|---|
| Tâche | Écrire du code | Faire de la recherche |
| Parties | Liées entre elles | Indépendantes |
| Choix | Un agent, un fil | Agent principal et sous-agents en parallèle |
| Prix accepté | Pas de parallélisme | Beaucoup plus de tokens |

:::pourquoi
Le décor et l'oiseau de Flappy Bird doivent aller ensemble.
Deux pistes de recherche, non.
Anthropic note lui-même que la plupart des tâches de code s'y prêtent mal.
:::

Un compromis se **chiffre**.

:::pourquoi
« Plus fiable » ne se discute pas.
« 12 % de réussite en plus pour 3 fois le coût » se discute.
Les chiffres viennent du jeu de test (étape 7) et de la mesure du coût (étape 8).
:::

On écrit aussi les **options écartées**, et une **condition de révision**.

:::pourquoi
L'option écartée reviendra sur la table.
La condition de révision dit quand la décision doit être rouverte.
Un nouveau modèle ou un nouveau prix peut tout changer.
:::

## À retenir

- Un ADR : titre, contexte, décision, statut, conséquences. Une ou deux pages.
- Jamais supprimé, seulement remplacé. Rangé dans le dépôt.
- Un compromis dit ce qu'on gagne, ce qu'on paie, et pourquoi ici.
- Chiffrer, écrire les options écartées, prévoir la condition de révision.
