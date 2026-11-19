---
id: "04-02"
titre: "MCP"
sous_rubriques:
  - id: versions
    titre: "Versions : initialize, puis sans état"
  - id: outils
    titre: "tools/list et tools/call"
  - id: ressources
    titre: "Ressources et prompts"
  - id: transports
    titre: "stdio et HTTP"
sources:
  - titre: "MCP — Lifecycle, révision 2025-11-25 (poignée de main initialize)"
    url: "https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle"
  - titre: "MCP — Overview et Versioning, révision 2026-07-28 (protocole sans état)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/index"
  - titre: "MCP — Tools, révision 2026-07-28"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
  - titre: "MCP — Transports stdio, révision 2026-07-28"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio"
  - titre: "MCP — Transports, révision 2025-06-18 (Streamable HTTP et avertissements de sécurité)"
    url: "https://modelcontextprotocol.io/specification/2025-06-18/basic/transports"
  - titre: "MCP Blog — MCP joins the Agentic AI Foundation, 9 décembre 2025"
    url: "https://blog.modelcontextprotocol.io/posts/2025-12-09-mcp-joins-agentic-ai-foundation/"
qcm:
  - id: versions-1
    sous_rubrique: versions
    question: "Quel problème MCP résout-il d'abord ?"
    choix:
      - "Chaque application devait coder une intégration pour chaque outil : N × M connecteurs"
      - "La vitesse des modèles"
      - "Le prix des tokens"
      - "La traduction des prompts"
    reponse: 0
    pourquoi: >-
      10 applications et 20 outils : 200 intégrations sur mesure.
      Avec un protocole commun, chaque outil écrit un serveur.
      Chaque application écrit un client.
      10 + 20 = 30 morceaux, qui marchent tous ensemble.

  - id: versions-2
    sous_rubrique: versions
    question: "Dans les versions jusqu'à 2025-11-25, quelle est la première chose qu'un client MCP envoie ?"
    choix:
      - "tools/call"
      - "Un ping"
      - "Une notification initialized"
      - "Une requête initialize avec sa version, ses capacités et son identité"
    reponse: 3
    pourquoi: >-
      Avant de travailler, client et serveur s'accordent.
      Quelle version du protocole ? Quelles fonctions de chaque côté ?
      Le serveur répond avec sa version, ses capacités, son identité.
      Le client confirme ensuite par la notification initialized.

  - id: versions-3
    sous_rubrique: versions
    question: "Le client propose la version 2025-11-25. Le serveur ne la connaît pas. Que fait le serveur, en mode initialize ?"
    choix:
      - "Il plante"
      - "Il répond avec une autre version qu'il supporte ; si le client ne la supporte pas, il se déconnecte"
      - "Il accepte quand même"
      - "Il renvoie -32601"
    reponse: 1
    pourquoi: >-
      La négociation se fait en un aller-retour.
      Le serveur propose sa version la plus récente.
      Le client accepte ou se déconnecte.
      Personne ne parle une version que l'autre ne comprend pas.

  - id: versions-4
    sous_rubrique: versions
    question: "Révision 2026-07-28 : qu'est-ce qui remplace la poignée de main initialize ?"
    choix:
      - "Rien, le serveur devine"
      - "Une clé d'API"
      - "Un cookie de session"
      - "Chaque requête porte sa version et les capacités du client dans _meta"
    reponse: 3
    pourquoi: >-
      Le protocole devient sans état.
      Chaque requête contient tout ce qu'il faut pour être traitée.
      Si la version n'est pas supportée, le serveur renvoie l'erreur -32022 avec la liste des versions acceptées.
      server/discover permet de les connaître à l'avance.

  - id: versions-5
    sous_rubrique: versions
    question: "Pourquoi rendre MCP sans état ?"
    choix:
      - "Pour économiser des octets"
      - "N'importe quelle instance du serveur peut traiter n'importe quelle requête"
      - "Pour supprimer les outils"
      - "Pour interdire le HTTP"
    reponse: 1
    pourquoi: >-
      Avec une session, la requête doit retrouver le bon serveur.
      Sans état, un répartiteur de charge envoie où il veut.
      Un serveur qui redémarre ne perd aucune session.
      L'état utile passe par des identifiants explicites, comme un numéro de panier.

  - id: outils-1
    sous_rubrique: outils
    question: "tools/list renvoie, pour chaque outil…"
    choix:
      - "Son nom, sa description, son schéma d'entrée, et éventuellement un schéma de sortie et des annotations"
      - "Son code source"
      - "Son prix"
      - "Ses logs"
    reponse: 0
    pourquoi: >-
      C'est tout ce que le modèle verra de l'outil.
      Le nom et la description lui disent quand l'utiliser.
      Le schéma d'entrée lui dit quoi fournir.
      Le schéma de sortie permet de valider le résultat.

  - id: outils-2
    sous_rubrique: outils
    question: "Un appel d'outil échoue parce que la date est dans le passé. Comment le serveur MCP doit-il le signaler ?"
    choix:
      - "Erreur JSON-RPC -32603"
      - "Un résultat vide"
      - "Un résultat normal avec isError à true et un message clair"
      - "Couper la connexion"
    reponse: 2
    pourquoi: >-
      C'est une erreur d'exécution, pas une erreur de protocole.
      Elle doit arriver au modèle pour qu'il se corrige.
      « La date doit être dans le futur » lui permet de réessayer.
      Les erreurs JSON-RPC servent aux requêtes mal formées ou aux outils inconnus.

  - id: outils-3
    sous_rubrique: outils
    question: "Le client appelle un outil qui n'existe pas. Que renvoie le serveur ?"
    choix:
      - "isError à true"
      - "Une erreur JSON-RPC -32602"
      - "Un résultat vide"
      - "Rien"
    reponse: 1
    pourquoi: >-
      La requête elle-même est invalide.
      C'est une erreur de protocole.
      -32602 signale des paramètres invalides, ici un nom d'outil inconnu.

  - id: outils-4
    sous_rubrique: outils
    question: "Un serveur annonce readOnlyHint à true pour un outil. Le client peut-il s'y fier ?"
    choix:
      - "Oui, toujours"
      - "Non, jamais"
      - "Oui, le protocole le vérifie"
      - "Seulement si le serveur est de confiance : les annotations sont des indications, pas des garanties"
    reponse: 3
    pourquoi: >-
      Une annotation est déclarée par le serveur lui-même.
      Un serveur malveillant peut se dire « lecture seule » et tout effacer.
      La spécification exige de les traiter comme non fiables.
      Sauf si le serveur vient d'une source de confiance.

  - id: outils-5
    sous_rubrique: outils
    question: "Deux serveurs exposent chacun un outil « search ». Que doit faire le client ?"
    choix:
      - "Garder le premier"
      - "Refuser les deux"
      - "Les distinguer, par exemple en préfixant le nom par le serveur"
      - "Les fusionner"
    reponse: 2
    pourquoi: >-
      Un nom d'outil n'est unique que dans son serveur.
      Le modèle ne saurait pas lequel appeler.
      Un préfixe comme « github.search » lève l'ambiguïté.

  - id: ressources-1
    sous_rubrique: ressources
    question: "Qui contrôle quoi dans MCP ?"
    choix:
      - "Outils : le modèle. Ressources : l'application. Prompts : l'utilisateur"
      - "Le modèle contrôle tout"
      - "L'utilisateur contrôle tout"
      - "Le serveur contrôle tout"
    reponse: 0
    pourquoi: >-
      Le modèle décide d'appeler un outil.
      L'application décide quelles ressources mettre en contexte.
      L'utilisateur choisit un prompt, souvent par une commande slash.
      Trois primitives, trois décideurs.

  - id: ressources-2
    sous_rubrique: ressources
    question: "Une ressource MCP est identifiée par…"
    choix:
      - "Un numéro"
      - "Un nom d'outil"
      - "Une URI, comme file:///projet/README.md"
      - "Un hash"
    reponse: 2
    pourquoi: >-
      Une URI désigne une donnée sans ambiguïté.
      resources/list donne la liste. resources/read donne le contenu.
      Les gabarits d'URI permettent des ressources paramétrées.

  - id: ressources-3
    sous_rubrique: ressources
    question: "Un serveur sert des fichiers via file://. Quel contrôle est obligatoire ?"
    choix:
      - "Nettoyer les chemins pour empêcher de sortir du dossier autorisé"
      - "Aucun"
      - "Chiffrer les fichiers"
      - "Limiter à 10 fichiers"
    reponse: 0
    pourquoi: >-
      « ../../.ssh/id_rsa » sort du dossier prévu.
      C'est une traversée de répertoire.
      La spécification exige de l'empêcher.

  - id: transports-1
    sous_rubrique: transports
    question: "En transport stdio, ton serveur affiche un message de debug avec print(). Que se passe-t-il ?"
    choix:
      - "Rien de grave"
      - "Le serveur redémarre"
      - "Le message s'affiche dans l'application"
      - "Le client reçoit une ligne qui n'est pas du JSON-RPC, et la communication casse"
    reponse: 3
    pourquoi: >-
      stdout est réservé aux messages MCP.
      Une ligne de debug y est lue comme un message invalide.
      Les logs vont sur stderr.
      C'est l'erreur la plus fréquente quand on écrit son premier serveur.

  - id: transports-2
    sous_rubrique: transports
    question: "En stdio, comment sont séparés les messages ?"
    choix:
      - "Par une longueur en en-tête"
      - "Par un retour à la ligne : un message par ligne, sans retour à la ligne à l'intérieur"
      - "Par un caractère nul"
      - "Par un délai"
    reponse: 1
    pourquoi: >-
      Le client lit ligne par ligne.
      Chaque ligne est un message JSON complet.
      Un retour à la ligne dans le JSON couperait le message en deux.
      Les sérialiseurs JSON compacts n'en produisent pas.

  - id: transports-3
    sous_rubrique: transports
    question: "Ton serveur MCP HTTP tourne en local. Quelles précautions la spécification demande-t-elle ?"
    choix:
      - "Aucune, c'est local"
      - "Désactiver HTTPS"
      - "Ouvrir sur 0.0.0.0 pour simplifier"
      - "Écouter sur 127.0.0.1, vérifier l'en-tête Origin, authentifier"
    reponse: 3
    pourquoi: >-
      Une page web malveillante peut viser ton serveur local.
      C'est l'attaque par rebinding DNS.
      Vérifier Origin la bloque.
      Écouter sur localhost évite l'exposition au réseau.

  - id: transports-4
    sous_rubrique: transports
    question: "En stdio, d'où le serveur tient-il ses identifiants (clés d'API) ?"
    choix:
      - "D'un flux OAuth complet"
      - "De son environnement, par exemple des variables d'environnement"
      - "Du modèle"
      - "De l'utilisateur, à chaque requête"
    reponse: 1
    pourquoi: >-
      Le serveur stdio est lancé par le client, sur la même machine.
      La spécification lui demande de prendre ses identifiants dans l'environnement.
      Le cadre d'autorisation OAuth vise les transports HTTP.
---

## Pourquoi un protocole {#versions}

Novembre 2024. Anthropic publie le **Model Context Protocol**.

Le problème : chaque application d'IA codait ses propres connecteurs vers chaque outil.

:::pourquoi
10 applications, 20 outils : 200 intégrations.
Avec un protocole commun : 10 clients et 20 serveurs.
Chaque nouveau serveur marche avec toutes les applications.
:::

En décembre 2025, Anthropic donne MCP à l'Agentic AI Foundation, sous l'égide de la Linux Foundation. Anthropic, Block et OpenAI la cofondent.

:::pourquoi
Un standard porté par un seul acteur inquiète ses concurrents.
Une fondation neutre rassure ceux qui l'adoptent.
:::

Trois rôles :

```mermaid
flowchart LR
  subgraph H["Application hôte"]
    L["LLM"]
    C1["Client MCP 1"]
    C2["Client MCP 2"]
  end
  C1 <-->|"JSON-RPC"| S1["Serveur GitHub"]
  C2 <-->|"JSON-RPC"| S2["Serveur fichiers"]
  L --- C1
  L --- C2
```

| Rôle | Exemple | Fait quoi |
|---|---|---|
| Hôte | Claude Desktop, un IDE | Contient le modèle et l'interface |
| Client | Un par serveur, dans l'hôte | Parle le protocole |
| Serveur | GitHub, fichiers, base de données | Expose outils, ressources, prompts |

### Les versions évoluent

Jusqu'à la révision **2025-11-25**, une connexion commence par une poignée de main.

```mermaid
sequenceDiagram
  participant C as Client
  participant S as Serveur
  C->>S: initialize (version, capacités, identité du client)
  S-->>C: version retenue, capacités, identité du serveur
  C--)S: notifications/initialized
  C->>S: tools/list, tools/call...
```

:::pourquoi
Les deux côtés s'accordent sur la version et les fonctions disponibles.
Si le client ne supporte pas la version proposée, il se déconnecte.
Avant cet accord, seuls les pings sont permis.
:::

La révision **2026-07-28** supprime cette poignée de main. Le protocole devient **sans état**.

Chaque requête porte sa version et les capacités du client dans `_meta`. Une version non supportée renvoie l'erreur -32022, avec la liste des versions acceptées.

:::pourquoi
Sans session, n'importe quelle instance du serveur traite n'importe quelle requête.
Un répartiteur de charge peut envoyer où il veut.
Un redémarrage ne casse rien.
L'état nécessaire passe par des identifiants explicites : un numéro de panier, de transaction.
:::

Beaucoup de serveurs parlent encore l'ancienne version. Un client peut parler les deux.

## tools/list et tools/call {#outils}

`tools/list` : le client découvre les outils.

`tools/call` : le client en appelle un, avec ses arguments.

Le résultat contient `content` (texte, image…), éventuellement `structuredContent` validé par le schéma de sortie, et `isError`.

Deux sortes d'erreurs :

| Erreur | Exemple | Forme | Va au modèle ? |
|---|---|---|---|
| Protocole | Outil inconnu, requête mal formée | Erreur JSON-RPC | Rarement utile |
| Exécution | Date dans le passé, API indisponible | Résultat avec `isError: true` | Oui, pour qu'il se corrige |

:::pourquoi
« La date doit être dans le futur » est une information exploitable.
Le modèle peut corriger l'argument et réessayer.
Une erreur de protocole signale un bug du client, pas une erreur du modèle.
:::

Les **annotations** (`readOnlyHint`, `destructiveHint`…) sont des indications. Pas des garanties.

:::pourquoi
C'est le serveur qui les déclare.
Un serveur malveillant peut mentir.
La spécification exige de les traiter comme non fiables, sauf serveur de confiance.
:::

Renvoie les outils dans un ordre stable.

:::pourquoi
La liste des outils entre dans le prompt.
Un ordre stable permet au cache de prompt de la réutiliser.
:::

## Ressources et prompts {#ressources}

MCP a trois primitives. Chacune a son décideur.

| Primitive | Qui décide | Exemple |
|---|---|---|
| Outil | Le modèle | Créer une issue GitHub |
| Ressource | L'application | Le fichier README du projet |
| Prompt | L'utilisateur | La commande /revue-de-code |

:::pourquoi
Le modèle ne devrait pas tout décider.
L'application choisit le contexte. L'utilisateur choisit le gabarit de tâche.
Le modèle agit dans ce cadre.
:::

Une **ressource** est identifiée par une URI. On la liste, on la lit, on peut s'abonner à ses changements.

Un **prompt** est un gabarit de messages, avec des arguments. Il apparaît souvent comme une commande slash.

Un serveur de fichiers doit empêcher la traversée de répertoire.

:::pourquoi
« ../../.ssh/id_rsa » sort du dossier autorisé.
La spécification l'interdit explicitement.
:::

## stdio et HTTP {#transports}

Deux transports standard.

**stdio** : le client lance le serveur comme sous-processus. Ils parlent par l'entrée et la sortie standard.

- Un message JSON par ligne. Pas de retour à la ligne à l'intérieur.
- `stdout` : uniquement des messages MCP.
- `stderr` : les logs.
- Arrêt : le client ferme l'entrée standard, attend, puis force l'arrêt si besoin.

:::pourquoi
Le client lit stdout ligne par ligne.
Un simple print() de debug y casse la communication.
C'est le bug numéro un des premiers serveurs.
:::

**Streamable HTTP** : le serveur est un service web, avec un seul point d'entrée. Il peut répondre en flux (SSE). Il remplace l'ancien transport HTTP+SSE de 2024.

En local, la spécification exige de vérifier l'en-tête `Origin`, et recommande d'écouter sur 127.0.0.1 et d'authentifier.

:::pourquoi
Une page web peut viser un serveur local par rebinding DNS.
Vérifier Origin bloque l'attaque.
Écouter sur 0.0.0.0 expose le serveur à tout le réseau.
:::

Identifiants : en HTTP, le cadre d'autorisation OAuth de MCP. En stdio, l'environnement.

## À retenir

- MCP transforme N × M intégrations en N + M.
- Jusqu'en 2025-11-25 : poignée de main initialize. Depuis 2026-07-28 : sans état, version dans chaque requête.
- Erreur d'exécution : `isError`, pour que le modèle se corrige. Erreur de protocole : JSON-RPC.
- Annotations non fiables. stdout réservé aux messages. Origin vérifié en HTTP.
