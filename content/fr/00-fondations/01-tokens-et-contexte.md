---
id: "00-01"
titre: "Tokens et contexte"
sous_rubriques:
  - id: tokens
    titre: "Ce qu'est un token"
  - id: fenetre
    titre: "La fenêtre de contexte"
  - id: cout
    titre: "Entrée, sortie et coût"
sources:
  - titre: "Rumbelow & Watkins — SolidGoldMagikarp (plus, prompt generation), SERI-MATS, 2023"
    url: "https://www.greaterwrong.com/posts/aPeJE8bSo6rAFoLqg/"
qcm:
  - id: tokens-1
    sous_rubrique: tokens
    question: "Tu écris « strawberry ». Que reçoit réellement le modèle ?"
    choix:
      - "Dix lettres, une par une"
      - "Une image du mot"
      - "Une suite de numéros de tokens"
      - "Un vecteur par lettre"
    reponse: 2
    pourquoi: >-
      Le texte passe d'abord par un tokenizer.
      Il le découpe en morceaux fréquents.
      Chaque morceau devient un numéro.
      Le modèle ne voit que ces numéros.
      Les lettres ont disparu avant même qu'il commence.

  - id: tokens-2
    sous_rubrique: tokens
    question: "Même phrase, en anglais et en français. Laquelle produit en général le plus de tokens ?"
    choix:
      - "Le français"
      - "L'anglais"
      - "Aucune différence"
      - "Cela dépend seulement du nombre de caractères"
    reponse: 0
    pourquoi: >-
      Les tokenizers sont surtout construits sur de l'anglais.
      Un mot anglais courant tient souvent en un token.
      Un mot français est plus souvent coupé en plusieurs.
      Même sens, plus de tokens.
      Donc plus cher et plus de place prise dans le contexte.

  - id: tokens-3
    sous_rubrique: tokens
    question: "En anglais, 1 000 tokens représentent environ…"
    choix:
      - "250 mots"
      - "4 000 mots"
      - "1 000 mots"
      - "750 mots"
    reponse: 3
    pourquoi: >-
      Un token fait en moyenne 4 caractères en anglais.
      Soit environ trois quarts de mot.
      C'est un ordre de grandeur, pas une règle.
      En français, compte plutôt moins de mots pour 1 000 tokens.

  - id: tokens-4
    sous_rubrique: tokens
    question: "En 2023, le mot « SolidGoldMagikarp » fait dérailler GPT-3. Pourquoi ?"
    choix:
      - "C'est un mot interdit par OpenAI"
      - "C'est un token du vocabulaire, presque jamais vu à l'entraînement"
      - "Il est trop long pour le modèle"
      - "Il cache une injection de prompt"
    reponse: 1
    pourquoi: >-
      Le vocabulaire de tokens a été construit sur un corpus.
      Le modèle a été entraîné sur un autre.
      Ce pseudo était fréquent dans le premier, quasi absent du second.
      Le modèle a donc un token qu'il n'a jamais appris à utiliser.
      Face à lui, il produit n'importe quoi.

  - id: fenetre-1
    sous_rubrique: fenetre
    question: "La fenêtre de contexte limite…"
    choix:
      - "Le prompt et la réponse ensemble"
      - "Seulement le prompt"
      - "Seulement la réponse"
      - "Le nombre de messages"
    reponse: 0
    pourquoi: >-
      Le modèle écrit token par token.
      Chaque token écrit est ajouté au contexte pour écrire le suivant.
      La réponse occupe donc la même fenêtre que le prompt.
      Un prompt qui remplit presque tout laisse peu de place pour répondre.

  - id: fenetre-2
    sous_rubrique: fenetre
    question: "Le modèle se souvient-il de ta conversation d'hier ?"
    choix:
      - "Oui, il garde une mémoire par utilisateur"
      - "Oui, s'il a été entraîné depuis"
      - "Non, l'application renvoie l'historique à chaque appel"
      - "Non, sauf si la température est à 0"
    reponse: 2
    pourquoi: >-
      Le modèle n'a pas d'état entre deux appels.
      Chaque appel repart de zéro.
      L'impression de mémoire vient de l'application.
      Elle renvoie tout l'historique à chaque message.
      Sans cela, le modèle ne sait rien de ce qui précède.

  - id: fenetre-3
    sous_rubrique: fenetre
    question: "Ton prompt dépasse la fenêtre de contexte. Que fait l'API ?"
    choix:
      - "Elle lit le début et ignore le reste"
      - "Elle refuse la requête avec une erreur"
      - "Elle résume le surplus automatiquement"
      - "Elle bascule sur un plus grand modèle"
    reponse: 1
    pourquoi: >-
      L'API ne devine pas quoi couper.
      Elle renvoie une erreur.
      Choisir ce qu'on garde, c'est le travail de ton application.
      Certaines interfaces de chat coupent en silence. L'API, non.
      C'est le sujet de l'étape 1.

  - id: cout-1
    sous_rubrique: cout
    question: "Quel token coûte le plus cher ?"
    choix:
      - "Un token d'entrée"
      - "Le premier token du prompt"
      - "Le même prix"
      - "Un token de sortie"
    reponse: 3
    pourquoi: >-
      L'entrée est traitée d'un bloc, en parallèle.
      La sortie est produite un token à la fois.
      Chaque token de sortie demande un passage complet dans le modèle.
      Plus de calcul, donc un prix plusieurs fois plus élevé.

  - id: cout-2
    sous_rubrique: cout
    question: "Ta réponse met 20 secondes. Quel levier agit le plus sur la latence ?"
    choix:
      - "Écrire le prompt en anglais"
      - "Raccourcir le prompt"
      - "Baisser la température"
      - "Raccourcir la réponse demandée"
    reponse: 3
    pourquoi: >-
      La sortie s'écrit token par token.
      Le temps total suit donc la longueur de la réponse.
      Un prompt plus court aide, mais beaucoup moins.
      La température ne change pas la vitesse.

  - id: cout-3
    sous_rubrique: cout
    question: "Une conversation coûte un peu plus cher à chaque nouveau message. Pourquoi ?"
    choix:
      - "Tout l'historique est renvoyé, et facturé, à chaque appel"
      - "Le fournisseur augmente ses prix avec l'usage"
      - "Le modèle réfléchit plus longtemps"
      - "Les tokens anciens coûtent plus cher"
    reponse: 0
    pourquoi: >-
      Le modèle n'a pas de mémoire.
      L'application renvoie toute la conversation à chaque tour.
      Chaque message ajoute des tokens d'entrée à tous les appels suivants.
      Le coût d'un tour grandit avec la longueur de la conversation.
---

## Ce qu'est un token {#tokens}

Un modèle ne lit pas de lettres. Il lit des tokens.

Un token est un morceau de texte fréquent. Un mot, un bout de mot, un espace suivi d'un mot.

:::pourquoi
Travailler lettre par lettre ferait des séquences très longues.
Travailler mot par mot exigerait un vocabulaire infini.
Le token est le compromis : un vocabulaire fixe, des séquences courtes.
:::

Le **tokenizer** fait le découpage. Il transforme chaque morceau en numéro.

```mermaid
flowchart LR
  T["Texte : strawberry"] --> K["Tokenizer"]
  K --> N["Numéros de tokens"]
  N --> M["Modèle"]
  M --> S["Numéros de sortie"]
  S --> K2["Tokenizer (sens inverse)"]
  K2 --> R["Texte de réponse"]
```

Le modèle ne voit jamais le texte. Seulement les numéros.

:::pourquoi
C'est pour ça qu'il rate « combien de r dans strawberry ».
L'information « lettre » a été effacée avant lui.
Pour compter des lettres, il faut un outil qui voit les lettres.
:::

Ordre de grandeur : en anglais, 1 token ≈ 4 caractères ≈ ¾ de mot.

Le français coûte plus de tokens que l'anglais pour le même sens.

:::pourquoi
Le vocabulaire est appris sur un corpus surtout anglais.
Les mots anglais fréquents y ont leur propre token.
Les mots français sont plus souvent coupés.
:::

Le vocabulaire peut contenir des tokens presque jamais vus à l'entraînement. En 2023, des chercheurs en trouvent dans GPT-3 : « SolidGoldMagikarp » en est le plus célèbre. Ces tokens le font dérailler.

:::pourquoi
Le vocabulaire et le modèle n'ont pas appris sur les mêmes données.
Un token présent dans l'un mais absent de l'autre n'a pas de sens appris.
:::

## La fenêtre de contexte {#fenetre}

La fenêtre de contexte, c'est tout ce que le modèle voit en une fois.

Elle se compte en tokens.

Elle contient **le prompt et la réponse**.

:::pourquoi
Le modèle écrit un token, l'ajoute au contexte, puis écrit le suivant.
La réponse grandit donc dans la même fenêtre que la question.
:::

Le modèle n'a pas de mémoire entre deux appels.

```mermaid
sequenceDiagram
  participant U as Utilisateur
  participant A as Application
  participant L as LLM
  U->>A: Message 1
  A->>L: Message 1
  L-->>A: Réponse 1
  U->>A: Message 2
  A->>L: Message 1 + Réponse 1 + Message 2
  L-->>A: Réponse 2
```

:::pourquoi
Chaque appel est indépendant.
C'est l'application qui renvoie l'historique.
La « mémoire » d'un chat est donc un choix d'application, pas une propriété du modèle.
:::

Si le contenu dépasse la fenêtre, l'API renvoie une erreur.

:::pourquoi
L'API ne sait pas ce qui compte pour toi.
Elle refuse au lieu de couper au hasard.
Décider quoi garder est ton travail. C'est l'étape 1.
:::

## Entrée, sortie et coût {#cout}

Les API facturent au million de tokens. L'entrée et la sortie ont deux prix différents.

Un token de sortie coûte plusieurs fois plus cher qu'un token d'entrée.

:::pourquoi
L'entrée est lue d'un seul bloc, en parallèle.
La sortie est écrite un token à la fois.
Chaque token écrit demande un passage complet dans le modèle.
:::

La longueur de la réponse fait la latence.

:::pourquoi
Mêmes raisons : la sortie est séquentielle.
Une réponse deux fois plus longue prend environ deux fois plus de temps.
:::

Une conversation longue coûte de plus en plus cher à chaque tour.

:::pourquoi
L'historique entier est renvoyé à chaque appel.
Il est facturé à chaque fois.
:::

## À retenir

- Le modèle voit des numéros de tokens, pas des lettres.
- La fenêtre contient la question **et** la réponse.
- Le modèle n'a aucune mémoire. L'application la simule.
- La sortie coûte plus cher et fait la latence.
