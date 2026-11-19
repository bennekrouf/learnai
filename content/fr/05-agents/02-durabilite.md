---
id: "05-02"
titre: "Durabilité"
sous_rubriques:
  - id: checkpoint
    titre: "Checkpoint"
  - id: reprise
    titre: "Reprise après crash"
  - id: idempotence
    titre: "Idempotence"
sources:
  - titre: "Anthropic — How we built our multi-agent research system, 2025 (reprise, checkpoints, rainbow deployments)"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Stripe — Idempotent requests"
    url: "https://docs.stripe.com/api/idempotent_requests"
  - titre: "AWS Builders' Library — Making retries safe with idempotent APIs"
    url: "https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/"
qcm:
  - id: checkpoint-1
    sous_rubrique: checkpoint
    question: "Qu'est-ce qu'un checkpoint pour un agent ?"
    choix:
      - "Un message de log"
      - "Une sauvegarde de son état, après chaque étape, dans un stockage durable"
      - "Un test unitaire"
      - "Un résumé envoyé à l'utilisateur"
    reponse: 1
    pourquoi: >-
      Le processus peut mourir à tout moment.
      Ce qui n'est qu'en mémoire disparaît avec lui.
      Le checkpoint écrit l'état sur disque ou en base.
      On peut repartir de là.

  - id: checkpoint-2
    sous_rubrique: checkpoint
    question: "Que doit contenir un checkpoint ?"
    choix:
      - "Le prompt système uniquement"
      - "Seulement le dernier message du modèle"
      - "Seulement l'heure"
      - "L'état courant, le contexte utile, les résultats d'outils déjà obtenus, et la version du format"
    reponse: 3
    pourquoi: >-
      L'état dit où on en est.
      Les résultats d'outils évitent de refaire le travail.
      Le contexte permet au modèle de continuer.
      La version permet de relire un ancien checkpoint après une mise à jour.

  - id: checkpoint-3
    sous_rubrique: checkpoint
    question: "Où stocker les checkpoints ?"
    choix:
      - "Dans une variable du processus"
      - "Dans un stockage durable : base de données ou fichiers persistants"
      - "Dans le contexte du modèle"
      - "Dans le cache du navigateur"
    reponse: 1
    pourquoi: >-
      Le checkpoint doit survivre au crash.
      Une variable meurt avec le processus.
      Le contexte du modèle n'est pas un stockage.
      Une base de données, si.

  - id: checkpoint-4
    sous_rubrique: checkpoint
    question: "L'agent attend la validation d'un humain, qui répondra demain. Que faire du processus ?"
    choix:
      - "Sauvegarder l'état et libérer le processus. On reprendra à la réponse"
      - "Garder le processus en attente jusqu'à demain"
      - "Annuler la tâche"
      - "Valider à la place de l'humain"
    reponse: 0
    pourquoi: >-
      Un processus qui attend une journée peut redémarrer entre-temps.
      Il occupe aussi des ressources pour rien.
      Avec un checkpoint, l'attente ne coûte rien.
      La réponse de l'humain relance la tâche depuis la sauvegarde.

  - id: reprise-1
    sous_rubrique: reprise
    question: "Une tâche de deux heures plante à 1 h 50. Sans checkpoint, que se passe-t-il ?"
    choix:
      - "Le modèle se souvient de tout"
      - "Elle reprend à 1 h 50"
      - "Tout recommence depuis le début"
      - "Rien, la tâche est considérée comme finie"
    reponse: 2
    pourquoi: >-
      Sans sauvegarde, il n'y a rien à reprendre.
      On repaie le temps et les tokens.
      Anthropic l'écrit : pour un agent long, redémarrer de zéro coûte cher et frustre.
      Ils reprennent depuis l'endroit où l'erreur est arrivée.

  - id: reprise-2
    sous_rubrique: reprise
    question: "Que fait l'agent quand il reprend après un crash ?"
    choix:
      - "Il refait toutes les étapes, pour être sûr"
      - "Il recharge le dernier checkpoint et continue à l'étape suivante"
      - "Il demande au modèle de deviner où il en était"
      - "Il saute la tâche"
    reponse: 1
    pourquoi: >-
      Les étapes sauvegardées ne sont pas refaites.
      Leurs résultats sont déjà là.
      Seule l'étape interrompue est rejouée.
      C'est pour elle qu'il faut l'idempotence.

  - id: reprise-3
    sous_rubrique: reprise
    question: "Tu déploies une nouvelle version pendant que des agents tournent. Comment faisait Anthropic ?"
    choix:
      - "On change le code sous les agents en cours"
      - "On coupe tout et on relance"
      - "On attend la nuit"
      - "Les anciennes et nouvelles versions tournent ensemble. Le trafic passe peu à peu sur la nouvelle"
    reponse: 3
    pourquoi: >-
      Un agent en cours suit le code avec lequel il a commencé.
      Le changer en route casse son état.
      Anthropic appelle ça des « rainbow deployments ».
      Les tâches en cours finissent sur l'ancienne version.

  - id: reprise-4
    sous_rubrique: reprise
    question: "Un checkpoint d'hier a un ancien format d'état. Le nouveau code doit le relire. Que prévoir ?"
    choix:
      - "Laisser le modèle interpréter"
      - "Supprimer les anciens checkpoints"
      - "Un numéro de version dans le checkpoint, et une migration"
      - "Rien, les formats ne changent jamais"
    reponse: 2
    pourquoi: >-
      Le code évolue. Les sauvegardes restent.
      La version dit comment lire le checkpoint.
      Une migration le convertit au nouveau format.
      Sans ça, la reprise plante au pire moment.

  - id: idempotence-1
    sous_rubrique: idempotence
    question: "Qu'est-ce qu'une opération idempotente ?"
    choix:
      - "La faire deux fois a le même effet que la faire une fois"
      - "Elle est très rapide"
      - "Elle ne peut jamais échouer"
      - "Elle ne coûte rien"
    reponse: 0
    pourquoi: >-
      « Mettre le statut à payé » : deux fois, même résultat.
      « Ajouter 10 € » : deux fois, 20 €.
      La première est idempotente. La seconde non.
      Rejouer une étape n'est sûr que si elle l'est.

  - id: idempotence-2
    sous_rubrique: idempotence
    question: "L'agent envoie un paiement. Il plante avant d'enregistrer le checkpoint. À la reprise, quel risque ?"
    choix:
      - "Perdre le modèle"
      - "Aucun"
      - "Payer deux fois"
      - "Un timeout"
    reponse: 2
    pourquoi: >-
      Le paiement est parti.
      Le checkpoint ne le sait pas.
      La reprise rejoue l'étape.
      Sans protection, le client est débité deux fois.

  - id: idempotence-3
    sous_rubrique: idempotence
    question: "Comment Stripe évite-t-il les doubles paiements quand un client réessaie ?"
    choix:
      - "Le client envoie une clé d'idempotence. Même clé, même résultat, sans nouveau débit"
      - "Il bloque tous les réessais"
      - "Il demande une confirmation par SMS"
      - "Il rembourse automatiquement le double"
    reponse: 0
    pourquoi: >-
      La clé accompagne la requête, dans un en-tête.
      Stripe mémorise le résultat associé à la clé.
      Une seconde requête avec la même clé reçoit ce résultat.
      L'opération n'est pas refaite.

  - id: idempotence-4
    sous_rubrique: idempotence
    question: "Comment construire la clé d'idempotence d'une étape d'agent ?"
    choix:
      - "Le texte de la réponse du modèle"
      - "Un nombre aléatoire nouveau à chaque tentative"
      - "L'heure actuelle"
      - "La générer une fois et la sauvegarder avant l'appel, ou la dériver de l'id de la tâche et de l'étape"
    reponse: 3
    pourquoi: >-
      La clé doit rester la même d'une tentative à l'autre.
      Une clé neuve à chaque essai ne protège de rien.
      L'id de tâche plus le numéro d'étape est stable.
      Le texte du modèle, lui, change à chaque génération.

  - id: idempotence-5
    sous_rubrique: idempotence
    question: "L'API d'envoi d'e-mail n'accepte pas de clé d'idempotence. Que faire ?"
    choix:
      - "Envoyer et espérer"
      - "Enregistrer « envoi prévu » avec un id, vérifier avant d'envoyer, puis marquer « envoyé »"
      - "Ne jamais réessayer aucune étape"
      - "Envoyer deux fois pour être sûr"
    reponse: 1
    pourquoi: >-
      On construit l'idempotence de notre côté.
      Avant l'envoi, on note l'intention avec un id.
      À la reprise, on regarde si cet id est déjà marqué envoyé.
      Il reste une petite fenêtre de doute : on la réduit, on la journalise.
---

## Checkpoint {#checkpoint}

Un agent peut tourner des heures. Le processus peut mourir à tout moment.

Un **checkpoint** sauvegarde l'état après chaque étape. Dans un stockage durable.

:::pourquoi
Ce qui n'est qu'en mémoire meurt avec le processus.
Une base de données survit au crash.
:::

Ce qu'il contient :

| Élément | Sert à |
|---|---|
| État courant | Savoir où on en est |
| Résultats d'outils | Ne pas refaire le travail |
| Contexte utile | Laisser le modèle continuer |
| Version du format | Relire après une mise à jour |

:::pourquoi
Chaque élément manquant oblige à refaire ou à deviner.
:::

Un checkpoint rend aussi l'**attente** gratuite. L'agent attend un humain ? On sauvegarde, on libère le processus.

:::pourquoi
Un processus qui attend une journée occupe des ressources.
Et il peut redémarrer entre-temps.
:::

## Reprise après crash {#reprise}

Reprendre, c'est recharger le dernier checkpoint et continuer.

```mermaid
sequenceDiagram
  participant A as Agent (processus 1)
  participant S as Stockage des checkpoints
  participant B as Agent (processus 2)
  A->>S: Étape 1 finie, sauvegarde
  A->>S: Étape 2 finie, sauvegarde
  A->>A: Crash pendant l'étape 3
  B->>S: Charge le dernier checkpoint
  S-->>B: État après l'étape 2
  B->>B: Rejoue l'étape 3 seulement
  B->>S: Étape 3 finie, sauvegarde
```

:::pourquoi
Anthropic l'écrit pour son système de recherche.
Redémarrer un agent long depuis zéro coûte cher et frustre.
Ils reprennent là où l'erreur est arrivée.
:::

Les déploiements aussi interrompent. Anthropic fait tourner l'ancienne et la nouvelle version côte à côte, et bascule le trafic peu à peu.

:::pourquoi
Un agent en cours suit le code avec lequel il a commencé.
Changer ce code en route peut casser son état.
:::

Mets un **numéro de version** dans chaque checkpoint.

:::pourquoi
Le code change. Les sauvegardes restent.
La version dit comment relire, et quelle migration appliquer.
:::

## Idempotence {#idempotence}

À la reprise, l'étape interrompue est **rejouée**. Elle a peut-être déjà agi.

```mermaid
sequenceDiagram
  participant A as Agent
  participant S as Stockage
  participant P as API de paiement
  A->>S: Étape « payer », clé tache-42-etape-3
  A->>P: Paiement 50 €, clé tache-42-etape-3
  P-->>A: Paiement accepté
  A->>A: Crash avant la sauvegarde
  A->>P: Reprise : même paiement, même clé
  P-->>A: Résultat déjà connu, pas de nouveau débit
  A->>S: Étape « payer » terminée
```

Une opération **idempotente** a le même effet faite une ou deux fois.

:::pourquoi
« Mettre le statut à payé » : deux fois, même résultat.
« Ajouter 10 € » : deux fois, 20 €.
Seule une étape idempotente se rejoue sans risque.
:::

Pour les API qui le permettent, envoie une **clé d'idempotence**. Stripe, par exemple, la reçoit dans un en-tête et renvoie le même résultat pour la même clé.

:::pourquoi
Le serveur reconnaît la requête déjà traitée.
Il ne la refait pas.
:::

La clé reste la **même** d'une tentative à l'autre : id de la tâche plus numéro d'étape, ou générée une fois et sauvegardée avant l'appel.

:::pourquoi
Une clé neuve à chaque essai ne protège de rien.
Le serveur verrait deux requêtes différentes.
:::

Sans clé côté API, construis l'idempotence toi-même : note l'intention, vérifie avant d'agir, marque comme fait.

:::pourquoi
Le registre dit si l'action a déjà eu lieu.
Une petite fenêtre de doute reste. On la journalise.
:::

## À retenir

- Checkpoint après chaque étape, en stockage durable, avec une version.
- Reprise : recharger, puis rejouer seulement l'étape interrompue.
- Toute étape rejouable doit être idempotente.
- Clé d'idempotence stable : même clé pour chaque tentative.
