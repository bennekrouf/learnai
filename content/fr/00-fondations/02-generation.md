---
id: "00-02"
titre: "Génération"
sous_rubriques:
  - id: suivant
    titre: "Prédire le token suivant"
  - id: temperature
    titre: "Température et top-p"
  - id: determinisme
    titre: "Le déterminisme"
  - id: structure
    titre: "Sorties structurées"
sources:
  - titre: "Holtzman et al. — The Curious Case of Neural Text Degeneration (top-p), 2019"
    url: "https://arxiv.org/abs/1904.09751"
  - titre: "Horace He, Thinking Machines Lab — Defeating Nondeterminism in LLM Inference, septembre 2025 (résumé par Simon Willison)"
    url: "https://simonwillison.net/2025/Sep/11/defeating-nondeterminism"
qcm:
  - id: suivant-1
    sous_rubrique: suivant
    question: "À chaque étape, que calcule le modèle ?"
    choix:
      - "La phrase entière d'un coup"
      - "Le mot le plus juste d'après ses sources"
      - "Une probabilité pour chaque token du vocabulaire"
      - "Un résumé de la question"
    reponse: 2
    pourquoi: >-
      Le modèle sort une liste de probabilités.
      Une par token possible, souvent plus de 100 000.
      On en choisit un. On l'ajoute au texte.
      Puis on recommence pour le suivant.

  - id: suivant-2
    sous_rubrique: suivant
    question: "Pourquoi un LLM peut-il affirmer une chose fausse avec assurance ?"
    choix:
      - "Il ment volontairement"
      - "Il produit le texte probable, pas le texte vrai"
      - "Sa température est trop basse"
      - "Il manque de tokens"
    reponse: 1
    pourquoi: >-
      Le modèle a appris à quoi ressemble un texte plausible.
      Il n'a pas de mécanisme pour vérifier un fait.
      Une fausse référence peut être très probable.
      Le ton assuré vient du style appris, pas d'une vérification.
      C'est le sujet de l'étape 3.

  - id: temperature-1
    sous_rubrique: temperature
    question: "Une température proche de 0 donne…"
    choix:
      - "Des réponses plus créatives"
      - "Des réponses plus courtes"
      - "Des réponses plus prévisibles"
      - "Des réponses plus rapides"
    reponse: 2
    pourquoi: >-
      La température étire ou écrase les probabilités.
      Basse, elle renforce le token le plus probable.
      Il est choisi presque à chaque fois.
      Haute, elle donne plus de chances aux tokens rares.

  - id: temperature-2
    sous_rubrique: temperature
    question: "Top-p à 0,9 signifie…"
    choix:
      - "On tire parmi les tokens les plus probables, jusqu'à 90 % de probabilité cumulée"
      - "On garde les 90 % de tokens les plus fréquents du vocabulaire"
      - "La réponse est juste à 90 %"
      - "On coupe la réponse à 90 % de max_tokens"
    reponse: 0
    pourquoi: >-
      On trie les tokens du plus probable au moins probable.
      On les additionne jusqu'à atteindre 0,9.
      On tire au sort seulement dans ce groupe.
      Les tokens absurdes de la longue traîne sont exclus.

  - id: temperature-3
    sous_rubrique: temperature
    question: "Tu extrais le montant et la date de factures. Quel réglage ?"
    choix:
      - "Aucune importance"
      - "Température haute"
      - "Top-p à 1 et température haute"
      - "Température basse"
    reponse: 3
    pourquoi: >-
      Une extraction a une seule bonne réponse.
      La variété n'apporte que des erreurs.
      On réserve la température haute aux tâches créatives.

  - id: temperature-4
    sous_rubrique: temperature
    question: "Faut-il régler la température et le top-p en même temps ?"
    choix:
      - "Oui, toujours les deux"
      - "En général, on n'en règle qu'un"
      - "Non, top-p est obsolète"
      - "Oui, sinon l'API refuse"
    reponse: 1
    pourquoi: >-
      Les deux agissent sur le même tirage.
      Les combiner rend l'effet difficile à prévoir.
      Les fournisseurs conseillent d'en modifier un seul.

  - id: determinisme-1
    sous_rubrique: determinisme
    question: "Température 0, même prompt, deux appels. Les réponses sont-elles identiques ?"
    choix:
      - "Non, pas garanti"
      - "Oui, c'est garanti"
      - "Oui, si le prompt est court"
      - "Non, le modèle change chaque jour"
    reponse: 0
    pourquoi: >-
      Le serveur regroupe ta requête avec d'autres, en lot.
      La taille du lot dépend de la charge du moment.
      Elle change l'ordre de certains calculs.
      Un minuscule écart peut faire basculer un token.
      Tout ce qui suit change alors.

  - id: determinisme-2
    sous_rubrique: determinisme
    question: "Conséquence pratique pour tes tests ?"
    choix:
      - "Comparer le texte exact de la réponse"
      - "Désactiver les tests sur les LLM"
      - "Vérifier des propriétés de la réponse, pas le texte exact"
      - "Toujours fixer la température à 2"
    reponse: 2
    pourquoi: >-
      Un test sur le texte exact cassera sans raison.
      On vérifie plutôt le fond : un champ présent, une valeur, une citation.
      C'est le sujet de l'étape 7.

  - id: structure-1
    sous_rubrique: structure
    question: "Tu écris « Réponds en JSON » dans le prompt. Le JSON est-il garanti ?"
    choix:
      - "Oui"
      - "Non, il faut une sortie structurée contrainte par un schéma"
      - "Oui, si la température est à 0"
      - "Non, les LLM ne savent pas écrire du JSON"
    reponse: 1
    pourquoi: >-
      Une consigne est une demande, pas une contrainte.
      Le modèle peut ajouter une phrase, oublier une accolade.
      Avec une sortie contrainte, l'API n'autorise que les tokens valides.
      Le schéma est alors respecté par construction.

  - id: structure-2
    sous_rubrique: structure
    question: "Un JSON valide selon le schéma est-il forcément juste ?"
    choix:
      - "Oui, le schéma l'a vérifié"
      - "Non, le schéma est ignoré"
      - "Oui, si tous les champs sont remplis"
      - "Non, la forme est bonne, le contenu peut être faux"
    reponse: 3
    pourquoi: >-
      Le schéma vérifie les types et les champs.
      Il ne vérifie pas que la date est la bonne.
      Un montant inventé reste un nombre valide.
      Il faut aussi valider les valeurs.

  - id: structure-3
    sous_rubrique: structure
    question: "La réponse s'arrête au milieu d'un JSON. Cause la plus probable ?"
    choix:
      - "Le modèle a planté"
      - "Le JSON était trop complexe"
      - "La température était trop basse"
      - "La limite max_tokens a été atteinte"
    reponse: 3
    pourquoi: >-
      max_tokens coupe la sortie net, même au milieu d'un mot.
      L'API indique la raison de l'arrêt.
      Vérifie-la avant de lire la réponse.
      Si c'est max_tokens, la réponse est incomplète.
---

## Prédire le token suivant {#suivant}

Un LLM fait une seule chose : prédire le token suivant.

À chaque étape, il donne une probabilité à chaque token du vocabulaire.

On en choisit un. On l'ajoute. On recommence.

:::pourquoi
C'est ainsi qu'il a été entraîné : deviner le mot suivant sur des milliards de textes.
Tout le reste en découle : écrire, résumer, coder.
:::

Le modèle produit le texte **probable**, pas le texte **vrai**.

:::pourquoi
Rien dans ce mécanisme ne vérifie un fait.
Une fausse citation bien formée peut être très probable.
D'où les hallucinations, sur un ton parfaitement assuré.
:::

## Température et top-p {#temperature}

Ces deux réglages décident **comment** on choisit parmi les probabilités.

**Température** : basse, le plus probable gagne presque toujours. Haute, les tokens rares ont leur chance.

:::pourquoi
La température divise les scores avant le calcul des probabilités.
Petite, elle creuse les écarts. Grande, elle les aplatit.
:::

**Top-p** : on garde les tokens les plus probables jusqu'à un total donné, par exemple 90 %. On tire au sort parmi eux.

:::pourquoi
Il coupe la longue traîne des tokens absurdes.
Le groupe gardé s'adapte : petit quand le modèle est sûr, large quand il hésite.
:::

Règle simple : tâche à réponse unique → température basse. Tâche créative → plus haute. Ne règle qu'un des deux.

## Le déterminisme {#determinisme}

Température 0 ne garantit pas la même réponse deux fois.

:::pourquoi
Ta requête est calculée en lot avec celles d'autres clients.
La taille du lot change avec la charge du serveur.
Elle modifie l'ordre de certains calculs, et donc des arrondis.
Un token peut basculer. Toute la suite diverge.
Thinking Machines l'a montré en 2025, et a proposé des calculs qui ne dépendent plus du lot.
:::

En pratique : teste des propriétés, pas un texte exact.

## Sorties structurées {#structure}

Demander du JSON dans le prompt ne le garantit pas.

Une **sortie structurée** donne un schéma à l'API. Elle n'autorise que les tokens qui respectent ce schéma.

:::pourquoi
À chaque étape, les tokens qui casseraient le schéma sont interdits.
Le modèle ne peut plus sortir du cadre.
:::

Le schéma garantit la forme. Pas le contenu.

:::pourquoi
Un montant inventé reste un nombre valide.
Valide toujours les valeurs, pas seulement la structure.
:::

Vérifie toujours la raison d'arrêt. `max_tokens` coupe la réponse net.

:::pourquoi
Une réponse coupée ressemble à une réponse normale.
Seule la raison d'arrêt te dit qu'il manque la fin.
:::

## À retenir

- Un LLM prédit le token suivant. Probable n'est pas vrai.
- Température basse pour les tâches à réponse unique.
- Température 0 n'est pas déterministe en pratique.
- Le schéma garantit la forme, jamais le fond.
