---
id: "09-03"
titre: "Benchmarks"
sous_rubriques:
  - id: reproductibilite
    titre: "Reproductibilité"
sources:
  - titre: "TechCrunch — Meta's vanilla Maverick AI model ranks below rivals on a popular chat benchmark, 11 avril 2025"
    url: "https://techcrunch.com/2025/04/11/metas-vanilla-maverick-ai-model-ranks-below-rivals-on-a-popular-chat-benchmark"
  - titre: "Singh et al. — The Leaderboard Illusion, 2025"
    url: "https://arxiv.org/abs/2504.20879"
  - titre: "OpenAI — Introducing SWE-bench Verified, 13 août 2024"
    url: "https://openai.com/index/introducing-swe-bench-verified/"
  - titre: "Anthropic — A statistical approach to model evaluations, 19 novembre 2024"
    url: "https://www.anthropic.com/research/statistical-approach-to-model-evals"
qcm:
  - id: reproductibilite-1
    sous_rubrique: reproductibilite
    question: "Qu'est-ce qu'un benchmark reproductible ?"
    choix:
      - "Un résultat publié sur un blog"
      - "Un résultat qu'une autre personne peut retrouver avec le même code, les mêmes données et les mêmes réglages"
      - "Un résultat qui monte à chaque essai"
      - "Un résultat validé par le modèle testé"
    reponse: 1
    pourquoi: >-
      Un chiffre qu'on ne peut pas refaire est une affirmation.
      Un chiffre qu'on peut refaire est une mesure.
      La différence tient à ce qu'on publie avec lui.

  - id: reproductibilite-2
    sous_rubrique: reproductibilite
    question: "En avril 2025, la variante de Llama 4 Maverick classée 2ᵉ sur LMArena n'était pas celle publiée. Où s'est classée la version publique ?"
    choix:
      - "Elle n'a jamais été testée"
      - "1ʳᵉ"
      - "2ᵉ aussi"
      - "32ᵉ"
    reponse: 3
    pourquoi: >-
      Meta avait testé une version « expérimentale », réglée pour la conversation.
      Elle était 2ᵉ.
      La version téléchargeable, testée ensuite, est arrivée 32ᵉ.
      Le chiffre décrivait un modèle que personne ne pouvait utiliser.

  - id: reproductibilite-3
    sous_rubrique: reproductibilite
    question: "Pourquoi indiquer la version exacte du modèle (avec sa date) et pas seulement son nom ?"
    choix:
      - "Pour faire plus sérieux"
      - "Un nom peut désigner des versions différentes au fil du temps, avec des résultats différents"
      - "Ce n'est pas utile"
      - "Le nom contient déjà le score"
    reponse: 1
    pourquoi: >-
      Un alias comme « dernier modèle » change sans prévenir.
      La version datée, elle, ne bouge pas.
      Llama 4 l'a montré : même nom de famille, deux classements très différents.
      Sans version exacte, personne ne refait la mesure.

  - id: reproductibilite-4
    sous_rubrique: reproductibilite
    question: "Que reproche l'étude « The Leaderboard Illusion » (2025) aux classements de Chatbot Arena ?"
    choix:
      - "Certains fournisseurs testent beaucoup de variantes en privé et ne publient que la meilleure"
      - "Le classement est trié par ordre alphabétique"
      - "Les votes sont faits par des modèles"
      - "Les tests sont trop courts"
    reponse: 0
    pourquoi: >-
      L'étude compte 27 variantes privées testées par Meta avant Llama 4.
      Garder la meilleure de 27, c'est tirer 27 fois et garder le meilleur tirage.
      Le score monte sans que le modèle soit meilleur en moyenne.
      Il faut dire combien d'essais ont précédé le chiffre.

  - id: reproductibilite-5
    sous_rubrique: reproductibilite
    question: "Pourquoi OpenAI a-t-il créé SWE-bench Verified en 2024 ?"
    choix:
      - "Pour remplacer tous les autres benchmarks"
      - "Pour rendre le test plus facile pour ses modèles"
      - "Une partie des tâches du SWE-bench original étaient mal décrites, ou avaient des tests trop stricts"
      - "Parce que SWE-bench était payant"
    reponse: 2
    pourquoi: >-
      93 développeurs ont relu 1 699 tâches.
      Chaque tâche a été annotée trois fois.
      500 tâches jugées claires et solubles forment le sous-ensemble.
      Un benchmark faux mesure mal, même reproductible.

  - id: reproductibilite-6
    sous_rubrique: reproductibilite
    question: "Quels réglages publier avec un résultat ?"
    choix:
      - "Seulement le score"
      - "Version du modèle, prompts, température, nombre maximal de tokens, version du jeu de données, code de l'évaluation"
      - "Seulement le nom du modèle"
      - "Le nom de l'auteur"
    reponse: 1
    pourquoi: >-
      Chaque réglage change le résultat.
      Un prompt différent, et le score bouge de plusieurs points.
      Le code de l'évaluation fixe la façon de noter.
      Sans eux, on compare des choses différentes.

  - id: reproductibilite-7
    sous_rubrique: reproductibilite
    question: "Ton agent obtient 74 % sur un seul passage. Que manque-t-il ?"
    choix:
      - "Un deuxième modèle"
      - "Rien, 74 % suffit"
      - "Un graphique en couleurs"
      - "Plusieurs passages et une marge d'erreur"
    reponse: 3
    pourquoi: >-
      Les sorties varient d'un passage à l'autre.
      74 % peut être 70 ou 78 au passage suivant.
      Anthropic recommande la marge d'erreur, comme vu à l'étape 7.
      Sans elle, un écart de 2 points ne veut rien dire.

  - id: reproductibilite-8
    sous_rubrique: reproductibilite
    question: "Qu'est-ce que la contamination d'un benchmark ?"
    choix:
      - "Une erreur de syntaxe dans le jeu de données"
      - "Un virus dans le code du test"
      - "Les questions ou réponses du test se retrouvent dans les données d'entraînement du modèle"
      - "Un test trop long"
    reponse: 2
    pourquoi: >-
      Un modèle qui a vu les réponses récite.
      Le score mesure la mémoire, pas la capacité.
      Un jeu de test privé, ou récent, limite ce risque.
      Ton golden set interne a cet avantage.

  - id: reproductibilite-9
    sous_rubrique: reproductibilite
    question: "Tu compares ton agent à un concurrent. Quelle condition rend la comparaison honnête ?"
    choix:
      - "Le même jeu de test, le même code d'évaluation, le même nombre de passages"
      - "Prendre son meilleur score et le pire du concurrent"
      - "Utiliser son propre jeu de test et le sien"
      - "Comparer les communiqués de presse"
    reponse: 0
    pourquoi: >-
      Deux chiffres mesurés différemment ne se comparent pas.
      Un code d'évaluation peut être plus généreux qu'un autre.
      Mêmes conditions, ou pas de comparaison.

  - id: reproductibilite-10
    sous_rubrique: reproductibilite
    question: "Que publier en plus du score pour un benchmark d'agent ?"
    choix:
      - "Une capture d'écran du score"
      - "Rien de plus"
      - "Le coût et la latence par tâche, et les sorties brutes de chaque cas"
      - "La liste des personnes de l'équipe"
    reponse: 2
    pourquoi: >-
      Un agent à 90 % qui coûte 10 fois plus n'est pas meilleur partout.
      Le coût et la latence complètent la qualité.
      Les sorties brutes permettent de vérifier la notation.
      Et de voir où l'agent échoue.
---

## Reproductibilité {#reproductibilite}

Un chiffre qu'on ne peut pas refaire est une **affirmation**. Un chiffre qu'on peut refaire est une **mesure**.

:::pourquoi
En avril 2025, Meta annonce Llama 4 Maverick.
Une variante « expérimentale », réglée pour la conversation, est 2ᵉ sur LMArena.
La version publique, testée ensuite, arrive 32ᵉ.
Le chiffre décrivait un modèle que personne ne pouvait télécharger.
:::

Ce qui fausse un chiffre :

| Piège | Exemple | Parade |
|---|---|---|
| Version floue | Une variante testée, une autre publiée | Version exacte, datée |
| Meilleur tirage | 27 variantes privées, une seule publiée | Dire combien d'essais |
| Test mal fait | Tâches mal décrites, tests trop stricts | Relire le jeu de test |
| Passage unique | 74 %, sans marge d'erreur | Plusieurs passages, marge d'erreur |
| Contamination | Le modèle a vu les réponses | Jeu privé ou récent |
| Conditions différentes | Deux codes d'évaluation | Mêmes conditions pour tous |

:::pourquoi
L'étude « The Leaderboard Illusion » (2025) compte 27 variantes privées testées par Meta avant Llama 4.
Garder la meilleure de 27, c'est garder le meilleur tirage.
OpenAI a créé SWE-bench Verified en 2024 : 93 développeurs ont relu 1 699 tâches pour en garder 500 claires et solubles.
:::

Le circuit d'un benchmark reproductible :

```mermaid
sequenceDiagram
  participant S as Script d'évaluation
  participant D as Jeu de données (version 1.3)
  participant M as Modèle (version datée)
  participant N as Notation
  participant R as Résultats publiés
  S->>D: Charge les 200 cas
  loop 5 passages
    S->>M: Prompt fixe, température fixe
    M-->>S: Sorties brutes
    S->>N: Note chaque cas
  end
  N-->>S: Score moyen et marge d'erreur
  S->>R: Score, marge, coût, latence, sorties brutes, réglages
```

:::pourquoi
Chaque élément est fixé et versionné.
Les cinq passages donnent la marge d'erreur.
Tout ce qui sort est publié : quelqu'un d'autre peut refaire le circuit.
:::

La fiche à publier avec chaque résultat :

```yaml
modele: claude-sonnet-x-2026-05-01   # version datée, pas un alias
jeu_de_donnees: support-golden v1.3 (200 cas)
prompts: prompts/agent-v12.md
temperature: 0
max_tokens: 4096
passages: 5
code_evaluation: eval/run.py @ commit 3f2a91c
score: 74 % ± 3 %
cout_par_tache: 0,18 €
latence_p95: 11 s
sorties_brutes: resultats/2026-11-19/
date: 2026-11-19
```

:::pourquoi
Chaque ligne change le résultat si elle change.
Un prompt différent peut faire bouger le score de plusieurs points.
Le coût et la latence évitent de vanter un agent trop cher.
:::

Une comparaison n'est honnête que dans les **mêmes conditions**.

:::pourquoi
Un code d'évaluation peut être plus généreux qu'un autre.
Deux chiffres mesurés différemment ne se comparent pas.
:::

## À retenir

- Version exacte du modèle, prompts, réglages, jeu de données, code : tout est publié.
- Plusieurs passages, une marge d'erreur, le nombre d'essais avant le chiffre.
- Le coût, la latence et les sorties brutes avec le score.
- Mêmes conditions pour comparer. Attention à la contamination.
