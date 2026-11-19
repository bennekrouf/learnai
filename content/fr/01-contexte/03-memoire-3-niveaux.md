---
id: "01-03"
titre: "Mémoire à 3 niveaux"
sous_rubriques:
  - id: travail
    titre: "Mémoire de travail"
  - id: episodique
    titre: "Mémoire épisodique"
  - id: semantique
    titre: "Mémoire sémantique"
sources:
  - titre: "Packer et al. — MemGPT: Towards LLMs as Operating Systems, 2023"
    url: "https://arxiv.org/abs/2310.08560"
  - titre: "Tulving — Episodic and semantic memory, 1972 (distinction reprise en psychologie cognitive)"
    url: "https://en.wikipedia.org/wiki/Episodic_memory"
qcm:
  - id: travail-1
    sous_rubrique: travail
    question: "Pendant une tâche, l'agent note son plan en 5 étapes et coche celles qui sont faites. Quelle mémoire ?"
    choix:
      - "Mémoire de travail"
      - "Mémoire épisodique"
      - "Mémoire sémantique"
      - "Aucune, c'est la réponse"
    reponse: 0
    pourquoi: >-
      Le plan sert à la tâche en cours.
      Il change à chaque étape.
      Il n'a plus d'intérêt une fois la tâche finie.
      C'est la définition de la mémoire de travail.

  - id: travail-2
    sous_rubrique: travail
    question: "Pourquoi donner un bloc-notes à un agent, plutôt que tout garder dans l'historique ?"
    choix:
      - "C'est plus joli"
      - "Le modèle ne lit pas l'historique"
      - "Le bloc-notes ne coûte pas de tokens"
      - "L'état utile reste court et à jour, même quand l'historique est coupé"
    reponse: 3
    pourquoi: >-
      L'historique grandit et finit par être résumé ou coupé.
      Le plan risque de partir avec.
      Un bloc-notes garde l'état utile en un seul endroit court.
      On peut couper l'historique sans perdre où on en est.

  - id: episodique-1
    sous_rubrique: episodique
    question: "« Le 3 mars, l'utilisateur a annulé sa commande 48213. » Quelle mémoire ?"
    choix:
      - "Mémoire de travail"
      - "Mémoire épisodique"
      - "Mémoire sémantique"
      - "Aucune"
    reponse: 1
    pourquoi: >-
      C'est un événement daté.
      Il raconte ce qui s'est passé, une fois.
      La mémoire épisodique garde les épisodes.

  - id: episodique-2
    sous_rubrique: episodique
    question: "Pourquoi compresser les épisodes au lieu de garder les conversations brutes ?"
    choix:
      - "Les conversations brutes exploseraient le budget de tokens"
      - "Pour cacher les données"
      - "Le modèle ne lit pas les conversations"
      - "Pour les traduire"
    reponse: 0
    pourquoi: >-
      Une conversation brute fait des milliers de tokens.
      Cent conversations ne tiennent dans aucune fenêtre.
      Un résumé daté garde l'essentiel en quelques lignes.
      On peut alors en rappeler beaucoup.

  - id: semantique-1
    sous_rubrique: semantique
    question: "« L'utilisateur est végétarien. » Quelle mémoire ?"
    choix:
      - "Mémoire de travail"
      - "Mémoire épisodique"
      - "Aucune"
      - "Mémoire sémantique"
    reponse: 3
    pourquoi: >-
      Ce n'est pas un événement, c'est un fait durable.
      Il n'a pas de date.
      La mémoire sémantique garde ce qu'on sait.

  - id: semantique-2
    sous_rubrique: semantique
    question: "L'utilisateur a commandé du poisson une fois. L'agent enregistre « aime le poisson ». Le problème ?"
    choix:
      - "Aucun"
      - "Une généralisation hâtive à partir d'un seul épisode"
      - "Le fait est trop court"
      - "Il fallait l'écrire en anglais"
    reponse: 1
    pourquoi: >-
      Un épisode n'est pas une habitude.
      C'était peut-être un cadeau, ou une exception.
      Un faux fait durable fausse toutes les sessions suivantes.
      Garde le lien vers les épisodes qui justifient chaque fait.

  - id: semantique-3
    sous_rubrique: semantique
    question: "L'utilisateur a 500 faits enregistrés. Lesquels injecter dans le contexte ?"
    choix:
      - "Les 500"
      - "Les plus récents"
      - "Ceux qui sont pertinents pour la question actuelle"
      - "Aucun"
    reponse: 2
    pourquoi: >-
      Tout injecter coûte cher et noie le modèle.
      Les plus récents ne sont pas forcément utiles.
      On cherche les faits proches de la question, souvent par embedding.
      C'est de la recherche, comme pour des documents.

  - id: semantique-4
    sous_rubrique: semantique
    question: "MemGPT (2023) compare la mémoire d'un agent à…"
    choix:
      - "Un cerveau humain"
      - "Un navigateur web"
      - "Une base SQL"
      - "Un système d'exploitation : le contexte est la RAM, le stockage externe est le disque"
    reponse: 3
    pourquoi: >-
      La RAM est rapide mais petite. Le disque est grand mais lent.
      Un système d'exploitation déplace les données de l'un à l'autre.
      MemGPT fait pareil : le modèle déplace lui-même ses souvenirs.
      Il appelle des fonctions pour ranger ou rappeler.
---

## Trois mémoires

Le modèle n'a aucune mémoire. On lui en construit une.

On reprend une distinction de la psychologie : travail, épisodes, faits.

| Mémoire | Contient | Exemple | Durée |
|---|---|---|---|
| Travail | L'état de la tâche en cours | « Étape 3 sur 5 : tests à relancer » | La tâche |
| Épisodique | Ce qui s'est passé | « Le 3 mars, commande annulée » | Semaines, mois |
| Sémantique | Ce qu'on sait | « Végétarien » | Longtemps, révisable |

:::pourquoi
Chaque mémoire répond à une question différente.
Où j'en suis ? Que s'est-il passé ? Qu'est-ce que je sais ?
Les mélanger donne un tas que personne ne sait trier.
:::

Les trois s'alimentent et se rappellent.

```mermaid
flowchart LR
  C["Conversation"] -->|"pendant la tâche"| T["Mémoire de travail"]
  C -->|"fin de session : résumé daté"| E["Mémoire épisodique"]
  E -->|"consolidation"| S["Mémoire sémantique"]
  Q["Nouvelle question"] -->|"recherche"| S
  Q -->|"recherche"| E
  S --> A["Assembleur de contexte"]
  E --> A
  T --> A
```

## Mémoire de travail {#travail}

C'est le bloc-notes de la tâche en cours : plan, étapes faites, décisions.

Elle vit le temps de la tâche.

:::pourquoi
L'historique finit par être coupé ou résumé.
Le plan risque de disparaître avec lui.
Un bloc-notes court garde l'état, quoi qu'il arrive à l'historique.
:::

## Mémoire épisodique {#episodique}

Elle garde les événements. Datés, compressés.

« 3 mars : l'utilisateur annule la commande 48213. Raison : délai trop long. »

:::pourquoi
Une conversation brute fait des milliers de tokens.
Un épisode compressé en fait quelques dizaines.
On peut en rappeler beaucoup plus.
:::

## Mémoire sémantique {#semantique}

Elle garde les faits durables. Sans date.

« Végétarien. » « Préfère les réponses courtes. » « Travaille en Rust. »

Ces faits viennent des épisodes. C'est la **consolidation**.

:::pourquoi
Un fait tiré de dix épisodes est solide.
Un fait tiré d'un seul est une hypothèse.
Garde le lien vers les épisodes qui le justifient.
:::

On n'injecte pas tous les faits. On cherche ceux qui servent la question.

:::pourquoi
Tout injecter coûte cher et noie le modèle.
Rappeler un souvenir, c'est une recherche, comme pour un document.
:::

En 2023, MemGPT pousse l'idée plus loin. Le contexte est la RAM. Le stockage est le disque. Le modèle range et rappelle lui-même, avec des outils.

:::pourquoi
Un système d'exploitation déplace déjà les données entre RAM et disque.
MemGPT donne ce rôle au modèle.
Il décide ce qui doit rester sous ses yeux.
:::

## À retenir

- Travail : où j'en suis. Épisodique : ce qui s'est passé. Sémantique : ce que je sais.
- Les épisodes se compressent. Les faits se consolident.
- Un fait doit garder ses sources.
- Rappeler, c'est chercher. On n'injecte que le pertinent.
