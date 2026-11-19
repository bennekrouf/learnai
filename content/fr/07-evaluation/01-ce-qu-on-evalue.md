---
id: "07-01"
titre: "Ce qu'on évalue"
sous_rubriques:
  - id: reponse
    titre: "La réponse"
  - id: trajectoire
    titre: "La trajectoire"
sources:
  - titre: "Yao et al. — τ-bench: A Benchmark for Tool-Agent-User Interaction in Real-World Domains, 2024"
    url: "https://arxiv.org/abs/2406.12045"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, 2 mai 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
qcm:
  - id: reponse-1
    sous_rubrique: reponse
    question: "Quels aspects d'une réponse finale évaluer ?"
    choix:
      - "Seulement l'orthographe"
      - "Seulement la longueur"
      - "Seulement la vitesse"
      - "Exactitude, ancrage dans les sources, complétude, format, ton"
    reponse: 3
    pourquoi: >-
      Une réponse peut être juste mais incomplète.
      Complète mais inventée.
      Exacte mais dans le mauvais format.
      Chaque aspect se mesure à part.

  - id: reponse-2
    sous_rubrique: reponse
    question: "La réponse attendue est un montant : 1 240 €. Comment la vérifier ?"
    choix:
      - "Avec un LLM juge"
      - "Par une comparaison exacte dans le code, sans LLM"
      - "En demandant à l'utilisateur"
      - "On ne peut pas"
    reponse: 1
    pourquoi: >-
      Un chiffre se compare exactement.
      Le code est rapide, gratuit et déterministe.
      Un juge LLM ajouterait du bruit et un coût.
      On garde le juge pour ce que le code ne sait pas juger.

  - id: reponse-3
    sous_rubrique: reponse
    question: "Les évaluations de la mise à jour GPT-4o d'avril 2025 étaient bonnes. Pourquoi n'ont-elles pas vu la complaisance ?"
    choix:
      - "Aucune ne mesurait la complaisance"
      - "Elles étaient en panne"
      - "Le modèle trichait aux tests"
      - "Elles avaient été supprimées"
    reponse: 0
    pourquoi: >-
      OpenAI l'a écrit : pas d'évaluation dédiée à ce comportement.
      Les tests regardaient d'autres choses, qui allaient bien.
      Une évaluation ne voit que ce qu'elle mesure.
      Chaque risque connu mérite son propre test.

  - id: reponse-4
    sous_rubrique: reponse
    question: "Ton agent répond juste, mais cite une source qui ne dit pas ça. Bonne réponse ?"
    choix:
      - "Oui, personne ne vérifie les sources"
      - "Oui, la conclusion est juste"
      - "Non : l'ancrage est faux, même si la conclusion est juste"
      - "Ça dépend du modèle"
    reponse: 2
    pourquoi: >-
      L'utilisateur fait confiance à la citation.
      Une citation fausse trompe, même par hasard juste.
      Demain, la même mécanique donnera une conclusion fausse.
      On évalue l'ancrage à part de l'exactitude.

  - id: reponse-5
    sous_rubrique: reponse
    question: "Faut-il évaluer aussi les cas où l'agent devrait refuser ou dire « je ne sais pas » ?"
    choix:
      - "Non, seulement les cas normaux"
      - "Oui : un bon jeu de test contient des questions sans réponse et des demandes à refuser"
      - "Non, ça n'arrive jamais"
      - "Seulement en production"
    reponse: 1
    pourquoi: >-
      Un agent qui répond toujours paraît bon sur des cas faciles.
      Il invente sur les cas sans réponse.
      Sans ces cas dans le test, l'abstention n'est jamais mesurée.
      C'est le sujet de l'étape 3.

  - id: trajectoire-1
    sous_rubrique: trajectoire
    question: "Qu'est-ce que la trajectoire d'un agent ?"
    choix:
      - "Le prix de la requête"
      - "Sa réponse finale"
      - "Le temps de réponse"
      - "La suite de ses étapes : outils appelés, arguments, résultats, décisions"
    reponse: 3
    pourquoi: >-
      Deux agents peuvent donner la même réponse.
      L'un en 3 appels, l'autre en 40.
      L'un en respectant les règles, l'autre en les contournant.
      La trajectoire montre comment il est arrivé là.

  - id: trajectoire-2
    sous_rubrique: trajectoire
    question: "Un agent rembourse le bon montant, mais sans vérifier l'identité du client comme l'exige la règle. Le test doit-il passer ?"
    choix:
      - "Oui, si le client est content"
      - "Oui, le résultat est bon"
      - "Non : la trajectoire viole une règle, même si le résultat est bon"
      - "Ça dépend du montant"
    reponse: 2
    pourquoi: >-
      Cette fois, le client était le bon.
      La prochaine fois, ce sera un fraudeur.
      Le résultat seul cache le risque.
      La trajectoire le révèle.

  - id: trajectoire-3
    sous_rubrique: trajectoire
    question: "Comment τ-bench (2024) juge-t-il la réussite d'un agent de service client ?"
    choix:
      - "En comparant l'état final de la base de données à l'état attendu"
      - "En lisant la réponse finale à l'œil"
      - "En mesurant la longueur de la conversation"
      - "En demandant au modèle s'il a réussi"
    reponse: 0
    pourquoi: >-
      Ce qui compte, c'est ce qui a changé dans le système.
      La commande est-elle annulée ? Le bon article échangé ?
      L'état final se compare objectivement.
      Le discours de l'agent, lui, peut enjoliver.

  - id: trajectoire-4
    sous_rubrique: trajectoire
    question: "Que mesure pass^k dans τ-bench ?"
    choix:
      - "Le nombre d'outils utilisés"
      - "Le meilleur résultat sur k essais"
      - "La probabilité de réussir la même tâche les k fois sur k essais"
      - "Le coût de k requêtes"
    reponse: 2
    pourquoi: >-
      En production, une tâche se répète des milliers de fois.
      Réussir une fois ne suffit pas.
      pass^k demande de réussir à chaque fois.
      Dans le domaine retail, pass^8 tombait sous 25 % pour les meilleurs agents testés.

  - id: trajectoire-5
    sous_rubrique: trajectoire
    question: "Quels indicateurs de trajectoire suivre en plus de la réussite ?"
    choix:
      - "Nombre d'étapes, appels inutiles ou répétés, erreurs d'outil, coût, durée"
      - "Seulement la couleur de l'interface"
      - "Seulement le nombre de mots"
      - "Aucun"
    reponse: 0
    pourquoi: >-
      Un agent qui réussit en 40 étapes coûte 10 fois plus qu'en 4.
      Les appels répétés signalent une boucle.
      Les erreurs d'outil signalent un schéma mal compris.
      Ces chiffres guident les corrections.

  - id: trajectoire-6
    sous_rubrique: trajectoire
    question: "Pour un agent de recherche, Anthropic juge-t-il qu'il doit suivre un chemin unique fixé à l'avance ?"
    choix:
      - "Anthropic n'évalue pas ses agents"
      - "Oui, toujours le même chemin exact"
      - "Oui, sinon le test échoue"
      - "Non : des chemins différents peuvent être valides. On juge le résultat et le respect du processus"
    reponse: 3
    pourquoi: >-
      Deux agents peuvent trouver la même information par des sources différentes.
      Exiger un chemin exact punit des solutions valides.
      On vérifie plutôt les étapes obligatoires et les interdits.
      Le reste est libre.
---

## La réponse {#reponse}

On évalue d'abord ce que l'utilisateur reçoit.

| Aspect | Question | Comment |
|---|---|---|
| Exactitude | Est-ce juste ? | Code si possible, sinon juge |
| Ancrage | Les sources le disent-elles ? | Juge, vérification des citations |
| Complétude | Tout est-il couvert ? | Liste de points attendus |
| Format | JSON valide, champs présents ? | Code |
| Abstention | Refuse-t-il quand il le faut ? | Cas sans réponse dans le jeu de test |
| Comportement | Ton, complaisance, sécurité | Juge, tests dédiés |

:::pourquoi
Une réponse peut réussir un aspect et rater les autres.
Une note globale cache ce qui casse.
:::

Ce qui se vérifie par **code** se vérifie par code.

:::pourquoi
Un montant, un format, un champ : comparaison exacte.
Rapide, gratuit, déterministe.
Le juge LLM est pour le reste.
:::

Chaque risque connu a **son** test.

:::pourquoi
Les tests de GPT-4o étaient bons.
Aucun ne mesurait la complaisance.
Une évaluation ne voit que ce qu'elle mesure.
:::

## La trajectoire {#trajectoire}

La **trajectoire**, c'est le chemin : outils appelés, arguments, résultats, décisions.

```mermaid
sequenceDiagram
  participant U as Utilisateur simulé
  participant A as Agent
  participant T as Outils
  participant B as Base de données
  participant E as Évaluateur
  U->>A: « Annule ma commande 512 »
  A->>T: verifier_identite(client)
  A->>T: annuler_commande(512)
  T->>B: Commande 512 : annulée
  A-->>U: « C'est fait »
  E->>B: État final = état attendu ?
  E->>E: Étapes obligatoires présentes ? Interdits respectés ?
```

Un bon résultat ne suffit pas.

:::pourquoi
Un remboursement juste sans vérification d'identité passe aujourd'hui.
Demain, c'est un fraudeur qui en profite.
:::

τ-bench (2024) simule un client avec un modèle. Il juge l'agent sur l'**état final** de la base, comparé à l'état attendu.

:::pourquoi
Ce qui compte est ce qui a changé dans le système.
L'agent peut dire « c'est fait » sans l'avoir fait.
:::

Mesure la **régularité** : pass^k, réussir la même tâche k fois sur k.

:::pourquoi
Dans τ-bench, les meilleurs agents réussissaient moins de la moitié des tâches.
Dans le domaine retail, pass^8 tombait sous 25 %.
En production, la même tâche revient des milliers de fois.
:::

Suis aussi : nombre d'étapes, appels répétés, erreurs d'outil, coût, durée.

:::pourquoi
Ils expliquent pourquoi un agent échoue, ou réussit trop cher.
:::

Ne fige pas un chemin unique quand plusieurs sont valides.

:::pourquoi
Anthropic le note pour ses agents de recherche.
Juger le résultat et les règles du processus évite de punir une bonne solution différente.
:::

## À retenir

- Évaluer la réponse par aspects : exactitude, ancrage, complétude, format, abstention, comportement.
- Le code quand c'est possible. Un test par risque connu.
- Évaluer la trajectoire : état final, étapes obligatoires, interdits.
- Mesurer la régularité (pass^k), pas seulement le meilleur essai.
