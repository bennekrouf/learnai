---
id: "07-03"
titre: "LLM as a judge"
sous_rubriques:
  - id: grille
    titre: "Rubric"
  - id: calibrage
    titre: "Calibration"
  - id: biais
    titre: "Bias"
sources:
  - titre: "Zheng et al. — Judging LLM-as-a-Judge with MT-Bench and Chatbot Arena, 2023"
    url: "https://arxiv.org/abs/2306.05685"
  - titre: "Shankar et al. — Who Validates the Validators?, 2024"
    url: "https://arxiv.org/abs/2404.12272"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, May 2, 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
qcm:
  - id: grille-1
    sous_rubrique: grille
    question: "When should you use an LLM as a judge?"
    choix:
      - "Always, even when code is enough"
      - "To compare two numbers"
      - "To validate a JSON"
      - "When the criterion can't be checked by code: grounding, tone, relevance, completeness of a text"
    reponse: 3
    pourquoi: >-
      “Is the answer polite?” can't be tested with an if.
      An LLM judge can read and assess.
      But it costs, it varies, it has biases.
      When code is enough, code wins.

  - id: grille-2
    sous_rubrique: grille
    question: "What should the rubric given to the judge contain?"
    choix:
      - "“Score this answer from 1 to 10”"
      - "One criterion at a time, a precise definition, examples of pass and fail, a structured output"
      - "The agent's system prompt only"
      - "Nothing, the judge knows"
    reponse: 1
    pourquoi: >-
      “Score from 1 to 10” lets the judge make up its own criteria.
      A precise criterion gives reproducible scores.
      The examples show where the line is.
      The structured output can be read by code.

  - id: grille-3
    sous_rubrique: grille
    question: "Why ask the judge for a binary verdict (pass / fail) rather than a score from 1 to 10?"
    choix:
      - "A well-defined binary decision is more stable and easier to check against a human"
      - "It costs fewer tokens, that's all"
      - "Judges can't count to 10"
      - "No reason"
    reponse: 0
    pourquoi: >-
      Between 6 and 7, the judge hesitates, and changes its mind from one call to the next.
      Between “cites a source that doesn't say it” and not, it is clear-cut.
      It is easy to compare with a human's view.
      Several binary criteria beat one vague score.

  - id: grille-4
    sous_rubrique: grille
    question: "Why ask the judge for a justification before its verdict?"
    choix:
      - "To bill more"
      - "To lengthen the answer"
      - "It reasons before deciding, and you can read why it scored that way"
      - "It is never useful"
    reponse: 2
    pourquoi: >-
      The justification forces the judge to look at the evidence.
      It lets you spot a judge applying the wrong criterion.
      When you disagree with it, you know why.

  - id: calibrage-1
    sous_rubrique: calibrage
    question: "How do you know whether your LLM judge is reliable?"
    choix:
      - "Ask it whether it is reliable"
      - "Compare its scores with humans' on a labeled sample"
      - "Use the biggest model and check nothing"
      - "See whether it answers fast"
    reponse: 1
    pourquoi: >-
      The judge is a model. It can be wrong.
      Humans label 100 to 200 cases.
      You measure the agreement between judge and humans.
      Without that, you don't know what the score is worth.

  - id: calibrage-2
    sous_rubrique: calibrage
    question: "Your judge agrees with humans 90% of the time, but 90% of the answers are good. What do you check?"
    choix:
      - "Its temperature"
      - "Nothing, 90% is excellent"
      - "Its speed"
      - "Its ability to spot failures: a judge that always says “pass” would also get 90%"
    reponse: 3
    pourquoi: >-
      With 90% good answers, always saying “yes” gives 90% agreement.
      That judge detects no failure.
      You measure separately: failures found, and passes recognized.
      Or a metric that corrects for chance, such as Cohen's kappa.

  - id: calibrage-3
    sous_rubrique: calibrage
    question: "According to Zheng et al. (2023), what agreement does a strong judge such as GPT-4 reach with human preferences?"
    choix:
      - "100%"
      - "About 20%"
      - "Over 80%, about the same as the agreement between humans"
      - "Less than chance"
    reponse: 2
    pourquoi: >-
      That is what made LLM judges popular.
      But it was on their task, with their protocol.
      On your task, you must measure again.
      A figure from a paper doesn't calibrate your judge.

  - id: calibrage-4
    sous_rubrique: calibrage
    question: "You change the judge's model or prompt. What do you do?"
    choix:
      - "Recalibrate it on the sample labeled by humans"
      - "Nothing, it's the same judge"
      - "Rerun the agent"
      - "Delete the old scores"
    reponse: 0
    pourquoi: >-
      Another judge is another ruler.
      The old scores are no longer comparable.
      You measure agreement with humans again.
      And you pin the judge's version in the evaluation.

  - id: biais-1
    sous_rubrique: biais
    question: "What is the position bias of a judge comparing two answers A and B?"
    choix:
      - "It only reads the end"
      - "It prefers short answers"
      - "It favors an answer depending on whether it comes first or second"
      - "It prefers answers in English"
    reponse: 2
    pourquoi: >-
      Zheng et al. measured it in 2023.
      Swapping A and B can change the verdict.
      The fix: judge in both orders.
      If the verdicts differ, it is a tie.

  - id: biais-2
    sous_rubrique: biais
    question: "The judge prefers longer answers, even with no more information. Which bias?"
    choix:
      - "Verbosity bias"
      - "Position bias"
      - "Recency bias"
      - "Confirmation bias"
    reponse: 0
    pourquoi: >-
      Longer looks more complete.
      The judge is impressed.
      An agent optimized against this judge learns to pad.
      The fix: an explicit “concise” criterion, and a length check.

  - id: biais-3
    sous_rubrique: biais
    question: "The judge and the agent are the same model. What is the risk?"
    choix:
      - "The judge refuses to score"
      - "No risk"
      - "The judge crashes"
      - "Self-enhancement bias: the judge favors answers that look like its own"
    reponse: 3
    pourquoi: >-
      Zheng et al. list it among judges' biases.
      A model finds natural what it would have written.
      The fix: a judge from another family, and human calibration.

  - id: biais-4
    sous_rubrique: biais
    question: "The April 2025 GPT-4o update used users' thumbs as a signal. What is the link with judges?"
    choix:
      - "No link"
      - "Every judge, human or model, has preferences that a system optimized against it will exploit"
      - "Thumbs are always reliable"
      - "LLM judges don't have this problem"
    reponse: 1
    pourquoi: >-
      Users like being told they are right.
      Optimizing their approval reinforced sycophancy.
      An LLM judge that likes long answers pushes toward padding.
      Optimizing a score also optimizes its flaws.
---

## Rubric {#grille}

Some criteria can't be checked by code: grounding, tone, relevance. You then ask an **LLM judge**.

:::pourquoi
“Is the answer polite?” can't be tested with an `if`.
But a judge costs, varies and has biases.
Code comes first when it is enough.
:::

A good rubric:

```text
Criterion: GROUNDING
Definition: every factual claim in the answer is supported
by at least one of the documents provided.
Pass if: every claim is supported.
Fail if: at least one claim is supported by no document.
Fail example: the answer says “delivery in 24 hours”, no document says so.
Output: {"justification": "...", "verdict": "pass" | "fail"}
```

:::pourquoi
One criterion at a time: the judge doesn't mix them up.
A definition and examples: it knows where the line is.
A structured output: code reads it.
:::

Prefer **binary** verdicts, one per criterion.

:::pourquoi
Between 6 and 7 out of 10, the judge hesitates and changes its mind.
Pass or fail is reproducible, and can be compared with a human.
:::

Ask for the **justification before** the verdict.

:::pourquoi
The judge looks at the evidence before deciding.
And you can read why it decided.
:::

## Calibration {#calibrage}

The judge is a model. It must be **checked**.

```mermaid
sequenceDiagram
  participant H as Humans
  participant E as Sample (150 cases)
  participant J as LLM judge
  participant M as Agreement measure
  H->>E: Label each case
  J->>E: Scores each case
  E->>M: Human verdicts and judge verdicts
  M-->>J: Failures found, passes recognized, kappa
  Note over J,M: Agreement too low: fix the rubric, then measure again
```

:::pourquoi
Without calibration, you don't know what the score is worth.
You may be measuring the judge, not the agent.
:::

Raw agreement can mislead.

| Measure | Question |
|---|---|
| Failures found | Of the real failures, how many does the judge see? |
| Passes recognized | Of the real passes, how many does it accept? |
| Cohen's kappa | Agreement, once chance agreement is removed |

:::pourquoi
If 90% of the answers are good, a judge that always says “pass” gets 90% agreement.
And finds no failure.
:::

Zheng and his coauthors (2023) measure over 80% agreement between GPT-4 and humans, the same as between humans.

:::pourquoi
That is encouraging, but it was their task.
On yours, you must measure again.
:::

Every change of the judge's model or rubric calls for a **new calibration**.

:::pourquoi
Another judge is another ruler.
The old scores can no longer be compared.
:::

## Bias {#biais}

LLM judges have known biases.

| Bias | Effect | Fix |
|---|---|---|
| Position | Prefers A or B depending on order | Judge in both orders |
| Verbosity | Prefers the longer one | “Concise” criterion, length check |
| Self-enhancement | Prefers its own style | Judge from another model family |
| Limited reasoning | Misses a calculation or logic error | Check by code when possible |

:::pourquoi
Zheng and his coauthors list these four limits.
Each fix removes one known source of error.
:::

Optimizing against a judge also optimizes its **flaws**.

:::pourquoi
GPT-4o was also trained on users' thumbs.
Users like being told they are right.
The model became sycophantic.
A judge that likes long answers teaches the agent to pad.
:::

## Key points

- The LLM judge is for what code can't judge.
- Rubric: one criterion, a definition, examples, a binary verdict, the justification first.
- Calibrate against humans. Measure the failures found, not only agreement.
- Know the biases: position, verbosity, self-enhancement. Optimizing a judge optimizes its flaws.
