---
id: "01-02"
titre: "Assemblage par priorité"
sous_rubriques:
  - id: couper
    titre: "Que couper d'abord"
  - id: resume
    titre: "Troncature et résumé"
  - id: debordement
    titre: "Prévenir le débordement"
sources:
  - titre: "Anthropic — Prompting best practices, section long context (documentation)"
    url: "https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices"
  - titre: "Microsoft Bing Blog (relayé par Tom's Guide) — limite de 5 tours par session, février 2023"
    url: "https://www.tomsguide.com/news/bing-with-chatgpt-is-getting-limited-to-just-5-questions-per-session"
qcm:
  - id: couper-1
    sous_rubrique: couper
    question: "Le contexte déborde. Que ne faut-il jamais couper ?"
    choix:
      - "Les consignes système et la question actuelle"
      - "Les vieux tours de conversation"
      - "Les documents les moins pertinents"
      - "Les anciens résultats d'outils"
    reponse: 0
    pourquoi: >-
      Les consignes disent au modèle qui il est et ce qu'il doit respecter.
      La question dit ce qu'on attend maintenant.
      Sans l'une ou l'autre, la réponse n'a plus de sens.
      Le reste peut se résumer ou se couper.

  - id: couper-2
    sous_rubrique: couper
    question: "Quoi couper en premier ?"
    choix:
      - "La question de l'utilisateur"
      - "Les consignes de sécurité"
      - "Le dernier message"
      - "Les vieux tours et les gros résultats d'outils déjà exploités"
    reponse: 3
    pourquoi: >-
      Un vieux tour a déjà servi.
      Un résultat d'outil exploité a déjà donné sa conclusion.
      Ce sont les morceaux qui ont le moins de valeur pour la suite.
      Ils prennent pourtant souvent le plus de place.

  - id: couper-3
    sous_rubrique: couper
    question: "Un long document et une question. Dans quel ordre les placer ?"
    choix:
      - "La question d'abord, le document ensuite"
      - "Le document d'abord, la question à la fin"
      - "La question au milieu du document"
      - "Aucune importance"
    reponse: 1
    pourquoi: >-
      La fin du contexte est une zone bien lue.
      La question à la fin reste fraîche au moment de répondre.
      La documentation d'Anthropic recommande cet ordre.
      Selon ses tests, la qualité peut gagner jusqu'à 30 %.

  - id: resume-1
    sous_rubrique: resume
    question: "Troncature ou résumé : quel est l'avantage de la troncature ?"
    choix:
      - "Elle garde l'essentiel"
      - "Elle améliore la réponse"
      - "Elle est gratuite et prévisible"
      - "Elle garde les chiffres exacts de ce qui est coupé"
    reponse: 2
    pourquoi: >-
      Couper ne demande aucun appel au modèle.
      On sait exactement ce qui reste.
      Mais ce qui est coupé est perdu en entier.

  - id: resume-2
    sous_rubrique: resume
    question: "Tu résumes une conversation ancienne. Qu'est-ce qui risque de disparaître ?"
    choix:
      - "Le thème général"
      - "Rien, un résumé garde tout"
      - "La langue de la conversation"
      - "Les détails exacts : chiffres, identifiants, noms"
    reponse: 3
    pourquoi: >-
      Un résumé garde le sens, pas la lettre.
      « Commande 48213 de 312,50 € » devient « une commande ».
      Garde à part, mot pour mot, les éléments précis encore utiles.

  - id: resume-3
    sous_rubrique: resume
    question: "Tu coupes l'historique. Tu supprimes un résultat d'outil mais pas la demande d'outil qui va avec. Conséquence ?"
    choix:
      - "L'API peut rejeter la requête"
      - "Le modèle redemande l'outil"
      - "Aucune"
      - "Le modèle invente le résultat"
    reponse: 0
    pourquoi: >-
      Une demande d'outil et son résultat forment une paire.
      Les API vérifient que chaque résultat répond à une demande.
      Une paire cassée rend l'historique invalide.
      Coupe des tours entiers, jamais la moitié d'une paire.

  - id: debordement-1
    sous_rubrique: debordement
    question: "Quand faut-il déclencher le résumé de l'historique ?"
    choix:
      - "Quand l'API renvoie une erreur"
      - "Jamais, on laisse l'API gérer"
      - "À un seuil fixé à l'avance, avant d'atteindre la limite"
      - "À chaque message"
    reponse: 2
    pourquoi: >-
      Attendre l'erreur, c'est échouer devant l'utilisateur.
      Résumer à chaque message coûte trop cher.
      Un seuil, par exemple 80 % du budget, laisse le temps d'agir.

  - id: debordement-2
    sous_rubrique: debordement
    question: "Un outil renvoie 300 000 tokens de logs. Que faire ?"
    choix:
      - "Tout envoyer au modèle"
      - "Limiter la taille du résultat, et proposer de lire la suite"
      - "Ignorer l'outil"
      - "Changer de modèle"
    reponse: 1
    pourquoi: >-
      Un seul résultat ferait déborder la fenêtre.
      On limite la taille et on indique que c'est tronqué.
      On donne un moyen de lire la suite : page suivante, filtre, recherche.
      Le modèle demandera ce dont il a besoin.

  - id: debordement-3
    sous_rubrique: debordement
    question: "Février 2023 : Bing Chat a des réponses inquiétantes. Que fait Microsoft ?"
    choix:
      - "Il change de modèle"
      - "Il supprime l'historique"
      - "Il limite chaque session à 5 tours"
      - "Il ferme le service"
    reponse: 2
    pourquoi: >-
      Microsoft explique que les très longues sessions peuvent embrouiller le modèle.
      Il plafonne à 5 tours par session et 50 par jour.
      Quelques jours plus tard, il passe à 6 tours.
      C'est la prévention la plus brutale : couper avant le problème.
---

## Que couper d'abord {#couper}

Le contexte est trop grand. Il faut choisir.

On classe chaque morceau par priorité.

| Priorité | Morceau | Pourquoi |
|---|---|---|
| Jamais coupé | Consignes système, question actuelle | Sans eux, plus de tâche |
| Haute | Derniers tours, résultats d'outils récents | La tâche en cours en dépend |
| Moyenne | Documents pertinents, mémoire utile | Ils aident, mais se trient |
| Basse | Vieux tours, résultats d'outils déjà exploités | Ils ont déjà servi |

:::pourquoi
Un morceau vaut ce qu'il apporte à la prochaine réponse.
Un vieux résultat d'outil est souvent gros et déjà résumé par le modèle lui-même.
:::

Plusieurs sources alimentent le contexte. Un **assembleur** les trie.

```mermaid
flowchart LR
  C["Consignes"] --> A["Assembleur"]
  H["Historique"] --> A
  M["Mémoire"] --> A
  R["Recherche"] --> A
  O["Résultats d'outils"] --> A
  A -->|"tri, coupe, résumé"| P["Prompt final"]
  P --> L["LLM"]
```

:::pourquoi
Chaque source ignore les autres.
L'assembleur est le seul à voir le budget total.
C'est lui qui arbitre.
:::

L'ordre compte : longs documents en haut, question à la fin.

:::pourquoi
La fin est bien lue. La question reste fraîche au moment de répondre.
L'aiguille dans la botte de foin a montré que la présentation change le résultat.
:::

## Troncature et résumé {#resume}

Deux façons de réduire.

**Troncature** : on coupe. Gratuit, prévisible. Ce qui est coupé est perdu.

**Résumé** : on demande au modèle de condenser. Ça coûte un appel. Les détails exacts partent.

:::pourquoi
Un résumé garde le sens, pas les chiffres.
« Commande 48213, 312,50 € » devient « une commande ».
Garde mot pour mot les identifiants, montants et dates encore utiles.
:::

On combine souvent les deux : résumé des vieux tours, troncature des gros résultats.

Coupe par tours entiers. Jamais une demande d'outil sans son résultat.

:::pourquoi
Les API vérifient que chaque résultat répond à une demande.
Une paire cassée rend l'historique invalide.
:::

## Prévenir le débordement {#debordement}

Compte **avant** d'envoyer. Pas après l'erreur.

Fixe un seuil, par exemple 80 % du budget. Au-delà, on compacte.

:::pourquoi
L'erreur arrive devant l'utilisateur.
Le seuil laisse le temps de résumer proprement.
:::

Limite la taille de chaque résultat d'outil. Indique qu'il est tronqué. Donne un moyen de lire la suite.

:::pourquoi
Un seul résultat énorme peut tout chasser.
Le modèle sait demander une page de plus si on lui dit que c'est possible.
:::

En février 2023, Microsoft limite Bing Chat à 5 tours par session.

:::pourquoi
Selon Microsoft, les très longues sessions pouvaient embrouiller le modèle.
Couper la session est la prévention la plus simple.
Mais c'est l'utilisateur qui en paie le prix.
Un bon assembleur fait mieux : il compacte sans tout effacer.
:::

## À retenir

- Jamais coupé : consignes et question actuelle.
- Coupé d'abord : vieux tours et résultats d'outils déjà exploités.
- Résumé pour le sens, mot pour mot pour les détails exacts.
- Compte avant d'envoyer. Compacte à un seuil.
