---
id: "09-03"
titre: "Benchmarks"
sous_rubriques:
  - id: reproductibilite
    titre: "Reproducibility"
sources:
  - titre: "TechCrunch — Meta's vanilla Maverick AI model ranks below rivals on a popular chat benchmark, April 11, 2025"
    url: "https://techcrunch.com/2025/04/11/metas-vanilla-maverick-ai-model-ranks-below-rivals-on-a-popular-chat-benchmark"
  - titre: "Singh et al. — The Leaderboard Illusion, 2025"
    url: "https://arxiv.org/abs/2504.20879"
  - titre: "OpenAI — Introducing SWE-bench Verified, August 13, 2024"
    url: "https://openai.com/index/introducing-swe-bench-verified/"
  - titre: "Anthropic — A statistical approach to model evaluations, November 19, 2024"
    url: "https://www.anthropic.com/research/statistical-approach-to-model-evals"
qcm:
  - id: reproductibilite-1
    sous_rubrique: reproductibilite
    question: "What is a reproducible benchmark?"
    choix:
      - "A result published on a blog"
      - "A result someone else can get again with the same code, the same data and the same settings"
      - "A result that goes up with each try"
      - "A result approved by the model being tested"
    reponse: 1
    pourquoi: >-
      A number nobody can redo is a claim.
      A number anyone can redo is a measurement.
      The difference lies in what you publish with it.

  - id: reproductibilite-2
    sous_rubrique: reproductibilite
    question: "In April 2025, the Llama 4 Maverick variant ranked 2nd on LMArena wasn't the one released. Where did the public version rank?"
    choix:
      - "It was never tested"
      - "1st"
      - "2nd as well"
      - "32nd"
    reponse: 3
    pourquoi: >-
      Meta had tested an “experimental” version, tuned for conversation.
      It was 2nd.
      The downloadable version, tested afterwards, came 32nd.
      The number described a model nobody could use.

  - id: reproductibilite-3
    sous_rubrique: reproductibilite
    question: "Why give the exact model version (with its date) and not only its name?"
    choix:
      - "To look more serious"
      - "A name can point to different versions over time, with different results"
      - "It isn't useful"
      - "The name already holds the score"
    reponse: 1
    pourquoi: >-
      An alias such as “latest model” changes without warning.
      The dated version doesn't move.
      Llama 4 showed it: same family name, two very different rankings.
      Without the exact version, nobody can redo the measurement.

  - id: reproductibilite-4
    sous_rubrique: reproductibilite
    question: "What does the study “The Leaderboard Illusion” (2025) criticize in the Chatbot Arena rankings?"
    choix:
      - "Some providers test many variants privately and only publish the best one"
      - "The ranking is sorted alphabetically"
      - "The votes are cast by models"
      - "The tests are too short"
    reponse: 0
    pourquoi: >-
      The study counts 27 private variants tested by Meta before Llama 4.
      Keeping the best of 27 is drawing 27 times and keeping the best draw.
      The score goes up without the model being better on average.
      You must say how many tries came before the number.

  - id: reproductibilite-5
    sous_rubrique: reproductibilite
    question: "Why did OpenAI create SWE-bench Verified in 2024?"
    choix:
      - "To replace every other benchmark"
      - "To make the test easier for its models"
      - "Part of the original SWE-bench tasks were badly specified, or had overly strict tests"
      - "Because SWE-bench was paid"
    reponse: 2
    pourquoi: >-
      93 developers reviewed 1,699 tasks.
      Each task was annotated three times.
      500 tasks judged clear and solvable form the subset.
      A flawed benchmark measures badly, even when reproducible.

  - id: reproductibilite-6
    sous_rubrique: reproductibilite
    question: "Which settings do you publish with a result?"
    choix:
      - "Only the score"
      - "Model version, prompts, temperature, max tokens, dataset version, evaluation code"
      - "Only the model's name"
      - "The author's name"
    reponse: 1
    pourquoi: >-
      Each setting changes the result.
      A different prompt, and the score moves by several points.
      The evaluation code fixes how scoring is done.
      Without them, you compare different things.

  - id: reproductibilite-7
    sous_rubrique: reproductibilite
    question: "Your agent scores 74% on a single run. What is missing?"
    choix:
      - "A second model"
      - "Nothing, 74% is enough"
      - "A colored chart"
      - "Several runs and an error bar"
    reponse: 3
    pourquoi: >-
      Outputs vary from one run to the next.
      74% could be 70 or 78 on the next run.
      Anthropic recommends the error bar, as seen in step 7.
      Without it, a 2-point gap means nothing.

  - id: reproductibilite-8
    sous_rubrique: reproductibilite
    question: "What is benchmark contamination?"
    choix:
      - "A syntax error in the dataset"
      - "A virus in the test code"
      - "The test's questions or answers end up in the model's training data"
      - "A test that is too long"
    reponse: 2
    pourquoi: >-
      A model that has seen the answers recites them.
      The score measures memory, not ability.
      A private or recent test set limits this risk.
      Your internal golden set has that advantage.

  - id: reproductibilite-9
    sous_rubrique: reproductibilite
    question: "You compare your agent with a competitor's. Which condition makes the comparison honest?"
    choix:
      - "The same test set, the same evaluation code, the same number of runs"
      - "Taking your best score and the competitor's worst"
      - "Using your own test set and theirs"
      - "Comparing press releases"
    reponse: 0
    pourquoi: >-
      Two numbers measured differently can't be compared.
      One evaluation code can be more generous than another.
      Same conditions, or no comparison.

  - id: reproductibilite-10
    sous_rubrique: reproductibilite
    question: "What do you publish on top of the score for an agent benchmark?"
    choix:
      - "A screenshot of the score"
      - "Nothing more"
      - "Cost and latency per task, and the raw outputs of each case"
      - "The list of people on the team"
    reponse: 2
    pourquoi: >-
      An agent at 90% that costs 10 times more isn't better everywhere.
      Cost and latency complete quality.
      The raw outputs let others check the scoring.
      And see where the agent fails.
---

## Reproducibility {#reproductibilite}

A number nobody can redo is a **claim**. A number anyone can redo is a **measurement**.

:::pourquoi
In April 2025, Meta announces Llama 4 Maverick.
An “experimental” variant, tuned for conversation, is 2nd on LMArena.
The public version, tested afterwards, comes 32nd.
The number described a model nobody could download.
:::

What skews a number:

| Trap | Example | Fix |
|---|---|---|
| Vague version | One variant tested, another released | Exact, dated version |
| Best draw | 27 private variants, a single one published | Say how many tries |
| Flawed test | Badly specified tasks, overly strict tests | Review the test set |
| Single run | 74%, with no error bar | Several runs, an error bar |
| Contamination | The model has seen the answers | A private or recent set |
| Different conditions | Two evaluation codes | Same conditions for everyone |

:::pourquoi
The study “The Leaderboard Illusion” (2025) counts 27 private variants tested by Meta before Llama 4.
Keeping the best of 27 is keeping the best draw.
OpenAI created SWE-bench Verified in 2024: 93 developers reviewed 1,699 tasks to keep 500 clear, solvable ones.
:::

The pipeline of a reproducible benchmark:

```mermaid
sequenceDiagram
  participant S as Evaluation script
  participant D as Dataset (version 1.3)
  participant M as Model (dated version)
  participant N as Scoring
  participant R as Published results
  S->>D: Loads the 200 cases
  loop 5 runs
    S->>M: Fixed prompt, fixed temperature
    M-->>S: Raw outputs
    S->>N: Scores each case
  end
  N-->>S: Average score and error bar
  S->>R: Score, error bar, cost, latency, raw outputs, settings
```

:::pourquoi
Each element is fixed and versioned.
The five runs give the error bar.
Everything that comes out is published: someone else can rerun the pipeline.
:::

The card to publish with each result:

```yaml
model: claude-sonnet-x-2026-05-01   # dated version, not an alias
dataset: support-golden v1.3 (200 cases)
prompts: prompts/agent-v12.md
temperature: 0
max_tokens: 4096
runs: 5
eval_code: eval/run.py @ commit 3f2a91c
score: 74% ± 3%
cost_per_task: $0.18
latency_p95: 11 s
raw_outputs: results/2026-11-19/
date: 2026-11-19
```

:::pourquoi
Each line changes the result if it changes.
A different prompt can move the score by several points.
Cost and latency avoid praising an agent that is too expensive.
:::

A comparison is only honest under the **same conditions**.

:::pourquoi
One evaluation code can be more generous than another.
Two numbers measured differently can't be compared.
:::

## Key points

- Exact model version, prompts, settings, dataset, code: everything is published.
- Several runs, an error bar, the number of tries before the number.
- Cost, latency and raw outputs alongside the score.
- Same conditions to compare. Watch out for contamination.
