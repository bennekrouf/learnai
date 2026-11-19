---
id: "05-04"
titre: "Humain dans la boucle"
sous_rubriques:
  - id: a-valider
    titre: "Étapes à valider"
  - id: decision
    titre: "Approuver ou rejeter"
sources:
  - titre: "Anthropic — Project Vend, 27 juin 2025"
    url: "https://www.anthropic.com/research/project-vend-1"
  - titre: "Anthropic — Building effective agents, 19 décembre 2024"
    url: "https://www.anthropic.com/engineering/building-effective-agents"
  - titre: "MCP — Tools, révision 2026-07-28 (human in the loop)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
qcm:
  - id: a-valider-1
    sous_rubrique: a-valider
    question: "Quelles étapes d'un agent faire valider par un humain ?"
    choix:
      - "Toutes, sans exception"
      - "L'irréversible, l'argent, ce qui sort vers l'extérieur, la production"
      - "Aucune, l'agent est autonome"
      - "Seulement les lectures de fichiers"
    reponse: 1
    pourquoi: >-
      Une erreur réversible se corrige.
      Un virement, un mail envoyé, une base effacée, non.
      On place l'humain là où l'erreur coûte cher.
      Ailleurs, il ralentit sans protéger.

  - id: a-valider-2
    sous_rubrique: a-valider
    question: "Claudius cédait aux demandes de remise sur Slack. Quelle barrière aurait aidé ?"
    choix:
      - "Toute remise au-delà d'un seuil passe en état « à valider » par un humain"
      - "Lui demander d'être plus ferme dans le prompt"
      - "Couper Slack"
      - "Interdire aux clients de parler"
    reponse: 0
    pourquoi: >-
      La pression des clients agissait sur le modèle.
      Un seuil dans le code ne se laisse pas convaincre.
      Les petites remises passent seules.
      Les grosses attendent un humain.

  - id: a-valider-3
    sous_rubrique: a-valider
    question: "Pourquoi ne pas tout faire valider, par prudence ?"
    choix:
      - "Parce que les humains sont toujours d'accord"
      - "Parce que c'est interdit"
      - "Trop de demandes, et l'humain finit par tout accepter sans lire"
      - "Parce que ça coûte des tokens"
    reponse: 2
    pourquoi: >-
      Cent validations par jour deviennent un réflexe.
      Le clic « approuver » ne veut plus rien dire.
      Moins de demandes, mieux choisies : chacune est vraiment lue.

  - id: a-valider-4
    sous_rubrique: a-valider
    question: "Comment régler le seuil de validation ?"
    choix:
      - "Le même pour tout, une fois pour toutes"
      - "Selon le risque : montant, réversibilité, destinataire, et l'historique des erreurs"
      - "Le modèle le choisit"
      - "Au hasard"
    reponse: 1
    pourquoi: >-
      Un remboursement de 5 € et un de 5 000 € n'ont pas le même risque.
      Un mail interne et un mail à un client non plus.
      On commence prudent.
      On relâche là où les validations sont toujours « oui ».

  - id: a-valider-5
    sous_rubrique: a-valider
    question: "L'agent attend une validation. L'humain répondra dans trois heures. Que devient la tâche ?"
    choix:
      - "Elle est annulée après une minute"
      - "Le processus reste bloqué trois heures"
      - "Elle continue sans attendre"
      - "Elle est sauvegardée dans l'état « en attente ». La réponse la relance"
    reponse: 3
    pourquoi: >-
      Une pause humaine peut durer des heures.
      Un checkpoint rend cette attente gratuite et sûre.
      La réponse arrive : on recharge l'état et on continue.
      C'est la durabilité au service de l'humain.

  - id: decision-1
    sous_rubrique: decision
    question: "Que montrer à l'humain qui doit approuver ?"
    choix:
      - "Tout l'historique brut de la conversation"
      - "« L'agent veut agir. OK ? »"
      - "L'action exacte, ses arguments, la raison donnée par l'agent, et l'effet attendu"
      - "Rien, pour aller vite"
    reponse: 2
    pourquoi: >-
      On ne juge que ce qu'on voit.
      « Rembourser » ne dit pas combien, ni à qui.
      La raison aide à repérer une erreur de logique.
      L'historique brut noie l'essentiel.

  - id: decision-2
    sous_rubrique: decision
    question: "Quelles réponses proposer à l'humain ?"
    choix:
      - "Approuver, rejeter avec une raison, ou modifier les arguments"
      - "Seulement « OK »"
      - "Seulement approuver ou ignorer"
      - "Un champ texte libre, sans boutons"
    reponse: 0
    pourquoi: >-
      Approuver : l'action part telle quelle.
      Rejeter : la raison guide l'agent.
      Modifier : l'humain corrige le montant sans tout refaire.
      Trois choix clairs, une décision traçable.

  - id: decision-3
    sous_rubrique: decision
    question: "L'humain rejette l'action. Que fait l'agent ?"
    choix:
      - "Il exécute quand même"
      - "Il réessaie la même action"
      - "Il reçoit le rejet et sa raison, et propose autre chose ou s'arrête"
      - "Il plante"
    reponse: 2
    pourquoi: >-
      Le rejet est une information.
      « Trop cher » invite à proposer moins.
      « Mauvais client » invite à vérifier.
      Réessayer la même action ignore l'humain.

  - id: decision-4
    sous_rubrique: decision
    question: "Personne ne répond à la demande de validation pendant 48 heures. Que faire ?"
    choix:
      - "Ne pas exécuter : expirer la demande et prévenir"
      - "Exécuter : qui ne dit mot consent"
      - "Attendre indéfiniment sans prévenir"
      - "Choisir au hasard"
    reponse: 0
    pourquoi: >-
      L'absence de réponse n'est pas un accord.
      Par défaut, l'action sensible n'a pas lieu.
      On expire la demande, on alerte, on trace.
      C'est le principe d'échec sûr.

  - id: decision-5
    sous_rubrique: decision
    question: "L'humain approuve un remboursement de 50 €. Ensuite, l'agent change le montant à 500 €. Que faire ?"
    choix:
      - "Exécuter les deux"
      - "Exécuter, c'est déjà approuvé"
      - "Exécuter 50 € et ignorer le changement sans rien dire"
      - "Redemander : l'approbation portait sur 50 €, pas sur 500 €"
    reponse: 3
    pourquoi: >-
      Une approbation vaut pour une action précise.
      On la lie aux arguments exacts, par exemple avec une empreinte.
      Si les arguments changent, l'approbation ne vaut plus.
      Sinon, on approuve une chose et on en exécute une autre.

  - id: decision-6
    sous_rubrique: decision
    question: "Que journaliser pour chaque validation ?"
    choix:
      - "Rien, c'est privé"
      - "Qui a décidé, quand, sur quelle action exacte, et quelle décision"
      - "Seulement les refus"
      - "Seulement l'heure"
    reponse: 1
    pourquoi: >-
      En cas d'incident, on doit savoir qui a validé quoi.
      Les validations toujours positives montrent où relâcher le seuil.
      Les refus fréquents montrent où l'agent se trompe.
---

## Étapes à valider {#a-valider}

Certaines étapes passent par un **humain** avant d'agir.

Lesquelles :

- l'irréversible : supprimer, envoyer, publier ;
- l'argent : payer, rembourser, accorder une remise ;
- ce qui sort : un mail, un message, une requête vers l'extérieur ;
- la production.

:::pourquoi
Une erreur réversible se corrige.
Un virement parti, non.
L'humain se place là où l'erreur coûte cher.
:::

Le seuil dépend du **risque** : montant, réversibilité, destinataire.

:::pourquoi
Claudius cédait aux demandes de remise sur Slack.
Un seuil dans le code ne se laisse pas convaincre.
Au-delà, l'humain décide.
:::

Pas de validation partout.

:::pourquoi
Trop de demandes, et l'humain clique « oui » par réflexe.
La validation perd toute valeur.
:::

Dans la machine à états, la validation est un **état** : « en attente ». La tâche est sauvegardée et le processus libéré.

:::pourquoi
Un humain peut répondre dans trois heures.
Un checkpoint rend cette attente gratuite.
:::

## Approuver ou rejeter {#decision}

```mermaid
sequenceDiagram
  participant M as Modèle
  participant O as Orchestrateur
  participant S as Stockage
  participant H as Humain
  M->>O: rembourser(client=812, montant=500 €)
  O->>O: Garde : montant > seuil de 100 €
  O->>S: État « en attente », action et empreinte des arguments
  O->>H: Action, arguments, raison de l'agent. Approuver, rejeter, modifier ?
  H-->>O: Modifié : 200 €
  O->>S: Charge l'état, enregistre la décision
  O->>M: Approuvé avec modification : 200 €
```

Montre l'action **exacte** : outil, arguments, raison donnée par l'agent, effet attendu.

:::pourquoi
« Rembourser » ne dit ni combien, ni à qui.
On ne juge que ce qu'on voit.
:::

Trois réponses : **approuver**, **rejeter** avec une raison, **modifier**.

:::pourquoi
La raison d'un rejet guide l'agent.
La modification évite de tout recommencer.
:::

Un rejet est renvoyé au modèle. Il propose autre chose, ou s'arrête.

:::pourquoi
Réessayer la même action ignore l'humain.
:::

L'approbation vaut pour **ces** arguments. S'ils changent, on redemande.

:::pourquoi
Sinon on approuve 50 € et on exécute 500 €.
Une empreinte des arguments lie la décision à l'action.
:::

Pas de réponse ? L'action **n'a pas lieu**.

:::pourquoi
Le silence n'est pas un accord.
La demande expire, on prévient, on trace.
:::

Journalise chaque décision : qui, quand, quoi, quelle réponse.

:::pourquoi
En cas d'incident, on sait qui a validé quoi.
Et les statistiques montrent où ajuster le seuil.
:::

## À retenir

- Valider l'irréversible, l'argent, la sortie externe, la production. Pas le reste.
- La validation est un état sauvegardé, pas un processus qui attend.
- Montrer l'action exacte. Approuver, rejeter, modifier.
- Approbation liée aux arguments. Sans réponse, rien ne part. Tout est journalisé.
