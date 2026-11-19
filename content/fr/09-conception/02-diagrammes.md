---
id: "09-02"
titre: "Diagrammes"
sous_rubriques:
  - id: flux-donnees
    titre: "Flux de données"
  - id: topologie
    titre: "Topologie d'agents"
sources:
  - titre: "Simon Brown — The C4 model for visualising software architecture"
    url: "https://c4model.com/"
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, 12 juin 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "Anthropic — How we built our multi-agent research system, 13 juin 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
qcm:
  - id: flux-donnees-1
    sous_rubrique: flux-donnees
    question: "Que montre un diagramme de flux de données ?"
    choix:
      - "Le planning du projet"
      - "L'organigramme de l'équipe"
      - "Les classes du code"
      - "Quelles données passent entre quels composants, et où elles sont stockées"
    reponse: 3
    pourquoi: >-
      Il suit la donnée, pas le code.
      De l'utilisateur au modèle, à la base, aux journaux.
      C'est là qu'on voit les fuites possibles.
      Et ce qu'il faut protéger.

  - id: flux-donnees-2
    sous_rubrique: flux-donnees
    question: "Qu'est-ce qu'une frontière de confiance sur un diagramme ?"
    choix:
      - "Le bord de la feuille"
      - "La limite où une donnée sort de ton contrôle, par exemple vers l'API d'un fournisseur de modèle"
      - "La séparation entre deux fichiers"
      - "Une couleur décorative"
    reponse: 1
    pourquoi: >-
      Au-delà de la frontière, d'autres règles s'appliquent.
      Un fournisseur externe, un outil tiers, un journal partagé.
      Chaque traversée est une question : quoi passe, et faut-il masquer ?
      C'est le lien avec l'étape 6.

  - id: flux-donnees-3
    sous_rubrique: flux-donnees
    question: "Sur une flèche de flux de données, qu'écrire ?"
    choix:
      - "Ce qui passe : « question + 5 extraits », « numéro de carte masqué »"
      - "Rien, la flèche suffit"
      - "Le nom du développeur"
      - "La couleur du composant"
    reponse: 0
    pourquoi: >-
      Une flèche sans étiquette dit seulement « ça parle ».
      Une flèche étiquetée dit quoi, donc quoi protéger.
      Le relecteur voit tout de suite une donnée personnelle qui sort.

  - id: flux-donnees-4
    sous_rubrique: flux-donnees
    question: "Que propose le modèle C4 de Simon Brown ?"
    choix:
      - "Un seul diagramme pour tout"
      - "Quatre couleurs obligatoires"
      - "Des niveaux de zoom : contexte, conteneurs, composants, code"
      - "Un langage de programmation"
    reponse: 2
    pourquoi: >-
      Contexte : le système et ses utilisateurs.
      Conteneurs : applications et bases déployées à part.
      Composants : les blocs dans un conteneur.
      On choisit le zoom selon le lecteur.

  - id: flux-donnees-5
    sous_rubrique: flux-donnees
    question: "Pourquoi écrire ses diagrammes en texte, avec Mermaid par exemple ?"
    choix:
      - "Ils sont plus jolis qu'un dessin"
      - "Ils vivent dans le dépôt, se relisent en PR et se mettent à jour avec le code"
      - "Ils ne peuvent pas devenir faux"
      - "Ils remplacent les tests"
    reponse: 1
    pourquoi: >-
      Une image exportée se périme en silence.
      Un diagramme en texte se modifie dans la même PR que le code.
      Le diff montre ce qui a changé.
      Il peut quand même devenir faux : il faut le relire.

  - id: flux-donnees-6
    sous_rubrique: flux-donnees
    question: "Quelle destination de données oublie-t-on souvent sur un diagramme d'agent ?"
    choix:
      - "La base de données principale"
      - "Le modèle"
      - "L'utilisateur"
      - "Les journaux et les traces, qui contiennent prompts et réponses"
    reponse: 3
    pourquoi: >-
      Les traces gardent tout : questions, documents, réponses.
      Elles partent souvent vers un outil d'observabilité externe.
      C'est une copie complète des données, hors de la base.
      Elle se protège comme la base.

  - id: topologie-1
    sous_rubrique: topologie
    question: "Que montre un diagramme de topologie d'agents ?"
    choix:
      - "Le prix de chaque appel"
      - "La taille des modèles"
      - "Qui appelle qui, en série ou en parallèle, et quel contexte chacun reçoit"
      - "La liste des développeurs"
    reponse: 2
    pourquoi: >-
      La topologie décide du coût, de la latence et des incohérences.
      Le diagramme rend ces choix visibles.
      Le contexte reçu par chaque agent est le point le plus oublié.

  - id: topologie-2
    sous_rubrique: topologie
    question: "Selon Cognition, que faut-il transmettre à un sous-agent ?"
    choix:
      - "Le contexte complet, y compris les traces des autres agents, pas seulement le texte de la tâche"
      - "Seulement le titre de la tâche"
      - "Rien, il doit deviner"
      - "Seulement le résultat attendu"
    reponse: 0
    pourquoi: >-
      Dans l'exemple Flappy Bird, chaque sous-agent avait sa tâche.
      Il ne voyait pas le travail de l'autre.
      Les choix implicites se sont contredits.
      Le diagramme doit montrer ce contexte transmis.

  - id: topologie-3
    sous_rubrique: topologie
    question: "Comment représenter des sous-agents qui travaillent en parallèle ?"
    choix:
      - "Des boîtes sans flèches"
      - "Une seule ligne droite"
      - "Des branches partant du même agent principal, qui reviennent vers lui"
      - "On ne les représente pas"
    reponse: 2
    pourquoi: >-
      Le parallélisme se voit à la forme.
      Branches qui partent et reviennent : on voit qui synthétise.
      Une ligne droite dirait l'inverse : une suite d'étapes.

  - id: topologie-4
    sous_rubrique: topologie
    question: "Quelle information ajouter à une topologie d'agents pour la rendre utile à une revue ?"
    choix:
      - "Les outils de chaque agent, et où l'état est stocké"
      - "Les avatars des agents"
      - "Les couleurs préférées de l'équipe"
      - "Rien de plus"
    reponse: 0
    pourquoi: >-
      Les outils disent ce que chaque agent peut faire : moindre privilège.
      L'état stocké dit ce qui survit à une panne : durabilité.
      Un relecteur vérifie les étapes 5 et 6 en un regard.

  - id: topologie-5
    sous_rubrique: topologie
    question: "Un diagramme veut tout montrer : flux, topologie, déploiement, coûts. Problème ?"
    choix:
      - "Mermaid refuse"
      - "Aucun, plus il y en a, mieux c'est"
      - "Il sera trop petit"
      - "Il devient illisible : un diagramme répond à une question"
    reponse: 3
    pourquoi: >-
      Le lecteur cherche une réponse précise.
      Trop d'informations la cachent.
      Mieux vaut trois diagrammes clairs qu'un seul confus.
      Chacun avec son titre et sa question.
---

## Flux de données {#flux-donnees}

Le **flux de données** suit la donnée, pas le code : d'où elle vient, où elle passe, où elle reste.

```mermaid
flowchart LR
  U[Utilisateur] -->|question| A[Application]
  A -->|question masquée| R[(Index vectoriel)]
  R -->|5 extraits| A
  subgraph Externe [Hors de notre contrôle]
    M[API du modèle]
    O[Outil d'observabilité]
  end
  A -->|question masquée + 5 extraits| M
  M -->|réponse| A
  A -->|trace complète| O
  A -->|réponse| U
  A -->|journal d'audit| J[(Journal)]
```

:::pourquoi
On voit d'un coup d'œil ce qui sort de chez soi.
Chaque flèche qui traverse la frontière est une question de sécurité.
:::

Trois règles :

1. Chaque flèche porte une **étiquette** : ce qui passe.
2. Les **frontières de confiance** sont dessinées.
3. Les **copies** sont visibles : journaux, traces, caches.

:::pourquoi
Une flèche sans étiquette dit « ça parle », pas « quoi ».
La frontière montre où les règles changent.
Les traces sont une copie complète des données, souvent oubliée.
:::

Le **modèle C4** de Simon Brown propose des niveaux de zoom :

| Niveau | Montre | Pour qui |
|---|---|---|
| Contexte | Le système, ses utilisateurs, ses voisins | Tout le monde |
| Conteneurs | Applications et bases déployées à part | Équipe technique |
| Composants | Les blocs dans un conteneur | Développeurs |
| Code | Le détail d'un composant (optionnel) | Rarement utile |

:::pourquoi
Un dirigeant n'a pas besoin des composants.
Un développeur a besoin de plus que le contexte.
On choisit le zoom selon le lecteur.
:::

Les diagrammes s'écrivent en **texte** (Mermaid) et vivent **dans le dépôt**.

:::pourquoi
Ils se modifient dans la même PR que le code.
Le diff montre ce qui a changé.
Une image exportée se périme en silence.
:::

## Topologie d'agents {#topologie}

La **topologie** montre qui appelle qui, en série ou en parallèle, avec quel contexte.

```mermaid
flowchart TB
  U[Utilisateur] --> P[Agent principal]
  P -->|piste 1 + plan complet| S1[Sous-agent 1]
  P -->|piste 2 + plan complet| S2[Sous-agent 2]
  P -->|piste 3 + plan complet| S3[Sous-agent 3]
  S1 -->|résumé + sources| P
  S2 -->|résumé + sources| P
  S3 -->|résumé + sources| P
  P --> E[(État sauvegardé)]
  S1 -.->|recherche web| W[Outils]
  S2 -.->|recherche web| W
  S3 -.->|recherche web| W
  P -->|réponse| U
```

:::pourquoi
La forme dit le parallélisme : branches qui partent et reviennent.
On voit qui synthétise, et ce que chacun reçoit.
:::

L'étiquette la plus importante : le **contexte transmis**.

:::pourquoi
Cognition : les sous-agents de Flappy Bird ne voyaient pas le travail de l'autre.
Leurs choix implicites se sont contredits.
Une flèche « tâche seule » annonce ce risque.
:::

Ajoute les **outils** de chaque agent et l'endroit où l'**état** est stocké.

:::pourquoi
Les outils montrent les droits : moindre privilège, étape 6.
L'état montre ce qui survit à une panne : durabilité, étape 5.
:::

Un diagramme répond à **une question**.

:::pourquoi
Flux, topologie, déploiement, coûts sur un seul schéma : personne ne lit.
Trois diagrammes clairs valent mieux qu'un seul confus.
:::

## À retenir

- Flux de données : étiquettes sur les flèches, frontières de confiance, copies visibles.
- C4 : choisir le niveau de zoom selon le lecteur.
- Topologie : qui appelle qui, en parallèle ou non, avec quel contexte.
- Diagrammes en texte, dans le dépôt. Un diagramme, une question.
