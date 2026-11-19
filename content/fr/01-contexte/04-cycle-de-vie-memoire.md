---
id: "01-04"
titre: "Cycle de vie de la mémoire"
sous_rubriques:
  - id: importance
    titre: "Score d'importance"
  - id: ttl
    titre: "TTL et éviction"
  - id: persistance
    titre: "Persistance entre sessions"
sources:
  - titre: "Park et al. — Generative Agents: Interactive Simulacra of Human Behavior, 2023"
    url: "https://arxiv.org/abs/2304.03442"
  - titre: "The Hacker News — ChatGPT macOS flaw could've enabled long-term spyware via memory function (SpAIware, Johann Rehberger), septembre 2024"
    url: "https://thehackernews.com/2024/09/chatgpt-macos-flaw-couldve-enabled-long.html"
qcm:
  - id: importance-1
    sous_rubrique: importance
    question: "Dans Generative Agents (Stanford, 2023), qui note l'importance d'un souvenir ?"
    choix:
      - "Le LLM lui-même, sur une échelle de 1 à 10"
      - "Un humain"
      - "Le nombre de mots du souvenir"
      - "L'heure du souvenir"
    reponse: 0
    pourquoi: >-
      Chaque souvenir est noté par le modèle quand il est créé.
      1 : banal, comme se brosser les dents.
      10 : marquant, comme une rupture ou une admission.
      Le modèle juge le sens. Une règle fixe en serait incapable.

  - id: importance-2
    sous_rubrique: importance
    question: "Dans Generative Agents, quels trois critères décident des souvenirs rappelés ?"
    choix:
      - "Longueur, langue, date"
      - "Popularité, auteur, source"
      - "Récence, importance, pertinence"
      - "Coût, latence, taille"
    reponse: 2
    pourquoi: >-
      Récence : un souvenir récemment utilisé remonte plus facilement.
      Importance : un événement marquant pèse plus.
      Pertinence : il doit parler de la situation actuelle.
      Les trois scores sont normalisés puis additionnés.

  - id: importance-3
    sous_rubrique: importance
    question: "Pourquoi ne pas rappeler seulement les souvenirs les plus pertinents ?"
    choix:
      - "La pertinence est impossible à calculer"
      - "Un souvenir pertinent mais ancien et banal peut écraser un événement récent et important"
      - "La pertinence coûte trop cher"
      - "Les modèles n'aiment pas la pertinence"
    reponse: 1
    pourquoi: >-
      La pertinence seule ignore le temps et la gravité.
      « Il a bu un café lundi » peut sembler proche de la question.
      « Il a démissionné hier » compte bien plus.
      Combiner les trois critères évite ce piège.

  - id: ttl-1
    sous_rubrique: ttl
    question: "Un TTL sur un souvenir, c'est…"
    choix:
      - "Sa taille en tokens"
      - "Son score d'importance"
      - "Sa durée de vie avant expiration"
      - "Son identifiant"
    reponse: 2
    pourquoi: >-
      TTL veut dire time to live, durée de vie.
      Passé ce délai, le souvenir expire.
      Sans expiration, la mémoire grossit sans fin.
      Et les vieux faits faux restent en place.

  - id: ttl-2
    sous_rubrique: ttl
    question: "Quelle mémoire doit avoir le TTL le plus court ?"
    choix:
      - "Mémoire de travail"
      - "Mémoire épisodique"
      - "Mémoire sémantique"
      - "Toutes le même"
    reponse: 0
    pourquoi: >-
      La mémoire de travail ne sert que pendant la tâche.
      Elle expire à la fin de la tâche.
      Les épisodes durent des semaines ou des mois.
      Les faits durent longtemps, mais restent révisables.

  - id: ttl-3
    sous_rubrique: ttl
    question: "L'utilisateur habitait Lyon. Il annonce qu'il a déménagé à Genève. Que faire ?"
    choix:
      - "Ajouter « habite Genève » à côté de « habite Lyon »"
      - "Supprimer toute la mémoire"
      - "Ignorer, la mémoire est figée"
      - "Remplacer l'ancien fait par le nouveau"
    reponse: 3
    pourquoi: >-
      Deux faits contradictoires sèment le doute.
      Le modèle peut choisir le mauvais.
      On met à jour le fait, et on garde l'épisode du déménagement.
      L'historique reste, le fait actuel est unique.

  - id: ttl-4
    sous_rubrique: ttl
    question: "La mémoire est pleine. Que retirer en premier ?"
    choix:
      - "Les plus récents"
      - "Les souvenirs au score combiné le plus bas"
      - "Au hasard"
      - "Les plus importants"
    reponse: 1
    pourquoi: >-
      On retire ce qui a le moins de chances de servir.
      Le score combiné le mesure : ancien, banal, rarement rappelé.
      C'est l'éviction.

  - id: persistance-1
    sous_rubrique: persistance
    question: "Septembre 2024 : un chercheur montre une attaque contre la mémoire de ChatGPT. Comment marche-t-elle ?"
    choix:
      - "Une page piégée fait enregistrer une instruction en mémoire, rejouée à chaque session"
      - "Il vole le mot de passe de l'utilisateur"
      - "Il efface la mémoire à distance"
      - "Il remplit la mémoire jusqu'à saturation"
    reponse: 0
    pourquoi: >-
      L'utilisateur fait lire une page ou un document piégé.
      Le texte caché pousse ChatGPT à enregistrer une instruction.
      Elle revient dans toutes les conversations suivantes.
      Johann Rehberger l'a appelée SpAIware.

  - id: persistance-2
    sous_rubrique: persistance
    question: "Quelle règle protège la mémoire persistante ?"
    choix:
      - "Tout ce que le modèle propose d'enregistrer est enregistré"
      - "La mémoire est effacée chaque jour"
      - "La mémoire est chiffrée, donc sûre"
      - "Ce qui vient d'une source non fiable n'entre pas en mémoire sans contrôle"
    reponse: 3
    pourquoi: >-
      La mémoire transforme une attaque ponctuelle en attaque permanente.
      Le chiffrement protège le stockage, pas le contenu.
      Ce qui vient d'une page, d'un mail ou d'un outil est une donnée.
      Avant d'en faire un souvenir, il faut un contrôle, ou l'accord de l'utilisateur.

  - id: persistance-3
    sous_rubrique: persistance
    question: "Deux utilisateurs utilisent le même agent. Que garantir ?"
    choix:
      - "Une mémoire commune, pour mieux apprendre"
      - "Une mémoire isolée par utilisateur"
      - "Aucune mémoire"
      - "Une mémoire par jour"
    reponse: 1
    pourquoi: >-
      Un souvenir d'un utilisateur ne doit jamais apparaître chez un autre.
      Ce serait une fuite de données personnelles.
      Chaque souvenir porte l'identifiant de son propriétaire.
      Chaque recherche filtre sur cet identifiant.

  - id: persistance-4
    sous_rubrique: persistance
    question: "Un utilisateur européen demande que l'agent oublie tout de lui. Que faire ?"
    choix:
      - "Refuser, la mémoire est technique"
      - "Masquer ses souvenirs à l'affichage"
      - "Effacer ses souvenirs dans tous les niveaux et toutes les copies"
      - "Attendre que le TTL expire"
    reponse: 2
    pourquoi: >-
      Le RGPD donne un droit à l'effacement.
      Des souvenirs sur une personne sont des données personnelles.
      Masquer ne suffit pas. Attendre non plus.
      Prévois l'effacement dès la conception, y compris dans les sauvegardes.
---

## Score d'importance {#importance}

Tout ne mérite pas d'être retenu.

En 2023, Stanford crée *Generative Agents* : 25 personnages simulés dans un petit village. Chacun a une mémoire.

Chaque souvenir reçoit une note d'importance, de 1 à 10. C'est le modèle qui note.

:::pourquoi
Une règle fixe ne sait pas qu'une rupture compte plus qu'un café.
Le modèle, lui, comprend le sens.
Une note au moment de l'enregistrement, et c'est réglé une fois pour toutes.
:::

Pour rappeler un souvenir, ils combinent trois scores.

| Score | Question | Calcul |
|---|---|---|
| Récence | Utilisé récemment ? | Décroît de 0,5 % par heure de jeu |
| Importance | Marquant ? | Note du modèle, de 1 à 10 |
| Pertinence | Lié à la situation ? | Similarité d'embeddings |

:::pourquoi
La pertinence seule ressortirait un café de lundi dernier.
La récence seule oublierait les grands événements.
Les trois ensemble imitent assez bien la mémoire humaine.
:::

## TTL et éviction {#ttl}

Une mémoire qui ne fait que grossir devient un problème.

Elle coûte. Elle ralentit la recherche. Elle garde des faits devenus faux.

Le **TTL** donne une durée de vie à chaque souvenir.

| Mémoire | TTL typique |
|---|---|
| Travail | Fin de la tâche |
| Épisodique | Semaines ou mois |
| Sémantique | Long, mais révisable |

:::pourquoi
Un plan de tâche n'a plus de valeur une fois la tâche finie.
Un épisode perd de la valeur avec le temps.
Un fait reste vrai tant qu'il n'est pas contredit.
:::

Un fait contredit est **remplacé**. Pas dupliqué.

:::pourquoi
« Habite Lyon » et « habite Genève » côte à côte : le modèle peut choisir le mauvais.
On met à jour le fait. L'épisode du déménagement garde l'historique.
:::

Quand la place manque, on **évince** les souvenirs au score combiné le plus bas.

## Persistance entre sessions {#persistance}

Une mémoire persistante survit à la fin de la session.

Elle est stockée hors du modèle : base de données, fichiers.

```mermaid
sequenceDiagram
  participant U as Utilisateur
  participant A as Application
  participant M as Mémoire
  participant L as LLM
  U->>A: Ouvre une session
  A->>M: Souvenirs utiles pour cet utilisateur ?
  M-->>A: Faits et épisodes pertinents
  A->>L: Contexte + souvenirs + question
  L-->>A: Réponse + faits à retenir
  A->>A: Contrôle des faits proposés
  A->>M: Enregistre les faits validés
```

:::pourquoi
Le modèle propose. L'application décide de ce qui entre en mémoire.
Un souvenir faux revient à chaque session.
:::

En septembre 2024, Johann Rehberger montre l'attaque *SpAIware* contre ChatGPT. Une page piégée fait enregistrer une instruction en mémoire. Elle revient dans toutes les conversations suivantes.

:::pourquoi
La mémoire rend une attaque permanente.
Ce qui vient d'une page, d'un mail ou d'un outil est une donnée.
Ça n'entre pas en mémoire sans contrôle.
:::

Une mémoire par utilisateur. Toujours isolée.

:::pourquoi
Un souvenir qui fuit chez un autre est une fuite de données personnelles.
Chaque recherche filtre par propriétaire.
:::

L'effacement doit être possible, partout.

:::pourquoi
Le RGPD donne un droit à l'effacement.
Des souvenirs sur une personne sont des données personnelles.
:::

## À retenir

- Rappeler selon trois critères : récence, importance, pertinence.
- Chaque mémoire a sa durée de vie. Un fait contredit est remplacé.
- Le modèle propose les souvenirs. L'application les contrôle.
- Mémoire isolée par utilisateur, et effaçable.
