---
id: "07-03"
titre: "LLM juge"
sous_rubriques:
  - id: grille
    titre: "Grille"
  - id: calibrage
    titre: "Calibrage"
  - id: biais
    titre: "Biais"
sources:
  - titre: "Zheng et al. — Judging LLM-as-a-Judge with MT-Bench and Chatbot Arena, 2023"
    url: "https://arxiv.org/abs/2306.05685"
  - titre: "Shankar et al. — Who Validates the Validators?, 2024"
    url: "https://arxiv.org/abs/2404.12272"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, 2 mai 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
qcm:
  - id: grille-1
    sous_rubrique: grille
    question: "Quand utiliser un LLM comme juge ?"
    choix:
      - "Toujours, même quand le code suffit"
      - "Pour comparer deux nombres"
      - "Pour valider un JSON"
      - "Quand le critère ne se vérifie pas par code : ancrage, ton, pertinence, complétude d'un texte"
    reponse: 3
    pourquoi: >-
      « La réponse est-elle polie ? » ne se teste pas avec un if.
      Un juge LLM peut lire et apprécier.
      Mais il coûte, il varie, il a des biais.
      Quand le code suffit, le code gagne.

  - id: grille-2
    sous_rubrique: grille
    question: "Que doit contenir la grille donnée au juge ?"
    choix:
      - "« Note cette réponse de 1 à 10 »"
      - "Un critère à la fois, une définition précise, des exemples de réussite et d'échec, une sortie structurée"
      - "Le prompt système de l'agent uniquement"
      - "Rien, le juge sait"
    reponse: 1
    pourquoi: >-
      « Note de 1 à 10 » laisse le juge inventer ses critères.
      Un critère précis donne des notes reproductibles.
      Les exemples montrent où passe la limite.
      La sortie structurée se lit par le code.

  - id: grille-3
    sous_rubrique: grille
    question: "Pourquoi demander au juge une note binaire (réussi / raté) plutôt qu'une note de 1 à 10 ?"
    choix:
      - "Une décision binaire bien définie est plus stable et plus facile à vérifier contre un humain"
      - "C'est moins cher en tokens, uniquement"
      - "Les juges ne savent pas compter jusqu'à 10"
      - "Aucune raison"
    reponse: 0
    pourquoi: >-
      Entre 6 et 7, le juge hésite, et change d'avis d'un appel à l'autre.
      Entre « cite une source qui ne le dit pas » et non, c'est net.
      On compare facilement avec l'avis d'un humain.
      Plusieurs critères binaires valent mieux qu'une note floue.

  - id: grille-4
    sous_rubrique: grille
    question: "Pourquoi demander au juge une justification avant sa note ?"
    choix:
      - "Pour le facturer plus"
      - "Pour allonger la réponse"
      - "Il raisonne avant de trancher, et on peut lire pourquoi il a noté ainsi"
      - "Ce n'est jamais utile"
    reponse: 2
    pourquoi: >-
      La justification force le juge à regarder les éléments.
      Elle permet de repérer un juge qui se trompe de critère.
      Quand on n'est pas d'accord avec lui, on sait pourquoi.

  - id: calibrage-1
    sous_rubrique: calibrage
    question: "Comment savoir si ton juge LLM est fiable ?"
    choix:
      - "Lui demander s'il est fiable"
      - "Comparer ses notes à celles d'humains sur un échantillon étiqueté"
      - "Utiliser le plus gros modèle et ne rien vérifier"
      - "Regarder s'il répond vite"
    reponse: 1
    pourquoi: >-
      Le juge est un modèle. Il peut se tromper.
      Des humains notent 100 à 200 cas.
      On mesure l'accord entre juge et humains.
      Sans ça, on ne sait pas ce que vaut le score.

  - id: calibrage-2
    sous_rubrique: calibrage
    question: "Ton juge a 90 % d'accord avec les humains, mais 90 % des réponses sont bonnes. Que vérifier ?"
    choix:
      - "Sa température"
      - "Rien, 90 % c'est excellent"
      - "Sa vitesse"
      - "Sa capacité à repérer les échecs : un juge qui dit toujours « réussi » aurait aussi 90 %"
    reponse: 3
    pourquoi: >-
      Avec 90 % de bonnes réponses, dire toujours « oui » donne 90 % d'accord.
      Ce juge ne détecte aucun échec.
      On mesure séparément : échecs trouvés, et réussites reconnues.
      Ou un indicateur qui corrige le hasard, comme le kappa de Cohen.

  - id: calibrage-3
    sous_rubrique: calibrage
    question: "Selon Zheng et al. (2023), quel accord un juge fort comme GPT-4 atteint-il avec les préférences humaines ?"
    choix:
      - "100 %"
      - "Environ 20 %"
      - "Plus de 80 %, du même ordre que l'accord entre humains"
      - "Moins que le hasard"
    reponse: 2
    pourquoi: >-
      C'est ce qui a rendu les juges LLM populaires.
      Mais c'est sur leur tâche, avec leur protocole.
      Sur ta tâche, il faut mesurer à nouveau.
      Un chiffre d'article ne calibre pas ton juge.

  - id: calibrage-4
    sous_rubrique: calibrage
    question: "Tu changes le modèle ou le prompt du juge. Que faire ?"
    choix:
      - "Le recalibrer sur l'échantillon étiqueté par les humains"
      - "Rien, c'est le même juge"
      - "Relancer l'agent"
      - "Supprimer les anciens scores"
    reponse: 0
    pourquoi: >-
      Un autre juge, c'est une autre règle graduée.
      Les anciens scores ne sont plus comparables.
      On remesure l'accord avec les humains.
      Et on fige la version du juge dans l'évaluation.

  - id: biais-1
    sous_rubrique: biais
    question: "Qu'est-ce que le biais de position d'un juge qui compare deux réponses A et B ?"
    choix:
      - "Il ne lit que la fin"
      - "Il préfère les réponses courtes"
      - "Il favorise une réponse selon qu'elle est présentée en premier ou en second"
      - "Il préfère les réponses en anglais"
    reponse: 2
    pourquoi: >-
      Zheng et al. l'ont mesuré en 2023.
      Échanger A et B peut changer le verdict.
      Parade : juger dans les deux ordres.
      Si les verdicts divergent, c'est une égalité.

  - id: biais-2
    sous_rubrique: biais
    question: "Le juge préfère les réponses plus longues, même sans plus d'information. Quel biais ?"
    choix:
      - "Le biais de verbosité"
      - "Le biais de position"
      - "Le biais de récence"
      - "Le biais de confirmation"
    reponse: 0
    pourquoi: >-
      Plus long semble plus complet.
      Le juge se laisse impressionner.
      L'agent optimisé contre ce juge apprend à délayer.
      Parade : critère explicite « concis », et contrôle de la longueur.

  - id: biais-3
    sous_rubrique: biais
    question: "Le juge et l'agent sont le même modèle. Quel risque ?"
    choix:
      - "Le juge refuse de noter"
      - "Aucun risque"
      - "Le juge plante"
      - "Le biais d'auto-complaisance : le juge favorise les réponses qui lui ressemblent"
    reponse: 3
    pourquoi: >-
      Zheng et al. le classent parmi les biais des juges.
      Un modèle trouve naturel ce qu'il aurait écrit.
      Parade : un juge d'une autre famille, et le calibrage humain.

  - id: biais-4
    sous_rubrique: biais
    question: "La mise à jour GPT-4o d'avril 2025 a utilisé les pouces des utilisateurs comme signal. Quel lien avec les juges ?"
    choix:
      - "Aucun lien"
      - "Tout juge, humain ou modèle, a des préférences qu'un système optimisé contre lui va exploiter"
      - "Les pouces sont toujours fiables"
      - "Les juges LLM n'ont pas ce problème"
    reponse: 1
    pourquoi: >-
      Les utilisateurs préfèrent qu'on leur donne raison.
      Optimiser leur approbation a renforcé la complaisance.
      Un juge LLM qui aime les réponses longues pousse à délayer.
      Optimiser une note, c'est aussi optimiser ses défauts.
---

## Grille {#grille}

Certains critères ne se vérifient pas par code : ancrage, ton, pertinence. On demande alors à un **LLM juge**.

:::pourquoi
« La réponse est-elle polie ? » ne se teste pas avec un `if`.
Mais un juge coûte, varie et a des biais.
Le code reste prioritaire quand il suffit.
:::

Une bonne grille :

```text
Critère : ANCRAGE
Définition : chaque affirmation factuelle de la réponse est soutenue
par au moins un des documents fournis.
Réussi si : toutes les affirmations sont soutenues.
Raté si : au moins une affirmation n'est soutenue par aucun document.
Exemple raté : la réponse dit « livraison en 24 h », aucun document ne le dit.
Sortie : {"justification": "...", "verdict": "reussi" | "rate"}
```

:::pourquoi
Un critère à la fois : le juge ne mélange pas.
Une définition et des exemples : il sait où passe la limite.
Une sortie structurée : le code la lit.
:::

Préfère des verdicts **binaires**, un par critère.

:::pourquoi
Entre 6 et 7 sur 10, le juge hésite et change d'avis.
Réussi ou raté se reproduit, et se compare à un humain.
:::

Demande la **justification avant** le verdict.

:::pourquoi
Le juge regarde les éléments avant de trancher.
Et toi, tu peux lire pourquoi il a tranché.
:::

## Calibrage {#calibrage}

Le juge est un modèle. Il faut le **vérifier**.

```mermaid
sequenceDiagram
  participant H as Humains
  participant E as Échantillon (150 cas)
  participant J as LLM juge
  participant M as Mesure d'accord
  H->>E: Étiquettent chaque cas
  J->>E: Note chaque cas
  E->>M: Verdicts humains et verdicts du juge
  M-->>J: Échecs trouvés, réussites reconnues, kappa
  Note over J,M: Accord insuffisant : on corrige la grille, puis on remesure
```

:::pourquoi
Sans calibrage, on ne sait pas ce que vaut le score.
On mesure peut-être le juge, pas l'agent.
:::

L'accord brut peut tromper.

| Mesure | Question |
|---|---|
| Échecs trouvés | Parmi les vrais échecs, combien le juge en voit ? |
| Réussites reconnues | Parmi les vraies réussites, combien il valide ? |
| Kappa de Cohen | Accord, une fois retiré l'accord dû au hasard |

:::pourquoi
Si 90 % des réponses sont bonnes, un juge qui dit toujours « réussi » a 90 % d'accord.
Et ne trouve aucun échec.
:::

Zheng et ses coauteurs (2023) mesurent plus de 80 % d'accord entre GPT-4 et les humains, comme entre humains.

:::pourquoi
C'est encourageant, mais c'était leur tâche.
Sur la tienne, il faut mesurer à nouveau.
:::

Chaque changement de modèle ou de grille du juge impose un **nouveau calibrage**.

:::pourquoi
Un autre juge, c'est une autre règle graduée.
Les anciens scores ne se comparent plus.
:::

## Biais {#biais}

Les juges LLM ont des biais connus.

| Biais | Effet | Parade |
|---|---|---|
| Position | Préfère A ou B selon l'ordre | Juger dans les deux ordres |
| Verbosité | Préfère le plus long | Critère « concis », contrôle de longueur |
| Auto-complaisance | Préfère son propre style | Juge d'une autre famille de modèle |
| Raisonnement limité | Rate une erreur de calcul ou de logique | Vérification par code quand c'est possible |

:::pourquoi
Zheng et ses coauteurs listent ces quatre limites.
Chaque parade retire une source d'erreur connue.
:::

Optimiser contre un juge, c'est optimiser ses **défauts** aussi.

:::pourquoi
GPT-4o a été entraîné aussi sur les pouces des utilisateurs.
Les utilisateurs aiment qu'on leur donne raison.
Le modèle est devenu complaisant.
Un juge qui aime les réponses longues apprend à l'agent à délayer.
:::

## À retenir

- Le juge LLM pour ce que le code ne sait pas juger.
- Grille : un critère, une définition, des exemples, un verdict binaire, une justification d'abord.
- Calibrer contre des humains. Mesurer les échecs trouvés, pas seulement l'accord.
- Connaître les biais : position, verbosité, auto-complaisance. Optimiser un juge, c'est optimiser ses défauts.
