---
id: "07-04"
titre: "In CI"
sous_rubriques:
  - id: regressions
    titre: "Regressions"
  - id: bloquer-pr
    titre: "Blocking a PR"
sources:
  - titre: "Anthropic — A statistical approach to model evaluations, November 19, 2024"
    url: "https://www.anthropic.com/research/statistical-approach-to-model-evals"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, May 2, 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
qcm:
  - id: regressions-1
    sous_rubrique: regressions
    question: "What is a regression for an agent?"
    choix:
      - "A new case added to the test set"
      - "A slower agent"
      - "A syntax error"
      - "A case that passed before a change and fails after it"
    reponse: 3
    pourquoi: >-
      You change a prompt to fix one case.
      Three other cases start failing.
      That is the main risk of every change.
      CI must see it before the merge.

  - id: regressions-2
    sous_rubrique: regressions
    question: "Which changes should rerun the evaluation?"
    choix:
      - "Only the Python code"
      - "Prompt, model or its version, tools, schemas, agent code, search settings"
      - "Only the model"
      - "Only the documentation"
    reponse: 1
    pourquoi: >-
      Behavior depends on all of that.
      A changed tool description changes the calls.
      A new model changes everything.
      The prompt is code: it is tested like code.

  - id: regressions-3
    sous_rubrique: regressions
    question: "The overall score goes from 82% to 81%. What do you look at?"
    choix:
      - "The error bar, and the list of cases that changed in each direction"
      - "Nothing, it's about the same"
      - "Only the number"
      - "The color of the badge"
    reponse: 0
    pourquoi: >-
      One point may be noise.
      But it may hide 10 cases gained and 11 lost.
      Among the lost ones, maybe a critical case.
      The case-by-case list says what really moved.

  - id: regressions-4
    sous_rubrique: regressions
    question: "The model's outputs vary from one run to the next. How do you make CI less noisy?"
    choix:
      - "Delete the cases that vary"
      - "Rerun until it passes"
      - "Several samples per case, an average per case, and a paired comparison between versions"
      - "Ignore the noise"
    reponse: 2
    pourquoi: >-
      Anthropic recommends resampling each question.
      The per-case average reduces variance.
      Comparing the same cases between versions removes the shared difficulty.
      Rerunning until green hides the real problems.

  - id: regressions-5
    sous_rubrique: regressions
    question: "Should you run the whole evaluation set on every commit?"
    choix:
      - "Yes, always everything, whatever the cost"
      - "A fast subset on every commit, the full set before merging or going to production"
      - "No, never"
      - "Only once a year"
    reponse: 1
    pourquoi: >-
      A full set with an LLM judge costs time and tokens.
      A small fast set gives immediate feedback.
      The full set protects the moments that matter.
      The cost of evaluation is part of the design.

  - id: bloquer-pr-1
    sous_rubrique: bloquer-pr
    question: "What kind of rule should block a PR?"
    choix:
      - "The model's view of its own PR"
      - "Any drop of a tenth of a point"
      - "Nothing should block"
      - "A critical case failing, or a significant drop on a criterion beyond the error bar"
    reponse: 3
    pourquoi: >-
      Some cases are non-negotiable: security, money, personal data.
      For the rest, a drop within the noise must not block.
      Otherwise developers learn to ignore CI.
      The rule must be fair to be respected.

  - id: bloquer-pr-2
    sous_rubrique: bloquer-pr
    question: "OpenAI launched GPT-4o despite expert testers' negative impression. Which process change does it announce?"
    choix:
      - "No more A/B tests"
      - "Getting rid of testers"
      - "Treating behavior issues as launch-blocking, like other safety risks"
      - "Launching faster"
    reponse: 2
    pourquoi: >-
      The signal existed, but it blocked nothing.
      A test that can't block is just an opinion.
      OpenAI makes these issues blocking.
      The lesson holds for CI: define what stops a release.

  - id: bloquer-pr-3
    sous_rubrique: bloquer-pr
    question: "A PR improves 15 cases and breaks 2 critical cases. Do you merge?"
    choix:
      - "No: a broken critical case blocks, whatever the gain elsewhere"
      - "Yes, the balance is positive"
      - "Yes, if the overall score goes up"
      - "It depends on the mood"
    reponse: 0
    pourquoi: >-
      An average trades a risk for comfort.
      A refund without verification isn't offset by nicer answers.
      Critical cases are safeguards, not points.

  - id: bloquer-pr-4
    sous_rubrique: bloquer-pr
    question: "What should the evaluation report on the PR show?"
    choix:
      - "“OK” or “KO” with no detail"
      - "A single percentage"
      - "Scores per criterion with error bars, cases gained and lost, and a link to each trajectory"
      - "The full raw logs"
    reponse: 2
    pourquoi: >-
      The reviewer must understand in a minute.
      Per criterion: you see what moves.
      Lost cases: you know what to look at.
      Trajectories: you see why.

  - id: bloquer-pr-5
    sous_rubrique: bloquer-pr
    question: "Who can override a block from the evaluation CI?"
    choix:
      - "A designated person, with a written, recorded justification"
      - "Anyone, without a trace"
      - "The model"
      - "Nobody, ever, even for a proven false positive"
    reponse: 0
    pourquoi: >-
      A judge can be wrong, a case can be badly written.
      You need an emergency exit.
      But it must be rare, justified and visible.
      Otherwise the block no longer means anything.
---

## Regressions {#regressions}

Every change can break what used to work. That is a **regression**.

What should rerun the evaluation:

- the prompt;
- the model or its version;
- the tools and their descriptions;
- the schemas;
- the agent's code;
- the search settings.

:::pourquoi
Behavior depends on all of that.
The prompt is code. It is tested like code.
:::

Look at the **cases that change**, not only the score.

| Case | Before | After |
|---|---|---|
| refund-017 | pass | fail |
| refund-022 | fail | pass |
| sec-004 (critical) | pass | fail |

:::pourquoi
82% then 81% may hide 10 cases gained and 11 lost.
Including, maybe, a critical case.
:::

Cut the **noise**: several samples per case, an average per case, a paired comparison.

:::pourquoi
Outputs vary from one run to the next.
Anthropic recommends resampling, showing an error bar, and comparing the same cases between versions.
:::

Two levels: a **small fast set** on every commit, the **full set** before merging or going to production.

:::pourquoi
The full set with a judge costs time and tokens.
The small set gives immediate feedback.
:::

## Blocking a PR {#bloquer-pr}

An evaluation that can't **block** anything is just an opinion.

:::pourquoi
At OpenAI, testers had sensed a problem.
Their view didn't block the release.
OpenAI now treats this kind of issue as blocking.
:::

```mermaid
sequenceDiagram
  participant D as Developer
  participant G as Repository (PR)
  participant C as Evaluation CI
  participant J as LLM judge
  participant R as Reviewer
  D->>G: Opens a PR (new prompt)
  G->>C: Runs the full set
  C->>C: Checks by code (final state, DAG, format)
  C->>J: Criteria code can't check
  J-->>C: Verdicts and justifications
  C->>C: Compares with the main branch, case by case
  C-->>G: Blocked: sec-004 (critical) regressed
  G-->>R: Report: criteria, lost cases, trajectories
```

Two blocking rules:

1. A **critical case** fails: security, money, personal data.
2. A criterion **drops beyond** its error bar.

:::pourquoi
A critical case isn't offset by 15 improved cases.
A drop within the noise must not block, or people learn to ignore CI.
:::

The **report** on the PR: scores per criterion with error bars, cases gained and lost, a link to each trajectory.

:::pourquoi
The reviewer must understand in a minute what moved, and why.
:::

An **emergency exit**: a designated person can override, with a written, recorded justification.

:::pourquoi
A case may be badly written. A judge may be wrong.
But an invisible bypass kills the rule.
:::

## Key points

- Any change of prompt, model, tool or schema reruns the evaluation.
- Look at the cases that change, with an error bar.
- A small fast set on every commit, the full set before merging.
- Block on a critical case or a significant drop. A readable report. Overrides recorded.
