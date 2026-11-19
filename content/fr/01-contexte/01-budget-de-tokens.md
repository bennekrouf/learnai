---
id: "01-01"
titre: "Budget de tokens"
sous_rubriques:
  - id: repartir
    titre: "Répartir la fenêtre"
  - id: sortie
    titre: "Réserver la sortie"
sources:
  - titre: "Liu et al. — Lost in the Middle: How Language Models Use Long Contexts, TACL 2024"
    url: "https://arxiv.org/abs/2307.03172"
  - titre: "Hong, Troynikov, Huber (Chroma) — Context Rot: How Increasing Input Tokens Impacts LLM Performance, juillet 2025"
    url: "https://research.trychroma.com/context-rot"
qcm:
  - id: repartir-1
    sous_rubrique: repartir
    question: "Ton modèle accepte 200 000 tokens. Faut-il remplir la fenêtre au maximum ?"
    choix:
      - "Oui, plus de contexte donne de meilleures réponses"
      - "Non, l'API refuse au-delà de 50 %"
      - "Oui, le prix ne dépend pas de la longueur"
      - "Non, la qualité baisse souvent quand l'entrée s'allonge"
    reponse: 3
    pourquoi: >-
      En 2025, Chroma teste 18 modèles sur des tâches simples.
      Plus l'entrée est longue, moins les résultats sont fiables.
      Chaque token en plus coûte aussi de l'argent et du temps.
      La fenêtre est un plafond, pas un objectif.

  - id: repartir-2
    sous_rubrique: repartir
    question: "L'information clé est dans un long contexte. Où le modèle l'utilise-t-il le moins bien ?"
    choix:
      - "Au milieu"
      - "Au début"
      - "À la fin"
      - "Partout pareil"
    reponse: 0
    pourquoi: >-
      L'étude Lost in the Middle mesure une courbe en U.
      Les modèles utilisent bien le début et la fin.
      Ils utilisent moins bien ce qui est au milieu.
      Place donc l'essentiel aux extrémités.

  - id: repartir-3
    sous_rubrique: repartir
    question: "Tu ajoutes 10 documents peu pertinents à côté du bon. La bonne réponse est toujours là. Que se passe-t-il ?"
    choix:
      - "Rien, le modèle trouve le bon document"
      - "Le modèle refuse de répondre"
      - "Le modèle se trompe plus souvent"
      - "La réponse devient plus complète"
    reponse: 2
    pourquoi: >-
      Un document proche mais faux est un distracteur.
      Chroma montre que plus il y a de distracteurs, plus la qualité baisse.
      Avoir la bonne réponse dans le contexte ne suffit pas.
      Il faut aussi retirer ce qui la brouille.

  - id: repartir-4
    sous_rubrique: repartir
    question: "Les définitions de tes 40 outils consomment-elles des tokens ?"
    choix:
      - "Non, elles sont gratuites"
      - "Oui, à chaque appel"
      - "Seulement au premier appel"
      - "Seulement si un outil est utilisé"
    reponse: 1
    pourquoi: >-
      Les outils sont décrits dans l'entrée du modèle.
      Le modèle n'a pas de mémoire, on les renvoie à chaque appel.
      40 outils détaillés peuvent coûter des milliers de tokens.
      Ne charge que les outils utiles à la tâche.

  - id: repartir-5
    sous_rubrique: repartir
    question: "Comment mesurer la taille de ton prompt en tokens ?"
    choix:
      - "Nombre de caractères divisé par 4"
      - "Nombre de mots"
      - "Avec le tokenizer ou l'API de comptage du modèle"
      - "Nombre de lignes multiplié par 10"
    reponse: 2
    pourquoi: >-
      Diviser par 4 marche en moyenne, en anglais.
      Le français, le code et le JSON s'en écartent beaucoup.
      Chaque modèle a son propre tokenizer.
      Pour un budget fiable, compte avec l'outil du modèle.

  - id: sortie-1
    sous_rubrique: sortie
    question: "Fenêtre de 200 000 tokens. Ton prompt en fait 190 000. Tu attends une réponse de 15 000. Problème ?"
    choix:
      - "La réponse ne tient pas dans la fenêtre"
      - "Aucun"
      - "Le prompt sera résumé"
      - "La réponse sera plus rapide"
    reponse: 0
    pourquoi: >-
      La réponse grandit dans la même fenêtre que le prompt.
      Il ne reste que 10 000 tokens.
      La réponse sera coupée, ou la requête refusée.
      Calcule toujours : entrée plus sortie maximale.

  - id: sortie-2
    sous_rubrique: sortie
    question: "Un modèle qui « réfléchit » avant de répondre. Quel impact sur ton budget ?"
    choix:
      - "Aucun, la réflexion est gratuite"
      - "La réflexion se fait hors de la fenêtre"
      - "La réflexion réduit l'entrée"
      - "La réflexion consomme aussi des tokens de sortie"
    reponse: 3
    pourquoi: >-
      Le raisonnement est du texte généré.
      Il est compté et facturé comme une sortie.
      Il prend de la place avant même la réponse visible.
      Prévois une réserve plus grande.

  - id: sortie-3
    sous_rubrique: sortie
    question: "Qui doit réserver la place de la sortie ?"
    choix:
      - "Le fournisseur, automatiquement"
      - "Ton application, avant d'envoyer la requête"
      - "Le modèle, en raccourcissant sa réponse"
      - "Personne"
    reponse: 1
    pourquoi: >-
      Le fournisseur ne sait pas quelle longueur tu attends.
      Le modèle ne sait pas combien de place il lui reste.
      Seule ton application connaît les deux.
      Elle calcule le budget avant chaque appel.
---

## Répartir la fenêtre {#repartir}

La fenêtre est partagée entre plusieurs occupants.

```mermaid
flowchart LR
  C["Consignes système"] --> F["Fenêtre de contexte"]
  O["Définitions d'outils"] --> F
  M["Mémoire"] --> F
  D["Documents retrouvés"] --> F
  H["Historique"] --> F
  F --> R["Réserve pour la réponse"]
```

Chaque occupant a une part. Ensemble, ils ne doivent pas dépasser la fenêtre.

:::pourquoi
Sans budget, le plus gros gagne.
Un seul résultat d'outil énorme peut chasser tout l'historique.
Un budget par occupant garde l'équilibre.
:::

Plus de contexte ne veut pas dire mieux.

:::pourquoi
En 2025, Chroma teste 18 modèles.
Plus l'entrée s'allonge, moins les réponses sont fiables. Même sur des tâches simples.
Les passages proches mais faux, les distracteurs, aggravent l'effet.
:::

La position compte aussi. Le milieu est la zone la moins bien lue.

:::pourquoi
L'étude Lost in the Middle (2023) trace une courbe en U.
Le début et la fin sont bien utilisés. Le milieu, moins.
:::

Les outils prennent de la place. Leurs définitions sont renvoyées à chaque appel.

:::pourquoi
Le modèle n'a pas de mémoire.
Tout ce qu'il doit savoir, il le reçoit à chaque fois.
:::

Mesure en tokens, avec le tokenizer du modèle. Pas en caractères.

:::pourquoi
La règle « 4 caractères = 1 token » vaut pour l'anglais courant.
Le code, le JSON et le français s'en écartent.
:::

## Réserver la sortie {#sortie}

La réponse vit dans la même fenêtre que le prompt.

Réserve sa place **avant** de remplir le reste.

:::pourquoi
Si l'entrée prend tout, la réponse est coupée ou refusée.
Une réponse coupée ressemble à une réponse normale.
:::

La formule : entrée + sortie maximale ≤ fenêtre.

Les modèles qui réfléchissent ont besoin d'une réserve plus grande.

:::pourquoi
Leur raisonnement est du texte généré.
Il compte comme sortie, avant la réponse visible.
:::

C'est ton application qui fait ce calcul.

:::pourquoi
Elle seule connaît la taille de l'entrée et la longueur attendue.
:::

## À retenir

- La fenêtre est un plafond, pas un objectif.
- Plus d'entrée, plus de distracteurs : moins de fiabilité.
- Le milieu est la zone la moins bien lue.
- Réserve d'abord la sortie. Compte avec le tokenizer du modèle.
