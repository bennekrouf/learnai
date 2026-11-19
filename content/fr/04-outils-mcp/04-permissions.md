---
id: "04-04"
titre: "Permissions"
sous_rubriques:
  - id: portee
    titre: "Portée des outils"
  - id: sandbox
    titre: "Sandbox"
  - id: confirmation
    titre: "Confirmation humaine"
sources:
  - titre: "Invariant Labs — GitHub MCP Exploited: Accessing private repositories via MCP, 26 mai 2025"
    url: "https://invariantlabs.ai/blog/mcp-github-vulnerability"
  - titre: "Simon Willison — The lethal trifecta for AI agents, 16 juin 2025"
    url: "https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/"
  - titre: "MCP — Tools, révision 2026-07-28 (human in the loop, security considerations)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
  - titre: "Fortune — Replit wiped database, juillet 2025"
    url: "https://fortune.com/2025/07/23/ai-coding-tool-replit-wiped-database-called-it-a-catastrophic-failure/"
qcm:
  - id: portee-1
    sous_rubrique: portee
    question: "Mai 2025 : une issue piégée dans un dépôt GitHub public fait fuiter un dépôt privé via le serveur MCP GitHub. Quelle condition l'a permis ?"
    choix:
      - "Une faille dans le chiffrement de GitHub"
      - "Un bug de JSON-RPC"
      - "Un mot de passe faible"
      - "L'agent avait un jeton donnant accès à tous les dépôts, publics et privés"
    reponse: 3
    pourquoi: >-
      L'agent lit l'issue publique, qui contient des instructions cachées.
      Son jeton lui donne accès aux dépôts privés de l'utilisateur.
      Il copie des données privées dans une pull request publique.
      Invariant Labs recommande un seul dépôt par session et des droits fins.

  - id: portee-2
    sous_rubrique: portee
    question: "Selon Simon Willison (2025), quel trio rend un agent vulnérable au vol de données ?"
    choix:
      - "Vitesse, coût, latence"
      - "Accès à des données privées, exposition à du contenu non fiable, capacité de communiquer vers l'extérieur"
      - "Trop d'outils, trop de tokens, trop de mémoire"
      - "Python, JSON, HTTP"
    reponse: 1
    pourquoi: >-
      Le contenu non fiable apporte l'instruction malveillante.
      Les données privées sont la cible.
      La communication externe sert à les faire sortir.
      Retirer un seul des trois casse l'attaque.

  - id: portee-3
    sous_rubrique: portee
    question: "Ton agent doit seulement lire des tickets. Quel jeton lui donner ?"
    choix:
      - "Un jeton en lecture seule, limité au projet concerné"
      - "Un jeton administrateur, pour éviter les blocages"
      - "Le jeton personnel du développeur"
      - "Aucun jeton, le modèle se débrouille"
    reponse: 0
    pourquoi: >-
      Le moindre privilège limite les dégâts possibles.
      Un agent piégé avec un jeton en lecture ne peut rien effacer.
      Limité à un projet, il ne voit pas les autres.
      Les droits sont appliqués par le service, pas par le modèle.

  - id: portee-4
    sous_rubrique: portee
    question: "Mieux vaut un outil « executer_sql » ou trois outils précis comme « lire_commande » ?"
    choix:
      - "executer_sql : plus flexible"
      - "Aucune différence"
      - "Des outils précis : chacun ne peut faire que ce qu'il doit faire"
      - "executer_sql : moins de tokens"
    reponse: 2
    pourquoi: >-
      executer_sql peut aussi faire DROP TABLE.
      lire_commande ne peut que lire une commande.
      La flexibilité d'un outil est aussi celle d'un attaquant.
      Un outil précis se valide et se surveille facilement.

  - id: sandbox-1
    sous_rubrique: sandbox
    question: "Un outil exécute du code écrit par le modèle. Où le faire tourner ?"
    choix:
      - "Sur la machine du développeur, directement"
      - "Dans un environnement isolé : conteneur ou VM, sans secrets, réseau limité, temps et mémoire bornés"
      - "Sur le serveur de production"
      - "N'importe où, le modèle est prudent"
    reponse: 1
    pourquoi: >-
      Le code généré peut être faux, ou piloté par une injection.
      Isolé, il ne voit ni tes fichiers ni tes clés.
      Sans réseau, il ne peut rien envoyer dehors.
      Avec des limites, une boucle infinie ne bloque pas la machine.

  - id: sandbox-2
    sous_rubrique: sandbox
    question: "Replit, juillet 2025. Quelle faiblesse d'architecture a rendu l'effacement possible ?"
    choix:
      - "Un modèle trop petit"
      - "Un réseau trop lent"
      - "Un mauvais schéma JSON"
      - "La même base servait au développement, aux tests et à la production"
    reponse: 3
    pourquoi: >-
      L'agent travaillait sur l'environnement de développement.
      Mais la base de production était la même.
      Une erreur de développement devenait une perte en production.
      Replit a ensuite séparé automatiquement les deux.

  - id: sandbox-3
    sous_rubrique: sandbox
    question: "Tu installes un serveur MCP stdio trouvé sur GitHub. Avec quels droits tourne-t-il ?"
    choix:
      - "Aucun, il est isolé par MCP"
      - "Lecture seule"
      - "Les tiens : il peut faire tout ce que ton compte peut faire"
      - "Ceux du modèle"
    reponse: 2
    pourquoi: >-
      Un serveur stdio est un programme lancé sur ta machine.
      MCP ne l'isole pas.
      Il peut lire tes fichiers et tes clés.
      Installer un serveur, c'est exécuter son code : vérifie la source.

  - id: confirmation-1
    sous_rubrique: confirmation
    question: "Que recommande la spécification MCP pour les appels d'outils ?"
    choix:
      - "Un humain dans la boucle, capable de refuser un appel"
      - "Les exécuter tous automatiquement"
      - "Les journaliser seulement"
      - "Les limiter à un par minute"
    reponse: 0
    pourquoi: >-
      La spécification dit qu'il devrait toujours y avoir un humain capable de refuser.
      L'application montre quels outils sont exposés.
      Elle signale quand un outil est appelé.
      Elle demande confirmation pour les opérations sensibles.

  - id: confirmation-2
    sous_rubrique: confirmation
    question: "Faut-il demander confirmation pour chaque lecture de fichier ?"
    choix:
      - "Oui, toujours"
      - "Oui, deux fois"
      - "Non : trop de confirmations, et l'utilisateur finit par tout accepter sans lire"
      - "Non, jamais rien"
    reponse: 2
    pourquoi: >-
      Cent demandes par heure deviennent un réflexe.
      L'utilisateur clique « oui » sans regarder.
      La confirmation perd toute valeur.
      On la réserve à l'irréversible, à l'envoi vers l'extérieur, à l'argent.

  - id: confirmation-3
    sous_rubrique: confirmation
    question: "Que montrer à l'humain avant une confirmation ?"
    choix:
      - "L'outil et ses arguments exacts : la requête SQL, le destinataire, le montant"
      - "« L'agent veut agir. OK ? »"
      - "Le prompt système"
      - "Rien, pour aller vite"
    reponse: 0
    pourquoi: >-
      On ne peut pas juger ce qu'on ne voit pas.
      « Envoyer un mail » ne dit pas à qui.
      La spécification recommande de montrer les entrées avant l'appel.
      C'est aussi ce qui révèle une fuite de données.

  - id: confirmation-4
    sous_rubrique: confirmation
    question: "Comment garantir qu'une action sensible attend vraiment l'humain ?"
    choix:
      - "Dire au modèle : « demande toujours avant de supprimer »"
      - "Faire confiance au modèle"
      - "Augmenter la température"
      - "Bloquer l'exécution dans le code tant que l'approbation n'est pas reçue"
    reponse: 3
    pourquoi: >-
      Le gel de Replit était une consigne au modèle.
      Il a été ignoré.
      Une barrière dans le code ne peut pas être ignorée.
      Le modèle peut demander. Seul le code peut empêcher.
---

## Portée des outils {#portee}

Chaque outil donne un pouvoir. On ne donne que le pouvoir nécessaire.

Mai 2025. Invariant Labs montre une attaque contre le serveur MCP officiel de GitHub.

```mermaid
sequenceDiagram
  participant A as Attaquant
  participant P as Dépôt public
  participant G as Agent + serveur MCP GitHub
  participant V as Dépôt privé
  A->>P: Ouvre une issue avec des instructions cachées
  G->>P: Lit les issues
  G->>V: Lit des données privées (le jeton le permet)
  G->>P: Publie ces données dans une pull request publique
```

:::pourquoi
Le serveur n'avait pas de bug au sens classique.
Le problème : un jeton qui ouvrait tous les dépôts, et un agent qui lisait du contenu public.
Invariant recommande un dépôt par session et des droits fins.
:::

Simon Willison nomme le schéma général le **trio fatal** : données privées, contenu non fiable, communication externe.

:::pourquoi
Le contenu non fiable porte l'ordre. Les données privées sont la cible. La sortie externe sert à les voler.
Retire un seul des trois, et l'attaque échoue.
:::

Règles :

- jeton en lecture seule si l'agent ne fait que lire ;
- limité à un projet, un dossier, un dépôt ;
- des outils précis plutôt qu'un outil qui fait tout.

:::pourquoi
`executer_sql` peut aussi faire `DROP TABLE`.
`lire_commande` ne peut que lire une commande.
Les droits sont appliqués par le service. Le modèle ne peut pas les contourner.
:::

## Sandbox {#sandbox}

Un outil qui exécute du code ou des commandes tourne dans un **bac à sable**.

| Limite | Pourquoi |
|---|---|
| Conteneur ou VM | Pas d'accès à tes fichiers |
| Aucun secret dans l'environnement | Rien à voler |
| Réseau coupé ou liste blanche | Rien à envoyer dehors |
| Temps, mémoire, disque bornés | Une boucle infinie ne bloque rien |
| Dossier de travail seul en écriture | Pas de modification ailleurs |

:::pourquoi
Le code généré peut être faux.
Il peut aussi être dicté par une injection.
L'isolement limite les dégâts dans les deux cas.
:::

Sépare développement et production.

:::pourquoi
Chez Replit, la même base servait au test et à la production.
Une erreur de l'agent en développement a effacé la production.
:::

Un serveur MCP stdio tourne avec **tes** droits.

:::pourquoi
C'est un programme lancé sur ta machine.
MCP ne l'isole pas.
L'installer, c'est exécuter son code.
:::

## Confirmation humaine {#confirmation}

La spécification MCP recommande un humain capable de refuser un appel d'outil.

```mermaid
sequenceDiagram
  participant L as LLM
  participant C as Client (barrière)
  participant H as Humain
  participant S as Serveur MCP
  L->>C: supprimer_fichiers(dossier=/prod/data)
  C->>H: Outil et arguments exacts. Approuver ?
  H-->>C: Refusé
  C-->>L: Refusé par l'utilisateur
  Note over C,S: L'appel n'atteint jamais le serveur
```

Quoi confirmer :

- l'irréversible : suppression, envoi, publication ;
- ce qui sort : un mail, un message, une requête vers l'extérieur ;
- l'argent ;
- la production.

:::pourquoi
Trop de confirmations tuent la confirmation.
L'utilisateur clique « oui » par réflexe.
On la garde pour ce qui compte.
:::

Montre l'outil et ses arguments **exacts**.

:::pourquoi
« L'agent veut envoyer un mail » ne dit pas à qui.
Voir le destinataire révèle une fuite.
:::

La barrière est dans le **code**, pas dans le prompt.

:::pourquoi
Le gel de Replit était une consigne. Il a été ignoré.
Un appel bloqué dans le code n'atteint jamais le serveur.
:::

Journalise chaque appel : qui, quel outil, quels arguments, quelle décision.

:::pourquoi
Sans trace, impossible de comprendre un incident.
C'est le sujet de l'étape 6.
:::

## À retenir

- Moindre privilège : lecture seule, portée réduite, outils précis.
- Le trio fatal : données privées, contenu non fiable, sortie externe. Retire-en un.
- Code généré en bac à sable. Développement séparé de la production.
- Confirmation humaine pour l'irréversible, imposée par le code.
