---
id: "03-03"
titre: "Ancrage"
sous_rubriques:
  - id: citer
    titre: "Citer ses sources"
  - id: support
    titre: "Vérifier le support"
sources:
  - titre: "Mata v. Avianca, Inc., S.D.N.Y. 2023 — ordonnance de sanctions (Justia)"
    url: "https://law.justia.com/cases/federal/district-courts/new-york/nysdce/1:2022cv01461/575368/54"
  - titre: "Magesh et al. (Stanford) — Hallucination-Free? Assessing the Reliability of Leading AI Legal Research Tools, 2024"
    url: "https://arxiv.org/abs/2405.20362"
  - titre: "Stanford HAI — AI on Trial: Legal Models Hallucinate in 1 out of 6 (or More) Benchmarking Queries"
    url: "https://hai.stanford.edu/news/ai-trial-legal-models-hallucinate-1-out-6-queries"
  - titre: "Anthropic — Reduce hallucinations (documentation)"
    url: "https://platform.claude.com/docs/en/test-and-evaluate/strengthen-guardrails/reduce-hallucinations"
  - titre: "Es et al. — RAGAS: Automated Evaluation of Retrieval Augmented Generation, 2023"
    url: "https://arxiv.org/abs/2309.15217"
qcm:
  - id: citer-1
    sous_rubrique: citer
    question: "2023, Mata v. Avianca. Un avocat doute des jurisprudences fournies par ChatGPT. Que fait-il ?"
    choix:
      - "Il les cherche dans une base juridique"
      - "Il appelle le greffe"
      - "Il demande à ChatGPT si elles sont réelles"
      - "Il les retire"
    reponse: 2
    pourquoi: >-
      Il demande à ChatGPT si l'affaire Varghese existe.
      ChatGPT répond oui, et qu'on la trouve dans les grandes bases juridiques.
      Elle n'existait pas. Les autres non plus.
      Demander au modèle de se vérifier n'est pas une vérification.

  - id: citer-2
    sous_rubrique: citer
    question: "Comment rendre une citation vérifiable par ton application ?"
    choix:
      - "Donner un identifiant à chaque morceau fourni, et exiger que la réponse cite ces identifiants"
      - "Demander au modèle d'inventer une référence précise"
      - "Demander une bibliographie à la fin"
      - "Faire confiance au modèle"
    reponse: 0
    pourquoi: >-
      Tu fournis les morceaux [1], [2], [3].
      Le modèle doit citer [2] après chaque affirmation.
      Ton code vérifie que [2] fait bien partie des morceaux fournis.
      Une référence hors de cette liste est rejetée d'office.

  - id: citer-3
    sous_rubrique: citer
    question: "Le modèle cite un passage entre guillemets. Comment le vérifier sans LLM ?"
    choix:
      - "Impossible"
      - "Vérifier la langue"
      - "Compter les mots"
      - "Vérifier que le texte cité apparaît mot pour mot dans le morceau source"
    reponse: 3
    pourquoi: >-
      Une citation exacte se vérifie par simple recherche de texte.
      Si elle n'apparaît pas dans la source, elle est inventée ou déformée.
      C'est gratuit, rapide et sans ambiguïté.
      D'où l'intérêt de demander des citations mot pour mot.

  - id: citer-4
    sous_rubrique: citer
    question: "Pour un long document, Anthropic conseille de demander d'abord…"
    choix:
      - "Un résumé"
      - "Des citations mot pour mot pertinentes, puis l'analyse à partir d'elles"
      - "Une traduction"
      - "Une note de confiance"
    reponse: 1
    pourquoi: >-
      Extraire d'abord les passages force le modèle à lire.
      L'analyse s'appuie ensuite sur ces passages précis.
      S'il ne trouve aucun passage, il doit le dire.
      La réponse reste attachée au texte réel.

  - id: support-1
    sous_rubrique: support
    question: "Le chatbot d'Air Canada cite la bonne page. Sa réponse est-elle ancrée ?"
    choix:
      - "Oui, puisqu'il cite la page"
      - "Impossible à dire"
      - "Oui, si la page est officielle"
      - "Non : la page dit le contraire de sa réponse"
    reponse: 3
    pourquoi: >-
      Une citation dit d'où vient une affirmation.
      Elle ne prouve pas que la source la soutient.
      Il faut vérifier, affirmation par affirmation, que la source dit bien ça.

  - id: support-2
    sous_rubrique: support
    question: "Stanford, 2024 : des outils juridiques avec RAG, vendus « sans hallucination ». Résultat ?"
    choix:
      - "Aucune erreur"
      - "Des réponses fausses ou mal sourcées dans plus d'une requête sur six"
      - "Plus d'erreurs que sans RAG"
      - "L'étude n'a pas pu les tester"
    reponse: 1
    pourquoi: >-
      Les chercheurs ont testé les outils de LexisNexis et Thomson Reuters.
      Ils hallucinent moins qu'un GPT-4 sans recherche.
      Mais ils se trompent encore dans plus d'une requête sur six.
      Le RAG réduit les hallucinations. Il ne les supprime pas.

  - id: support-3
    sous_rubrique: support
    question: "Comment vérifier qu'une réponse est soutenue par ses sources ?"
    choix:
      - "Découper la réponse en affirmations, et juger chacune contre son passage source"
      - "Compter les citations"
      - "Mesurer la longueur de la réponse"
      - "Demander à l'utilisateur"
    reponse: 0
    pourquoi: >-
      Une réponse mélange affirmations vraies et fausses.
      Un jugement global les noie.
      Chaque affirmation reçoit un verdict : soutenue, contredite, absente.
      On retire ou on signale celles qui ne sont pas soutenues.

  - id: support-4
    sous_rubrique: support
    question: "La « fidélité » (faithfulness) d'une réponse, dans RAGAS, mesure…"
    choix:
      - "Si la réponse est vraie dans le monde réel"
      - "Sa longueur"
      - "La part de ses affirmations soutenues par le contexte fourni"
      - "Sa politesse"
    reponse: 2
    pourquoi: >-
      La fidélité ne juge pas la vérité absolue.
      Elle juge le lien entre la réponse et les documents fournis.
      3 affirmations soutenues sur 4 : fidélité de 0,75.
      Une réponse vraie mais sortie de nulle part n'est pas fidèle.
---

## Citer ses sources {#citer}

Une réponse sans source ne se vérifie pas.

Printemps 2023. Dans l'affaire *Mata v. Avianca*, un avocat new-yorkais dépose un mémoire. Il cite des jurisprudences trouvées avec ChatGPT.

Aucune n'existe.

Il avait pourtant demandé à ChatGPT si elles étaient réelles. ChatGPT avait dit oui.

:::pourquoi
Le modèle produit du texte probable.
« Oui, cette affaire existe » est une réponse probable.
Le juge sanctionne les avocats. La faute était de ne pas avoir vérifié.
:::

La citation doit être vérifiable **par ton code**, pas par le modèle.

```mermaid
sequenceDiagram
  participant A as Application
  participant R as Recherche
  participant L as LLM
  participant V as Vérificateur
  A->>R: Question
  R-->>A: Morceaux [1] [2] [3]
  A->>L: Question + morceaux numérotés
  L-->>A: Réponse avec [2] et une citation mot pour mot
  A->>V: Réponse + morceaux
  V-->>A: [2] existe ? Citation présente dans [2] ?
```

:::pourquoi
Les identifiants viennent de ton application. Le modèle ne peut pas en inventer de valides.
Une citation mot pour mot se vérifie par simple recherche de texte.
:::

Pour un long document, demande d'abord les passages exacts, puis l'analyse.

:::pourquoi
C'est une recommandation d'Anthropic.
Extraire d'abord force le modèle à s'appuyer sur le texte réel.
S'il ne trouve rien, il le dit au lieu d'inventer.
:::

## Vérifier le support {#support}

Citer ne suffit pas. Air Canada citait la bonne page. La page disait le contraire.

:::pourquoi
La citation dit d'où vient l'affirmation.
Elle ne dit pas si la source la soutient.
:::

En 2024, Stanford teste des outils juridiques à base de RAG. Certains sont vendus « sans hallucination ». Ils se trompent dans plus d'une requête sur six.

:::pourquoi
Le RAG réduit les hallucinations, il ne les supprime pas.
Le modèle peut mal lire, mal combiner, ou compléter de mémoire.
Il faut contrôler la sortie, pas seulement l'entrée.
:::

La méthode : découper la réponse en affirmations, puis juger chacune.

Exemple fictif, une boutique en ligne :

| Affirmation | Source | Verdict |
|---|---|---|
| « Retour gratuit sous 60 jours » | CGV : « retour gratuit sous 30 jours » | Contredite |
| « Le remboursement se fait sur le moyen de paiement d'origine » | CGV, article 7 | Soutenue |
| « Les soldes sont exclus des retours » | Aucune | Absente |

:::pourquoi
Une réponse globale mélange vrai et faux.
Un verdict par affirmation montre exactement ce qui pose problème.
On retire ou on signale ce qui n'est pas soutenu.
:::

La **fidélité** est la part d'affirmations soutenues par le contexte.

:::pourquoi
Elle ne juge pas la vérité du monde. Elle juge le lien avec les sources.
Une réponse vraie mais sans source n'est pas fidèle.
:::

## À retenir

- Le modèle ne vérifie pas ses propres citations.
- Identifiants fournis par l'application, citations mot pour mot vérifiées par le code.
- Citer n'est pas soutenir. On juge chaque affirmation.
- Le RAG réduit les hallucinations. Il ne les supprime pas.
