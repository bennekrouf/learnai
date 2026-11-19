---
id: "03-05"
titre: "Détecter les hallucinations"
sous_rubriques:
  - id: coherence
    titre: "Cohérence"
  - id: juge
    titre: "Juge"
sources:
  - titre: "Manakul, Liusie, Gales — SelfCheckGPT: Zero-Resource Black-Box Hallucination Detection, 2023"
    url: "https://arxiv.org/abs/2303.08896"
  - titre: "Farquhar, Kossen, Kuhn, Gal — Detecting hallucinations in large language models using semantic entropy, Nature, juin 2024"
    url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11186750"
  - titre: "Min et al. — FActScore: Fine-grained Atomic Evaluation of Factual Precision, 2023"
    url: "https://arxiv.org/abs/2305.14251"
qcm:
  - id: coherence-1
    sous_rubrique: coherence
    question: "Principe de SelfCheckGPT (2023) ?"
    choix:
      - "Demander au modèle s'il a halluciné"
      - "Mesurer la longueur de la réponse"
      - "Comparer la réponse à Wikipédia"
      - "Générer plusieurs réponses : un fait qui varie d'une réponse à l'autre est suspect"
    reponse: 3
    pourquoi: >-
      Un fait que le modèle connaît revient à chaque tirage.
      Un fait inventé change d'un tirage à l'autre.
      On tire plusieurs réponses et on compare.
      Aucune source externe n'est nécessaire.

  - id: coherence-2
    sous_rubrique: coherence
    question: "Trois tirages répondent « Paris », « la capitale française » et « Paris, en France ». Sont-ils cohérents ?"
    choix:
      - "Non, les mots diffèrent"
      - "Oui, ils ont le même sens"
      - "Non, il en faut 10"
      - "Impossible à dire"
    reponse: 1
    pourquoi: >-
      Les mots changent, le sens non.
      C'est l'idée de l'entropie sémantique (Nature, 2024).
      On regroupe les réponses par sens, pas par texte.
      Si toutes tombent dans le même groupe, le modèle est stable.

  - id: coherence-3
    sous_rubrique: coherence
    question: "Entropie sémantique élevée, ça veut dire…"
    choix:
      - "Les réponses tirées ont des sens très différents : risque d'invention"
      - "Le modèle est très sûr"
      - "La réponse est longue"
      - "La question est en anglais"
    reponse: 0
    pourquoi: >-
      L'entropie mesure la dispersion.
      Beaucoup de sens différents : le modèle improvise.
      Les auteurs appellent ces inventions arbitraires des confabulations.
      On peut alors s'abstenir ou aller chercher une source.

  - id: coherence-4
    sous_rubrique: coherence
    question: "Quelle hallucination la cohérence ne détecte-t-elle pas ?"
    choix:
      - "Une invention qui change à chaque tirage"
      - "Une date inventée"
      - "Une erreur que le modèle répète à chaque fois, parce qu'il l'a apprise"
      - "Un nom inventé"
    reponse: 2
    pourquoi: >-
      Si le modèle a appris un fait faux, il le répète avec constance.
      Tous les tirages sont d'accord, et tous ont tort.
      La cohérence mesure l'improvisation, pas la vérité.
      D'où l'intérêt de vérifier aussi contre des sources.

  - id: coherence-5
    sous_rubrique: coherence
    question: "Le coût principal des méthodes de cohérence ?"
    choix:
      - "Plusieurs générations par question : coût et latence multipliés"
      - "Aucun"
      - "Un gros index"
      - "Une base de données externe"
    reponse: 0
    pourquoi: >-
      Cinq tirages coûtent environ cinq fois une réponse.
      On les réserve aux cas où une erreur coûte cher.
      Ou on les lance après coup, sur un échantillon.

  - id: juge-1
    sous_rubrique: juge
    question: "FActScore (2023) évalue un texte en…"
    choix:
      - "Une note globale de 1 à 10"
      - "Mesurant sa ressemblance avec la question"
      - "Comptant les mots rares"
      - "Le découpant en faits atomiques, vérifiés un par un contre une source"
    reponse: 3
    pourquoi: >-
      Une biographie contient des dizaines de faits.
      Une note globale cache les erreurs isolées.
      Un fait atomique se vérifie : soutenu ou non.
      Le score est la part de faits soutenus.

  - id: juge-2
    sous_rubrique: juge
    question: "Un LLM juge vérifie les affirmations. Que doit-il recevoir ?"
    choix:
      - "Seulement la réponse"
      - "Chaque affirmation et les passages sources correspondants"
      - "Seulement la question"
      - "Le prompt système du premier modèle"
    reponse: 1
    pourquoi: >-
      Sans les sources, le juge vérifie de mémoire.
      Il peut alors halluciner à son tour.
      Avec les passages, il compare un texte à un texte.
      C'est une tâche bien plus fiable.

  - id: juge-3
    sous_rubrique: juge
    question: "Peut-on faire confiance au juge sans contrôle ?"
    choix:
      - "Oui, c'est un LLM"
      - "Oui, s'il est plus grand que le premier modèle"
      - "Non : il faut mesurer son accord avec des verdicts humains"
      - "Oui, à température 0"
    reponse: 2
    pourquoi: >-
      Le juge est un modèle. Il se trompe aussi.
      On prend une centaine de cas annotés par des humains.
      On mesure combien de fois le juge est d'accord.
      C'est le calibrage du juge, à l'étape 7.

  - id: juge-4
    sous_rubrique: juge
    question: "Vérifier chaque réponse ajoute de la latence. Quelle stratégie ?"
    choix:
      - "Ne jamais vérifier"
      - "Vérifier une fois par mois"
      - "Vérifier avant d'afficher pour les sujets à risque, et après coup sur un échantillon pour le reste"
      - "Laisser l'utilisateur vérifier"
    reponse: 2
    pourquoi: >-
      Un conseil juridique ou médical mérite d'attendre la vérification.
      Une question banale peut s'afficher tout de suite.
      L'échantillon après coup mesure la qualité dans le temps.
      Il alimente aussi le jeu de test.
---

## Cohérence {#coherence}

Idée simple : pose la même question plusieurs fois.

Un fait connu revient. Un fait inventé change.

:::pourquoi
Quand le modèle sait, la réponse la plus probable domine.
Quand il improvise, plusieurs réponses se valent, et le tirage varie.
:::

**SelfCheckGPT** (2023) compare plusieurs tirages entre eux. Pas besoin de source externe.

**L'entropie sémantique** (Nature, 2024) regroupe les tirages **par sens**. Beaucoup de sens différents : risque d'invention.

```mermaid
flowchart LR
  Q["Question"] --> L["LLM, 3 tirages"]
  L --> T1["Paris"]
  L --> T2["La capitale française"]
  L --> T3["Lyon"]
  T1 --> G["Regroupement par sens"]
  T2 --> G
  T3 --> G
  G --> E["2 sens : entropie moyenne"]
```

:::pourquoi
« Paris » et « la capitale française » disent la même chose.
Compter les textes différents gonflerait l'incertitude.
Compter les sens différents mesure la vraie hésitation.
:::

Limite : une erreur apprise est répétée avec constance.

:::pourquoi
Tous les tirages sont d'accord, et tous ont tort.
La cohérence détecte l'improvisation, pas l'erreur apprise.
:::

Coût : plusieurs générations par question.

## Juge {#juge}

Un second modèle vérifie la réponse contre les sources.

```mermaid
flowchart LR
  R["Réponse"] --> D["Découpage en affirmations"]
  D --> J["LLM juge"]
  S["Passages sources"] --> J
  J --> V["Verdicts : soutenue, contredite, absente"]
  V --> X["Décision : afficher, corriger, s'abstenir"]
```

:::pourquoi
Comparer une affirmation à un passage est une tâche simple.
Bien plus simple que produire la réponse.
Un modèle plus petit peut souvent la faire.
:::

**FActScore** (2023) formalise l'idée : des faits atomiques, vérifiés un par un.

:::pourquoi
Une note globale cache une erreur isolée.
Un verdict par fait la montre.
:::

Le juge doit recevoir les sources.

:::pourquoi
Sans elles, il vérifie de mémoire.
Il peut halluciner à son tour.
:::

Le juge se trompe aussi. On mesure son accord avec des humains.

:::pourquoi
Un juge non calibré donne une fausse sécurité.
C'est le sujet de l'étape 7.
:::

Où placer la vérification ?

| Cas | Stratégie |
|---|---|
| Sujet à risque : santé, droit, argent | Vérifier avant d'afficher |
| Question courante | Afficher, vérifier un échantillon après coup |

## À retenir

- Cohérence : un fait qui varie entre tirages est suspect.
- On compare des sens, pas des mots.
- La cohérence rate les erreurs apprises. Le juge avec sources les attrape.
- Le juge se calibre contre des humains.
