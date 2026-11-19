---
id: "04-03"
titre: "Schémas stricts"
sous_rubriques:
  - id: json-schema
    titre: "JSON Schema"
  - id: valider
    titre: "Valider entrées et sorties"
sources:
  - titre: "MCP — Overview, révision 2026-07-28, section JSON Schema Usage"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/index"
  - titre: "MCP — Tools, révision 2026-07-28 (security considerations, outputSchema)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
  - titre: "Pydantic — JSON Schema"
    url: "https://docs.pydantic.dev/latest/concepts/json_schema/"
qcm:
  - id: json-schema-1
    sous_rubrique: json-schema
    question: "Que fait additionalProperties à false dans un schéma d'outil ?"
    choix:
      - "Il refuse tout champ qui n'est pas déclaré"
      - "Il rend tous les champs obligatoires"
      - "Il autorise les champs vides"
      - "Il désactive la validation"
    reponse: 0
    pourquoi: >-
      Sans lui, un argument inconnu passe sans bruit.
      Le modèle peut inventer « force: true » ou « admin: true ».
      Le serveur risque de l'ignorer, ou pire, de l'utiliser.
      Refuser l'inconnu, c'est la valeur par défaut d'un schéma strict.

  - id: json-schema-2
    sous_rubrique: json-schema
    question: "Le paramètre « priorite » n'accepte que basse, normale ou haute. Comment l'écrire ?"
    choix:
      - "type: string, avec une phrase dans la description"
      - "type: number"
      - "type: string, enum: [basse, normale, haute]"
      - "type: object"
    reponse: 2
    pourquoi: >-
      Une description se lit. Un enum se vérifie.
      Le modèle voit les trois valeurs possibles.
      Le validateur rejette « urgente » sans ambiguïté.

  - id: json-schema-3
    sous_rubrique: json-schema
    question: "À quoi sert la description de chaque propriété dans le schéma ?"
    choix:
      - "À rien, seul le type compte"
      - "Le modèle la lit pour savoir quoi mettre dans le champ"
      - "À la documentation interne seulement"
      - "À la validation"
    reponse: 1
    pourquoi: >-
      « date » ne dit pas le format attendu.
      « Date de départ, format AAAA-MM-JJ, dans le futur » le dit.
      Le modèle ne voit que le schéma et ses descriptions.
      Une bonne description évite la moitié des erreurs.

  - id: json-schema-4
    sous_rubrique: json-schema
    question: "Un schéma contient un $ref vers une URL externe. Que dit la spécification MCP ?"
    choix:
      - "Le télécharger automatiquement"
      - "Le remplacer par {}"
      - "L'ignorer et tout accepter"
      - "Ne pas le résoudre automatiquement : c'est désactivé par défaut"
    reponse: 3
    pourquoi: >-
      Résoudre un $ref, c'est faire une requête réseau.
      Un schéma piégé pourrait viser un service interne.
      La spécification interdit la résolution automatique.
      Un schéma qui ne se résout pas doit être rejeté, pas accepté par défaut.

  - id: json-schema-5
    sous_rubrique: json-schema
    question: "Avec Pydantic, pourquoi générer le schéma depuis une classe Python ?"
    choix:
      - "C'est plus rapide à exécuter"
      - "Pydantic est obligatoire pour MCP"
      - "La même classe décrit l'outil et valide les arguments : une seule source de vérité"
      - "Pour éviter JSON"
    reponse: 2
    pourquoi: >-
      Un schéma écrit à la main et un code de validation séparé divergent.
      Avec Pydantic, model_json_schema() produit le schéma.
      La même classe valide les arguments reçus.
      Changer un champ change les deux à la fois.

  - id: valider-1
    sous_rubrique: valider
    question: "Le client MCP a déjà validé les arguments contre le schéma. Le serveur doit-il revalider ?"
    choix:
      - "Oui : la spécification l'exige, le client peut être bogué ou malveillant"
      - "Non, c'est redondant"
      - "Seulement en production"
      - "Seulement pour les nombres"
    reponse: 0
    pourquoi: >-
      Le serveur ne contrôle pas ses clients.
      Un client peut ne rien vérifier, ou être écrit par un attaquant.
      La spécification dit : les serveurs doivent valider toutes les entrées.
      La frontière de confiance est chez le serveur.

  - id: valider-2
    sous_rubrique: valider
    question: "« La date de départ doit être dans le futur. » Le schéma peut-il le garantir ?"
    choix:
      - "Oui, avec format: date"
      - "Oui, avec minimum"
      - "Non : c'est une règle métier, à vérifier dans le code"
      - "Oui, avec enum"
    reponse: 2
    pourquoi: >-
      Le schéma vérifie la forme : c'est une date.
      Il ne connaît pas la date du jour.
      Ni si l'utilisateur possède ce dossier.
      Ces règles vivent dans le code, après le schéma.

  - id: valider-3
    sous_rubrique: valider
    question: "Les arguments sont invalides. Quel message renvoyer au modèle ?"
    choix:
      - "« date_depart invalide : 2024-03-01 est passée. Aujourd'hui : 2026-11-19. Format AAAA-MM-JJ. »"
      - "« Erreur. »"
      - "La pile d'appels complète"
      - "Rien"
    reponse: 0
    pourquoi: >-
      Le modèle se corrige avec ce qu'on lui dit.
      « Erreur » ne lui dit rien.
      Le champ, la valeur, la règle et un exemple lui suffisent.
      La pile d'appels, elle, peut fuiter des chemins internes.

  - id: valider-4
    sous_rubrique: valider
    question: "Un outil déclare un outputSchema. Qui doit le respecter et qui doit vérifier ?"
    choix:
      - "Personne, c'est indicatif"
      - "Seul le client"
      - "Le modèle doit le respecter"
      - "Le serveur doit le respecter, le client devrait le vérifier"
    reponse: 3
    pourquoi: >-
      Le serveur s'engage sur la forme de son résultat.
      Le client vérifie avant de passer le résultat au modèle ou au code.
      Une sortie invalide détectée tôt évite une erreur plus loin.

  - id: valider-5
    sous_rubrique: valider
    question: "Ton API LLM propose un mode strict qui force les arguments à respecter le schéma. Peut-on retirer la validation côté serveur ?"
    choix:
      - "Oui"
      - "Non : le serveur sert d'autres clients, et le schéma ne couvre pas les règles métier"
      - "Oui, si la température est à 0"
      - "Oui, en développement"
    reponse: 1
    pourquoi: >-
      Le mode strict protège un client, pas le serveur.
      D'autres clients appellent le même serveur.
      Et une date valide peut rester une date passée.
      Chaque couche valide ce qu'elle reçoit.
---

## JSON Schema {#json-schema}

Un outil MCP décrit ses arguments avec **JSON Schema**. Par défaut, la version 2020-12.

```json
{
  "type": "object",
  "properties": {
    "date_depart": { "type": "string", "format": "date",
                     "description": "Date de départ, AAAA-MM-JJ, dans le futur" },
    "priorite": { "type": "string", "enum": ["basse", "normale", "haute"] },
    "passagers": { "type": "integer", "minimum": 1, "maximum": 9 }
  },
  "required": ["date_depart", "passagers"],
  "additionalProperties": false
}
```

Les mots-clés qui comptent :

| Mot-clé | Rôle |
|---|---|
| `required` | Champs obligatoires |
| `enum` | Liste fermée de valeurs |
| `minimum`, `maximum`, `pattern` | Bornes et format |
| `additionalProperties: false` | Refuse tout champ non déclaré |
| `description` | Ce que le modèle lit pour remplir le champ |

:::pourquoi
Un schéma sert deux lecteurs.
Le modèle le lit pour savoir quoi écrire.
Le validateur le lit pour refuser ce qui ne va pas.
:::

Un outil sans paramètre : `{"type": "object", "additionalProperties": false}`.

:::pourquoi
La spécification le recommande.
Il accepte seulement un objet vide.
:::

Ne résous jamais un `$ref` distant automatiquement.

:::pourquoi
Ce serait une requête réseau déclenchée par un schéma.
Un schéma piégé pourrait viser un service interne.
:::

Avec **Pydantic**, une classe Python produit le schéma (`model_json_schema()`) et valide les arguments. `extra="forbid"` donne `additionalProperties: false`.

:::pourquoi
Une seule définition pour décrire et pour valider.
Elles ne peuvent plus diverger.
:::

## Valider entrées et sorties {#valider}

Le serveur valide **toujours** ses entrées. Même si le client l'a déjà fait.

```mermaid
flowchart LR
  M["Modèle"] -->|"arguments"| C["Client MCP"]
  C -->|"tools/call"| S1["Serveur : schéma"]
  S1 -->|"forme valide"| S2["Serveur : règles métier"]
  S2 -->|"autorisé"| X["Exécution"]
  X --> S3["Serveur : sortie conforme à outputSchema"]
  S3 -->|"résultat"| C
  S1 -.->|"isError, message précis"| C
  S2 -.->|"isError, message précis"| C
```

:::pourquoi
Le serveur ne contrôle pas ses clients.
La spécification exige qu'il valide tout ce qui entre.
:::

Deux couches :

1. **Le schéma** : types, champs, bornes.
2. **Les règles métier** : date dans le futur, ressource appartenant à l'utilisateur, montant sous le plafond.

:::pourquoi
Un schéma ne connaît ni la date du jour, ni les droits de l'utilisateur.
Ces règles vivent dans le code.
:::

En cas d'erreur, renvoie un message que le modèle peut exploiter : le champ, la valeur, la règle, un exemple.

:::pourquoi
Le modèle se corrige avec ce qu'on lui dit.
« Erreur » ne lui apprend rien.
:::

Valide aussi la sortie. Le serveur respecte son `outputSchema`. Le client vérifie.

:::pourquoi
Une sortie mal formée casse le code qui la lit.
La détecter à la frontière évite une erreur plus loin, plus difficile à comprendre.
:::

Le mode strict d'une API LLM ne remplace pas la validation serveur.

:::pourquoi
Il protège un client. Le serveur en a d'autres.
Et il ne vérifie pas les règles métier.
:::

## À retenir

- Schéma strict : `required`, `enum`, bornes, `additionalProperties: false`.
- Les descriptions sont lues par le modèle. Soigne-les.
- Le serveur valide toujours : schéma, puis règles métier.
- Messages d'erreur précis. Sorties validées aussi.
