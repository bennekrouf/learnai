---
id: "03-05"
titre: "Detecting hallucinations"
sous_rubriques:
  - id: coherence
    titre: "Consistency"
  - id: juge
    titre: "Judge"
sources:
  - titre: "Manakul, Liusie, Gales — SelfCheckGPT: Zero-Resource Black-Box Hallucination Detection, 2023"
    url: "https://arxiv.org/abs/2303.08896"
  - titre: "Farquhar, Kossen, Kuhn, Gal — Detecting hallucinations in large language models using semantic entropy, Nature, June 2024"
    url: "https://pmc.ncbi.nlm.nih.gov/articles/PMC11186750"
  - titre: "Min et al. — FActScore: Fine-grained Atomic Evaluation of Factual Precision, 2023"
    url: "https://arxiv.org/abs/2305.14251"
qcm:
  - id: coherence-1
    sous_rubrique: coherence
    question: "The principle of SelfCheckGPT (2023)?"
    choix:
      - "Asking the model whether it hallucinated"
      - "Measuring the answer's length"
      - "Comparing the answer with Wikipedia"
      - "Generating several answers: a fact that varies from one answer to the next is suspect"
    reponse: 3
    pourquoi: >-
      A fact the model knows comes back on every sample.
      A made-up fact changes from one sample to the next.
      Draw several answers and compare.
      No external source is needed.

  - id: coherence-2
    sous_rubrique: coherence
    question: "Three samples answer “Paris”, “the French capital” and “Paris, in France”. Are they consistent?"
    choix:
      - "No, the words differ"
      - "Yes, they have the same meaning"
      - "No, you need 10"
      - "Impossible to say"
    reponse: 1
    pourquoi: >-
      The words change, the meaning doesn't.
      That is the idea behind semantic entropy (Nature, 2024).
      Answers are grouped by meaning, not by text.
      If they all fall in the same group, the model is stable.

  - id: coherence-3
    sous_rubrique: coherence
    question: "High semantic entropy means…"
    choix:
      - "The sampled answers have very different meanings: a risk of making things up"
      - "The model is very sure"
      - "The answer is long"
      - "The question is in English"
    reponse: 0
    pourquoi: >-
      Entropy measures spread.
      Many different meanings: the model is improvising.
      The authors call these arbitrary inventions confabulations.
      You can then abstain or go find a source.

  - id: coherence-4
    sous_rubrique: coherence
    question: "Which hallucination does consistency fail to detect?"
    choix:
      - "A made-up fact that changes on every sample"
      - "A made-up date"
      - "An error the model repeats every time, because it learned it"
      - "A made-up name"
    reponse: 2
    pourquoi: >-
      If the model learned a false fact, it repeats it consistently.
      All samples agree, and all are wrong.
      Consistency measures improvisation, not truth.
      Hence the value of also checking against sources.

  - id: coherence-5
    sous_rubrique: coherence
    question: "The main cost of consistency methods?"
    choix:
      - "Several generations per question: cost and latency multiplied"
      - "None"
      - "A large index"
      - "An external database"
    reponse: 0
    pourquoi: >-
      Five samples cost about five times one answer.
      They are kept for cases where an error is costly.
      Or run afterwards, on a sample.

  - id: juge-1
    sous_rubrique: juge
    question: "FActScore (2023) evaluates a text by…"
    choix:
      - "An overall score from 1 to 10"
      - "Measuring how much it looks like the question"
      - "Counting rare words"
      - "Splitting it into atomic facts, checked one by one against a source"
    reponse: 3
    pourquoi: >-
      A biography holds dozens of facts.
      An overall score hides isolated errors.
      An atomic fact can be checked: supported or not.
      The score is the share of supported facts.

  - id: juge-2
    sous_rubrique: juge
    question: "An LLM judge checks the claims. What must it receive?"
    choix:
      - "Only the answer"
      - "Each claim and the matching source passages"
      - "Only the question"
      - "The first model's system prompt"
    reponse: 1
    pourquoi: >-
      Without the sources, the judge checks from memory.
      It may then hallucinate in turn.
      With the passages, it compares text against text.
      That is a far more reliable task.

  - id: juge-3
    sous_rubrique: juge
    question: "Can you trust the judge without checking it?"
    choix:
      - "Yes, it is an LLM"
      - "Yes, if it is larger than the first model"
      - "No: you must measure its agreement with human verdicts"
      - "Yes, at temperature 0"
    reponse: 2
    pourquoi: >-
      The judge is a model. It makes mistakes too.
      Take a hundred cases labeled by humans.
      Measure how often the judge agrees.
      That is calibrating the judge, in step 7.

  - id: juge-4
    sous_rubrique: juge
    question: "Checking every answer adds latency. Which strategy?"
    choix:
      - "Never check"
      - "Check once a month"
      - "Check before showing for risky topics, and afterwards on a sample for the rest"
      - "Let the user check"
    reponse: 2
    pourquoi: >-
      Legal or medical advice is worth waiting for the check.
      An ordinary question can show right away.
      The sample checked afterwards measures quality over time.
      It also feeds the test set.
---

## Consistency {#coherence}

A simple idea: ask the same question several times.

A known fact comes back. A made-up fact changes.

:::pourquoi
When the model knows, the most likely answer dominates.
When it improvises, several answers are equally likely, and the sample varies.
:::

**SelfCheckGPT** (2023) compares several samples with one another. No external source needed.

**Semantic entropy** (Nature, 2024) groups the samples **by meaning**. Many different meanings: a risk of making things up.

```mermaid
flowchart LR
  Q["Question"] --> L["LLM, 3 samples"]
  L --> T1["Paris"]
  L --> T2["The French capital"]
  L --> T3["Lyon"]
  T1 --> G["Grouping by meaning"]
  T2 --> G
  T3 --> G
  G --> E["2 meanings: medium entropy"]
```

:::pourquoi
“Paris” and “the French capital” say the same thing.
Counting different texts would inflate the uncertainty.
Counting different meanings measures the real hesitation.
:::

Limit: a learned error is repeated consistently.

:::pourquoi
All samples agree, and all are wrong.
Consistency detects improvisation, not learned errors.
:::

Cost: several generations per question.

## Judge {#juge}

A second model checks the answer against the sources.

```mermaid
flowchart LR
  R["Answer"] --> D["Split into claims"]
  D --> J["LLM judge"]
  S["Source passages"] --> J
  J --> V["Verdicts: supported, contradicted, absent"]
  V --> X["Decision: show, fix, abstain"]
```

:::pourquoi
Comparing a claim with a passage is a simple task.
Much simpler than producing the answer.
A smaller model can often do it.
:::

**FActScore** (2023) formalizes the idea: atomic facts, checked one by one.

:::pourquoi
An overall score hides an isolated error.
A verdict per fact shows it.
:::

The judge must receive the sources.

:::pourquoi
Without them, it checks from memory.
It may hallucinate in turn.
:::

The judge makes mistakes too. Measure its agreement with humans.

:::pourquoi
An uncalibrated judge gives a false sense of security.
That is the topic of step 7.
:::

Where do you place the check?

| Case | Strategy |
|---|---|
| Risky topic: health, law, money | Check before showing |
| Everyday question | Show, check a sample afterwards |

## Key points

- Consistency: a fact that varies between samples is suspect.
- Compare meanings, not words.
- Consistency misses learned errors. The judge with sources catches them.
- The judge is calibrated against humans.
