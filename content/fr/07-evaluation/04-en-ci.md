---
id: "07-04"
titre: "En CI"
sous_rubriques:
  - id: regressions
    titre: "Régressions"
  - id: bloquer-pr
    titre: "Bloquer une PR"
sources:
  - titre: "Anthropic — A statistical approach to model evaluations, 19 novembre 2024"
    url: "https://www.anthropic.com/research/statistical-approach-to-model-evals"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, 2 mai 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
qcm:
  - id: regressions-1
    sous_rubrique: regressions
    question: "Qu'est-ce qu'une régression pour un agent ?"
    choix:
      - "Un nouveau cas ajouté au jeu de test"
      - "Un agent plus lent"
      - "Une erreur de syntaxe"
      - "Un cas qui réussissait avant un changement et qui échoue après"
    reponse: 3
    pourquoi: >-
      On change un prompt pour corriger un cas.
      Trois autres cas se mettent à échouer.
      C'est le risque principal de chaque modification.
      La CI doit le voir avant la fusion.

  - id: regressions-2
    sous_rubrique: regressions
    question: "Quels changements doivent relancer l'évaluation ?"
    choix:
      - "Seulement le code Python"
      - "Prompt, modèle ou sa version, outils, schémas, code de l'agent, paramètres de recherche"
      - "Seulement le modèle"
      - "Seulement la documentation"
    reponse: 1
    pourquoi: >-
      Le comportement dépend de tout cela.
      Une description d'outil modifiée change les appels.
      Un nouveau modèle change tout.
      Le prompt est du code : il se teste comme du code.

  - id: regressions-3
    sous_rubrique: regressions
    question: "Le score global passe de 82 % à 81 %. Que regarder ?"
    choix:
      - "La marge d'erreur, et la liste des cas qui ont changé dans chaque sens"
      - "Rien, c'est à peu près pareil"
      - "Seulement le chiffre"
      - "La couleur du badge"
    reponse: 0
    pourquoi: >-
      Un point peut être du bruit.
      Mais il peut cacher 10 cas gagnés et 11 perdus.
      Parmi les perdus, peut-être un cas critique.
      La liste cas par cas dit ce qui a vraiment bougé.

  - id: regressions-4
    sous_rubrique: regressions
    question: "Les sorties du modèle varient d'une exécution à l'autre. Comment rendre la CI moins bruitée ?"
    choix:
      - "Supprimer les cas qui varient"
      - "Relancer jusqu'à ce que ça passe"
      - "Plusieurs tirages par cas, une moyenne par cas, et une comparaison appariée entre versions"
      - "Ignorer le bruit"
    reponse: 2
    pourquoi: >-
      Anthropic recommande de rééchantillonner chaque question.
      La moyenne par cas réduit la variance.
      Comparer les mêmes cas entre versions retire la difficulté commune.
      Relancer jusqu'au vert cache les vrais problèmes.

  - id: regressions-5
    sous_rubrique: regressions
    question: "Faut-il lancer tout le jeu d'évaluation à chaque commit ?"
    choix:
      - "Oui, toujours tout, quel que soit le coût"
      - "Un sous-ensemble rapide à chaque commit, le jeu complet avant la fusion ou la mise en production"
      - "Non, jamais"
      - "Seulement une fois par an"
    reponse: 1
    pourquoi: >-
      Un jeu complet avec juge LLM coûte du temps et des tokens.
      Un petit jeu rapide donne un retour immédiat.
      Le jeu complet protège les moments qui comptent.
      Le coût de l'évaluation fait partie de la conception.

  - id: bloquer-pr-1
    sous_rubrique: bloquer-pr
    question: "Quel genre de règle doit bloquer une PR ?"
    choix:
      - "L'avis du modèle sur sa propre PR"
      - "N'importe quelle baisse d'un dixième de point"
      - "Rien ne doit bloquer"
      - "Un cas critique qui échoue, ou une baisse significative sur un critère au-delà de la marge d'erreur"
    reponse: 3
    pourquoi: >-
      Certains cas sont non négociables : sécurité, argent, données personnelles.
      Pour le reste, une baisse dans le bruit ne doit pas bloquer.
      Sinon les développeurs apprennent à ignorer la CI.
      La règle doit être juste pour être respectée.

  - id: bloquer-pr-2
    sous_rubrique: bloquer-pr
    question: "OpenAI a lancé GPT-4o malgré l'impression négative de testeurs experts. Quel changement de processus annonce-t-il ?"
    choix:
      - "Ne plus faire de tests A/B"
      - "Supprimer les testeurs"
      - "Traiter les problèmes de comportement comme bloquants pour une sortie, comme d'autres risques de sécurité"
      - "Lancer plus vite"
    reponse: 2
    pourquoi: >-
      Le signal existait, mais il ne bloquait rien.
      Un test qui ne peut pas bloquer n'est qu'une opinion.
      OpenAI rend ces problèmes bloquants.
      La leçon vaut pour une CI : définir ce qui arrête une sortie.

  - id: bloquer-pr-3
    sous_rubrique: bloquer-pr
    question: "Une PR améliore 15 cas et en casse 2 cas critiques. On fusionne ?"
    choix:
      - "Non : un cas critique cassé bloque, quel que soit le gain ailleurs"
      - "Oui, le bilan est positif"
      - "Oui, si le score global monte"
      - "Ça dépend de l'humeur"
    reponse: 0
    pourquoi: >-
      Une moyenne échange un risque contre un confort.
      Un remboursement sans vérification ne se compense pas par des réponses plus jolies.
      Les cas critiques sont des garde-fous, pas des points.

  - id: bloquer-pr-4
    sous_rubrique: bloquer-pr
    question: "Que doit afficher le rapport d'évaluation sur la PR ?"
    choix:
      - "« OK » ou « KO » sans détail"
      - "Un seul pourcentage"
      - "Les scores par critère avec marge d'erreur, les cas gagnés et perdus, et un lien vers chaque trajectoire"
      - "Les logs bruts complets"
    reponse: 2
    pourquoi: >-
      Le relecteur doit comprendre en une minute.
      Par critère : on voit ce qui bouge.
      Cas perdus : on sait quoi regarder.
      Trajectoires : on voit pourquoi.

  - id: bloquer-pr-5
    sous_rubrique: bloquer-pr
    question: "Qui peut passer outre un blocage de la CI d'évaluation ?"
    choix:
      - "Une personne désignée, avec une justification écrite et tracée"
      - "N'importe qui, sans trace"
      - "Le modèle"
      - "Personne, jamais, même pour un faux positif prouvé"
    reponse: 0
    pourquoi: >-
      Un juge peut se tromper, un cas peut être mal écrit.
      Il faut une sortie de secours.
      Mais elle doit être rare, justifiée et visible.
      Sinon le blocage ne veut plus rien dire.
---

## Régressions {#regressions}

Chaque changement peut casser ce qui marchait. C'est une **régression**.

Ce qui doit relancer l'évaluation :

- le prompt ;
- le modèle ou sa version ;
- les outils et leurs descriptions ;
- les schémas ;
- le code de l'agent ;
- les paramètres de recherche.

:::pourquoi
Le comportement dépend de tout cela.
Le prompt est du code. Il se teste comme du code.
:::

Regarde les **cas qui changent**, pas seulement le score.

| Cas | Avant | Après |
|---|---|---|
| remb-017 | réussi | raté |
| remb-022 | raté | réussi |
| secu-004 (critique) | réussi | raté |

:::pourquoi
82 % puis 81 % peut cacher 10 cas gagnés et 11 perdus.
Dont peut-être un cas critique.
:::

Réduis le **bruit** : plusieurs tirages par cas, moyenne par cas, comparaison appariée.

:::pourquoi
Les sorties varient d'une exécution à l'autre.
Anthropic recommande de rééchantillonner, d'afficher une marge d'erreur, et de comparer les mêmes cas entre versions.
:::

Deux niveaux : un **petit jeu rapide** à chaque commit, le **jeu complet** avant la fusion ou la mise en production.

:::pourquoi
Le jeu complet avec juge coûte du temps et des tokens.
Le petit jeu donne un retour immédiat.
:::

## Bloquer une PR {#bloquer-pr}

Une évaluation qui ne peut rien **bloquer** n'est qu'une opinion.

:::pourquoi
Chez OpenAI, des testeurs avaient senti un problème.
Leur avis ne bloquait pas la sortie.
OpenAI traite désormais ce genre de problème comme bloquant.
:::

```mermaid
sequenceDiagram
  participant D as Développeur
  participant G as Dépôt (PR)
  participant C as CI d'évaluation
  participant J as LLM juge
  participant R as Relecteur
  D->>G: Ouvre une PR (nouveau prompt)
  G->>C: Lance le jeu complet
  C->>C: Contrôles par code (état final, DAG, format)
  C->>J: Critères non vérifiables par code
  J-->>C: Verdicts et justifications
  C->>C: Compare à la branche principale, cas par cas
  C-->>G: Bloqué : secu-004 (critique) a régressé
  G-->>R: Rapport : critères, cas perdus, trajectoires
```

Deux règles de blocage :

1. Un **cas critique** échoue : sécurité, argent, données personnelles.
2. Un critère **baisse au-delà** de sa marge d'erreur.

:::pourquoi
Un cas critique ne se compense pas par 15 cas améliorés.
Une baisse dans le bruit ne doit pas bloquer, sinon on apprend à ignorer la CI.
:::

Le **rapport** sur la PR : scores par critère avec marge d'erreur, cas gagnés et perdus, lien vers chaque trajectoire.

:::pourquoi
Le relecteur doit comprendre en une minute ce qui a bougé, et pourquoi.
:::

Une **sortie de secours** : une personne désignée peut passer outre, avec une justification écrite et tracée.

:::pourquoi
Un cas peut être mal écrit. Un juge peut se tromper.
Mais un contournement invisible tue la règle.
:::

## À retenir

- Tout changement de prompt, modèle, outil ou schéma relance l'évaluation.
- Regarder les cas qui changent, avec une marge d'erreur.
- Petit jeu rapide à chaque commit, jeu complet avant fusion.
- Bloquer sur un cas critique ou une baisse significative. Rapport lisible. Contournement tracé.
