---
id: "07-02"
titre: "Reference"
sous_rubriques:
  - id: golden-set
    titre: "Golden set"
  - id: dag
    titre: "Expected DAG"
  - id: note-etape
    titre: "Per-step score"
sources:
  - titre: "Anthropic — Adding Error Bars to Evals: A Statistical Approach to Language Model Evaluations, 2024"
    url: "https://arxiv.org/abs/2411.00640"
  - titre: "Yao et al. — τ-bench, 2024"
    url: "https://arxiv.org/abs/2406.12045"
  - titre: "Shankar et al. — Who Validates the Validators?, 2024"
    url: "https://arxiv.org/abs/2404.12272"
qcm:
  - id: golden-set-1
    sous_rubrique: golden-set
    question: "What is a golden set?"
    choix:
      - "The model's best answers"
      - "A fixed set of test cases, each with its expected result, checked by a human"
      - "The training data"
      - "A set of system prompts"
    reponse: 1
    pourquoi: >-
      Each case: an input, and what is expected.
      A human has checked the expected result.
      You replay these cases at every change.
      It is the ruler that lets you compare.

  - id: golden-set-2
    sous_rubrique: golden-set
    question: "Where do the best cases in a golden set come from?"
    choix:
      - "Generated at random"
      - "Only from examples the developer made up"
      - "From a public benchmark, as is"
      - "From real requests and real failures seen, plus edge cases written on purpose"
    reponse: 3
    pourquoi: >-
      Real requests reflect real use.
      Each bug fixed becomes a case, so it doesn't come back.
      Edge cases test what rarely happens but costs a lot.
      A public benchmark doesn't know your business.

  - id: golden-set-3
    sous_rubrique: golden-set
    question: "A golden set of 20 cases. The new prompt goes from 15 to 16 successes. Is it an improvement?"
    choix:
      - "Yes, clearly"
      - "You can't conclude that: the gap is within the noise for so few cases"
      - "Yes, by 5%"
      - "No, it is necessarily worse"
    reponse: 1
    pourquoi: >-
      With 20 cases, a single case flipping is 5 points.
      Generation randomness is enough to explain it.
      Anthropic recommends showing an error bar with every score.
      More cases, or several samples per case, cut that noise.

  - id: golden-set-4
    sous_rubrique: golden-set
    question: "Why compare two versions on exactly the same cases?"
    choix:
      - "Per-case (paired) differences are far less noisy than two separate scores"
      - "Only to save time"
      - "Because the law requires it"
      - "It isn't useful"
    reponse: 0
    pourquoi: >-
      Some cases are hard for every version.
      Comparing case by case removes that shared difficulty.
      Anthropic recommends these paired tests.
      You also see exactly which cases changed.

  - id: golden-set-5
    sous_rubrique: golden-set
    question: "Do you need a test set separate from the one used to tune the prompt?"
    choix:
      - "No, it's a waste"
      - "No, the same one is enough"
      - "Yes: otherwise you optimize for those exact cases and the score becomes misleading"
      - "Only for big models"
    reponse: 2
    pourquoi: >-
      Tune a prompt on 50 cases long enough and it learns them.
      The score goes up, the real behavior doesn't.
      A held-out set tells the truth.
      You don't look at it while tuning.

  - id: dag-1
    sous_rubrique: dag
    question: "What is an expected DAG for evaluating an agent?"
    choix:
      - "The exact list of calls, in a single order"
      - "The graph of required steps and their dependencies, without imposing a total order"
      - "The system prompt"
      - "A database schema"
    reponse: 1
    pourquoi: >-
      DAG: directed acyclic graph.
      It says “verify identity before refunding”.
      It doesn't say in which order to read two independent documents.
      Several valid trajectories pass the test.

  - id: dag-2
    sous_rubrique: dag
    question: "Why a DAG rather than an ordered list of expected calls?"
    choix:
      - "No difference"
      - "A DAG is shorter to write"
      - "Lists are forbidden in YAML"
      - "A rigid list fails correct trajectories that change the order of independent steps"
    reponse: 3
    pourquoi: >-
      The agent can read the invoice then the contract, or the other way round.
      Both orders are fine.
      An ordered list rejects one of them.
      The DAG only imposes the real dependencies.

  - id: dag-3
    sous_rubrique: dag
    question: "What do you check when comparing the real trajectory with the expected DAG?"
    choix:
      - "That the arguments are written with the same spaces"
      - "That the trajectory has exactly the same length"
      - "The mandatory steps are present, their dependencies respected, and no forbidden action appears"
      - "Only the last step"
    reponse: 2
    pourquoi: >-
      Presence: nothing essential is skipped.
      Order: each step comes after what it depends on.
      Forbidden: no deletion, no unplanned external sending.
      Extra steps are allowed, but counted.

  - id: dag-4
    sous_rubrique: dag
    question: "How do you compare the arguments of an expected call with those of the real call?"
    choix:
      - "On the fields that matter, with one rule per field: equality, set, or pattern"
      - "Exact comparison of the whole JSON text"
      - "Never compare arguments"
      - "Ask the model whether they look alike"
    reponse: 0
    pourquoi: >-
      The order id must be exact.
      A search query can be phrased ten ways.
      The order of JSON keys doesn't matter at all.
      One rule per field avoids false failures.

  - id: note-etape-1
    sous_rubrique: note-etape
    question: "Why score each step and not only the final result?"
    choix:
      - "Because the final result doesn't count"
      - "To have more numbers in the table"
      - "To know where the agent fails: tool choice, arguments, reading the result"
      - "To slow the evaluation down"
    reponse: 2
    pourquoi: >-
      “Failed” doesn't say what to fix.
      “Right tool, wrong argument at step 3” does.
      You know whether to rework the schema, the prompt or the tool.

  - id: note-etape-2
    sous_rubrique: note-etape
    question: "Which scale do you use to score a step?"
    choix:
      - "A simple, defined scale: pass / fail, or a few precisely described levels"
      - "A score from 0 to 100 with no definition"
      - "An emoji"
      - "Free text"
    reponse: 0
    pourquoi: >-
      Between 73 and 76, nobody knows the difference.
      Two graders won't give the same number.
      Pass or fail, with a definition, is reproducible.
      You can then count and compare.

  - id: note-etape-3
    sous_rubrique: note-etape
    question: "The agent fails step 2, then corrects itself at step 3 and succeeds. How do you score it?"
    choix:
      - "Remove this case from the test set"
      - "Total failure"
      - "Total success, forget step 2"
      - "Step 2 failed, recovery passed, result passed: each fact is scored separately"
    reponse: 3
    pourquoi: >-
      Correcting itself is a strength.
      But the first mistake costs time and tokens.
      Scoring separately keeps both pieces of information.
      An average would flatten them.

  - id: note-etape-4
    sous_rubrique: note-etape
    question: "According to Shankar et al. (2024), what happens when you define evaluation criteria?"
    choix:
      - "They are perfect at the first attempt"
      - "The criteria change as you grade real outputs: that is criteria drift"
      - "They become useless"
      - "The model picks them on its own"
    reponse: 1
    pourquoi: >-
      You need criteria to grade.
      But grading outputs makes you discover new criteria.
      Some only appear once you see the answers.
      The rubric is revised after reading real results.
---

## Golden set {#golden-set}

A **golden set**: fixed test cases, each with its expected result, checked by a human.

```yaml
- id: refund-017
  input: "I want a refund for order 512, the item arrived broken."
  expected:
    final_state: { order_512: refunded, amount: 49.90 }
    mandatory_steps: [verify_identity, read_order, refund]
    forbidden: [send_external_email]
  origin: "real ticket #8812, anonymized"
```

:::pourquoi
Without a fixed reference, you can't compare two versions.
You compare impressions.
:::

Where the cases come from:

- **real requests**, anonymized;
- every **bug** fixed;
- **edge cases** written on purpose: no answer, ambiguous, malicious.

:::pourquoi
A bug turned into a test can't come back unseen.
Edge cases are costly when they fail.
:::

Show an **error bar**. Compare versions **case by case**.

:::pourquoi
With 20 cases, one case flipping is 5 points.
Anthropic recommends giving the standard error with every score.
And analyzing paired differences: same case, two versions.
:::

Keep a **held-out** set, never used to tune the prompt.

:::pourquoi
Tune on the same cases long enough and the prompt learns them.
The score goes up. The real behavior doesn't.
:::

## Expected DAG {#dag}

For an agent, what is expected is often a **path**. But not a single path.

A **DAG** (directed acyclic graph) describes the required steps and their dependencies.

```mermaid
flowchart LR
  I["verify_identity"] --> R["refund"]
  C["read_order"] --> R
  P["read_return_policy"] --> R
  R --> N["notify_customer"]
```

`read_order` and `read_return_policy` can come in any order. `refund` comes after all three.

:::pourquoi
An ordered list rejects correct trajectories.
The DAG only imposes the real dependencies.
:::

You check:

| Check | Fails if |
|---|---|
| Presence | A mandatory step is missing |
| Dependencies | A step comes before what it depends on |
| Forbidden | A forbidden action appears |
| Arguments | A key field differs: id, amount |
| Extras | Allowed, but counted in the cost |

:::pourquoi
Each check catches one family of errors.
Together, they judge the path without freezing it.
:::

Compare arguments **field by field**, with one rule per field.

:::pourquoi
The id must be exact.
A search query can be phrased ten ways.
:::

## Per-step score {#note-etape}

You score **each step**, not only the end.

| Step | Criterion | Score |
|---|---|---|
| 1. verify_identity | Right tool, right arguments | pass |
| 2. read_order | Right id | fail (id 215 instead of 512) |
| 3. read_order | Correction after the error | pass |
| 4. refund | Right amount, after verification | pass |

:::pourquoi
“Failed” doesn't say what to fix.
“Wrong argument at step 2” does.
:::

A **simple, defined** scale: pass or fail, or a few described levels.

:::pourquoi
A score from 0 to 100 with no definition isn't reproducible.
Two graders will give two numbers.
:::

The criteria **change**. Shankar and his coauthors (2024) call it criteria drift.

:::pourquoi
Grading real outputs makes you discover new criteria.
You revise the rubric after reading results, not only before.
:::

## Key points

- Golden set: fixed cases, checked expectations, drawn from real use, bugs and edge cases.
- Error bars, case-by-case comparison, a held-out set.
- Expected DAG: mandatory steps, dependencies, forbidden actions. No needless order.
- Per-step score, a simple scale, a rubric revised by reading the results.
