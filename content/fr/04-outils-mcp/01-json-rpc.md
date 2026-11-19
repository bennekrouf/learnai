---
id: "04-01"
titre: "JSON-RPC"
sous_rubriques:
  - id: requete
    titre: "Requête et réponse"
  - id: notification
    titre: "Notification"
  - id: erreurs
    titre: "Erreurs"
sources:
  - titre: "JSON-RPC 2.0 Specification"
    url: "https://www.jsonrpc.org/specification"
  - titre: "Model Context Protocol — Overview, révision 2026-07-28 (messages et codes d'erreur)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/index"
qcm:
  - id: requete-1
    sous_rubrique: requete
    question: "Une requête JSON-RPC 2.0 contient…"
    choix:
      - "token, method, résultat"
      - "url, verbe HTTP, corps"
      - "seulement method"
      - "jsonrpc, id, method, et éventuellement params"
    reponse: 3
    pourquoi: >-
      jsonrpc vaut toujours « 2.0 ».
      method dit quoi faire, par exemple « tools/call ».
      params porte les arguments.
      id permet de relier la réponse à la requête.

  - id: requete-2
    sous_rubrique: requete
    question: "À quoi sert l'id d'une requête ?"
    choix:
      - "À authentifier le client"
      - "À relier chaque réponse à sa requête, même si les réponses arrivent dans le désordre"
      - "À compter les requêtes facturées"
      - "À chiffrer le message"
    reponse: 1
    pourquoi: >-
      Un client peut envoyer trois requêtes sans attendre.
      La troisième peut répondre en premier.
      La réponse porte le même id que sa requête.
      Le client sait ainsi à quoi elle répond.

  - id: requete-3
    sous_rubrique: requete
    question: "Une réponse JSON-RPC valide contient…"
    choix:
      - "result OU error, jamais les deux"
      - "result et error ensemble"
      - "ni l'un ni l'autre"
      - "seulement l'id"
    reponse: 0
    pourquoi: >-
      Soit l'opération a réussi : result.
      Soit elle a échoué : error.
      Les deux à la fois seraient ambigus.
      Le client teste la présence de l'un ou de l'autre.

  - id: requete-4
    sous_rubrique: requete
    question: "JSON-RPC impose-t-il un transport (HTTP, socket…) ?"
    choix:
      - "Oui, HTTP uniquement"
      - "Oui, WebSocket uniquement"
      - "Non, il définit les messages, pas le canal"
      - "Oui, TCP uniquement"
    reponse: 2
    pourquoi: >-
      JSON-RPC décrit la forme des messages.
      Ils peuvent passer par HTTP, une socket, ou l'entrée standard d'un processus.
      C'est ce qui permet à MCP d'avoir plusieurs transports.

  - id: notification-1
    sous_rubrique: notification
    question: "Qu'est-ce qui distingue une notification d'une requête ?"
    choix:
      - "Elle est plus courte"
      - "Elle n'a pas d'id, et le destinataire ne doit pas répondre"
      - "Elle est chiffrée"
      - "Elle passe par un autre canal"
    reponse: 1
    pourquoi: >-
      Sans id, il n'y a rien à quoi relier une réponse.
      La notification est un message à sens unique.
      Exemple : « la liste des outils a changé ».
      L'émetteur n'attend rien en retour.

  - id: notification-2
    sous_rubrique: notification
    question: "Un client veut annuler une requête en cours dans MCP. Il envoie…"
    choix:
      - "Une nouvelle requête avec le même id"
      - "Une erreur -32603"
      - "Rien, il ferme la connexion"
      - "Une notification d'annulation qui cite l'id de la requête"
    reponse: 3
    pourquoi: >-
      L'annulation n'attend pas de réponse : c'est une notification.
      Elle porte l'id de la requête à annuler.
      Le serveur arrête le travail dès que possible.
      Il n'envoie plus rien pour cette requête.

  - id: erreurs-1
    sous_rubrique: erreurs
    question: "Le client appelle une méthode que le serveur ne connaît pas. Quel code ?"
    choix:
      - "-32700"
      - "-32602"
      - "-32601"
      - "-32603"
    reponse: 2
    pourquoi: >-
      -32601 : méthode introuvable.
      -32700 : JSON illisible.
      -32602 : paramètres invalides.
      -32603 : erreur interne du serveur.

  - id: erreurs-2
    sous_rubrique: erreurs
    question: "Le serveur reçoit un texte qui n'est pas du JSON valide. Quel code ?"
    choix:
      - "-32700, erreur d'analyse"
      - "-32600, requête invalide"
      - "-32601, méthode introuvable"
      - "-32603, erreur interne"
    reponse: 0
    pourquoi: >-
      Le serveur n'a même pas pu lire le message.
      C'est une erreur d'analyse : -32700.
      -32600 sert quand le JSON est lisible mais n'a pas la forme d'une requête.

  - id: erreurs-3
    sous_rubrique: erreurs
    question: "Que contient l'objet error d'une réponse ?"
    choix:
      - "Une pile d'appels complète, obligatoire"
      - "Seulement un texte libre"
      - "Un code entier, un message, et éventuellement des données"
      - "Un code HTTP"
    reponse: 2
    pourquoi: >-
      Le code permet au programme de réagir.
      Le message aide l'humain qui lit les logs.
      data peut préciser, par exemple les versions supportées.
      N'y mets jamais de secret : il remonte au client.
---

## Requête et réponse {#requete}

MCP parle **JSON-RPC 2.0**. Un format simple pour appeler une fonction à distance.

```json
{ "jsonrpc": "2.0", "id": 7, "method": "tools/call",
  "params": { "name": "meteo", "arguments": { "ville": "Lyon" } } }
```

```json
{ "jsonrpc": "2.0", "id": 7, "result": { "content": [ { "type": "text", "text": "12 °C" } ] } }
```

Même id dans la question et la réponse.

:::pourquoi
Plusieurs requêtes peuvent être en vol en même temps.
Les réponses peuvent revenir dans le désordre.
L'id dit à quelle question répond chaque réponse.
:::

Une réponse contient `result` **ou** `error`. Jamais les deux.

MCP ajoute deux règles : l'id n'est jamais `null`, et il est unique parmi les requêtes en cours.

:::pourquoi
Un id nul ou réutilisé rendrait la correspondance ambiguë.
:::

JSON-RPC ne dit rien du transport.

:::pourquoi
Il décrit les messages, pas le tuyau.
MCP peut donc passer par l'entrée standard ou par HTTP.
:::

## Notification {#notification}

Une notification est un message **sans id**. On n'y répond pas.

```json
{ "jsonrpc": "2.0", "method": "notifications/tools/list_changed" }
```

:::pourquoi
Certains messages informent sans rien demander.
« La liste des outils a changé. » « Annule la requête 7. »
Attendre une réponse serait inutile.
:::

## Erreurs {#erreurs}

Les codes standard de JSON-RPC :

| Code | Signification | Exemple |
|---|---|---|
| -32700 | Erreur d'analyse | JSON illisible |
| -32600 | Requête invalide | Il manque `method` |
| -32601 | Méthode introuvable | `tools/cal` au lieu de `tools/call` |
| -32602 | Paramètres invalides | Outil inconnu, argument manquant |
| -32603 | Erreur interne | Exception côté serveur |

MCP réserve ses propres codes, par exemple -32022 pour une version de protocole non supportée.

:::pourquoi
Un code entier se teste dans le programme.
Un message en texte libre ne se teste pas de façon fiable.
:::

Ne mets jamais de secret dans `message` ou `data`.

:::pourquoi
L'erreur remonte au client, et souvent au modèle.
Un chemin de fichier ou une clé dans une erreur, c'est une fuite.
:::

```mermaid
sequenceDiagram
  participant H as Application hôte
  participant C as Client MCP
  participant S as Serveur MCP
  H->>C: Appelle l'outil meteo
  C->>S: Requête id 7 : tools/call
  C->>S: Requête id 8 : resources/read
  S-->>C: Réponse id 8
  S-->>C: Réponse id 7
  C->>S: Notification : annule l'id 9
  C-->>H: Résultat de l'outil meteo
```

## À retenir

- Requête : id, method, params. Réponse : même id, result ou error.
- Notification : pas d'id, pas de réponse.
- Les codes d'erreur se testent. Les messages ne contiennent pas de secret.
- JSON-RPC décrit les messages, pas le transport.
