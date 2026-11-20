---
id: "08-02"
titre: "Routing"
sous_rubriques:
  - id: selon-complexite
    titre: "By complexity"
  - id: repli
    titre: "Fallback chains"
sources:
  - titre: "Ong et al. — RouteLLM: Learning to Route LLMs with Preference Data, 2024"
    url: "https://arxiv.org/abs/2406.18665"
  - titre: "LMSYS — RouteLLM: An Open-Source Framework for Cost-Effective LLM Routing, July 2024"
    url: "https://lmsys.org/blog/2024-07-01-routellm/"
  - titre: "Chen, Zaharia, Zou — FrugalGPT, 2023"
    url: "https://arxiv.org/abs/2305.05176"
qcm:
  - id: selon-complexite-1
    sous_rubrique: selon-complexite
    question: "What is model routing?"
    choix:
      - "Sending each request to every model"
      - "Always using the biggest model"
      - "Spreading requests at random"
      - "Sending each request to the cheapest model able to handle it well"
    reponse: 3
    pourquoi: >-
      Many requests are simple.
      A small model handles them well, for far less.
      The big model is kept for what needs it.
      You pay for difficulty, not volume.

  - id: selon-complexite-2
    sous_rubrique: selon-complexite
    question: "Which request can go to a small model?"
    choix:
      - "Planning a database migration in ten steps"
      - "Sorting an email into one of five categories"
      - "Analyzing an 80-page contract"
      - "Fixing a bug across several files"
    reponse: 1
    pourquoi: >-
      Classify, extract, rephrase: short, well-framed tasks.
      Plan, reason at length, code across several files: hard tasks.
      Complexity depends on the task, not the length of the question.

  - id: selon-complexite-3
    sous_rubrique: selon-complexite
    question: "What does RouteLLM (2024) show on MT Bench?"
    choix:
      - "A router keeps 95% of GPT-4's performance while sending only 26% of requests to GPT-4"
      - "Small models always beat GPT-4"
      - "Routing always halves quality"
      - "Routing saves nothing"
    reponse: 0
    pourquoi: >-
      The router learns from human preferences.
      It predicts when the small model is enough.
      Three quarters of the requests go to the cheap model.
      Quality stays close to the big model.

  - id: selon-complexite-4
    sous_rubrique: selon-complexite
    question: "What is a cascade, as in FrugalGPT?"
    choix:
      - "Asking the big model to choose the small one"
      - "Calling every model at the same time"
      - "Trying a small model first, checking the answer, and moving up to a bigger one if it isn't good enough"
      - "Cutting the answer into pieces"
    reponse: 2
    pourquoi: >-
      The router decides before. The cascade decides after.
      A checker scores the small model's answer.
      If it is weak, you pay for the big one.
      FrugalGPT reports up to 98% savings at GPT-4 level on its tests.

  - id: selon-complexite-5
    sous_rubrique: selon-complexite
    question: "What is the hidden cost of a cascade?"
    choix:
      - "No hidden cost"
      - "When you move up a level, you pay for the small model on top of the big one, and wait longer"
      - "It turns off caching"
      - "It forces a change of provider"
    reponse: 1
    pourquoi: >-
      A hard request goes through two models.
      Cost and latency add up.
      If most requests move up, the cascade costs more than the big model alone.
      You measure the escalation rate.

  - id: selon-complexite-6
    sous_rubrique: selon-complexite
    question: "How do you know whether your routing is good?"
    choix:
      - "Ask the small model whether it answered well"
      - "Look only at the bill"
      - "Trust the router"
      - "Evaluate each branch on the test set, and track quality and cost per branch"
    reponse: 3
    pourquoi: >-
      A saving that breaks answers isn't a saving.
      The test set from step 7 says whether the small model holds up.
      Per branch: you see where it breaks.
      The router itself gets tested.

  - id: repli-1
    sous_rubrique: repli
    question: "What is a fallback chain for?"
    choix:
      - "Sending the same request to every model"
      - "Lowering quality on purpose"
      - "Switching to another model or provider when the main one fails or doesn't answer"
      - "Replacing tests"
    reponse: 2
    pourquoi: >-
      Providers have outages and rate limits.
      Without a fallback, your outage follows theirs.
      With a fallback, the service carries on, maybe a little less well.

  - id: repli-2
    sous_rubrique: repli
    question: "The main model returns a 429 error (too many requests). What do you do?"
    choix:
      - "Retry with growing waits, then switch to the fallback if it persists"
      - "Retry in a loop without waiting"
      - "Give up and show a raw error"
      - "Send the request to every model in the chain"
    reponse: 0
    pourquoi: >-
      A rate limit is often transient.
      Retrying without waiting makes the saturation worse.
      After a few tries, the fallback takes over.
      It is the backoff from step 5.

  - id: repli-3
    sous_rubrique: repli
    question: "The main model refuses a request for safety reasons. Should you move to the fallback model?"
    choix:
      - "Yes, every refusal should be worked around"
      - "Yes, another model might accept"
      - "No: a refusal isn't an outage. Replaying it elsewhere gets around the rule"
      - "Yes, but without logging anything"
    reponse: 2
    pourquoi: >-
      The fallback is for technical outages.
      A refusal is a decision.
      Working around it with another model means hunting for the most permissive one.
      You handle the refusal, you don't recycle it.

  - id: repli-4
    sous_rubrique: repli
    question: "Your fallback model has never been evaluated. Risk?"
    choix:
      - "On outage day, it answers, but maybe badly: broken format, tools called wrongly"
      - "None, one model is as good as another"
      - "It will necessarily be better"
      - "It will refuse to start"
    reponse: 0
    pourquoi: >-
      Each model reacts differently to the same prompt.
      The fallback is only used in a crisis: that is when you discover its flaws.
      It goes through the same test set as the main one.
      Sometimes with an adapted prompt.

  - id: repli-5
    sous_rubrique: repli
    question: "What should the last link of a fallback chain do?"
    choix:
      - "Nothing, the chain never stops"
      - "Restart the whole chain from the beginning, forever"
      - "Make up an answer"
      - "Fail cleanly: a clear message, the task queued or handed to a human"
    reponse: 3
    pourquoi: >-
      Every model can be unavailable at the same time.
      An endless chain becomes a costly loop.
      A clean failure keeps trust.
      The task can resume later.

  - id: repli-6
    sous_rubrique: repli
    question: "Why log which model answered each request?"
    choix:
      - "It isn't useful"
      - "To tie quality, cost and incidents to the right model"
      - "For decoration"
      - "To slow the agent down"
    reponse: 1
    pourquoi: >-
      A drop in quality on a Tuesday? Maybe the fallback ran all day.
      Without that record, you look for the bug in the wrong place.
      The model used is part of the trajectory.
---

## By complexity {#selon-complexite}

Not all requests are equal. **Routing** sends each one to the cheapest model that handles it well.

:::pourquoi
Sorting an email doesn't need the biggest model.
Paying for the big model on everything means paying for difficulty on every easy request.
:::

| Task | Model |
|---|---|
| Classify, extract, rephrase | Small |
| Answer from provided documents | Medium |
| Plan, reason at length, code across several files | Big |

Two ways to route:

```mermaid
flowchart LR
  Q[Request] --> R{Router}
  R -->|simple| P[Small model]
  R -->|hard| G[Big model]
  Q2[Request] --> P2[Small model]
  P2 --> V{Checker}
  V -->|good answer| OK[Answer]
  V -->|weak answer| G2[Big model]
  G2 --> OK
```

- The **router** decides **before**, from the request.
- The **cascade** decides **after**, from the small model's answer.

:::pourquoi
RouteLLM (2024) learns a router from human preferences.
On MT Bench, it keeps 95% of GPT-4's performance with 26% of requests sent to GPT-4.
FrugalGPT (2023) describes the cascade and reports up to 98% savings at GPT-4 level on its tests.
:::

A cascade has a **hidden cost**.

:::pourquoi
A request that moves up pays for two models and waits twice.
If many move up, the cascade costs more than the big model alone.
You track the escalation rate.
:::

Routing is **evaluated** branch by branch.

:::pourquoi
A saving that breaks answers isn't a saving.
The test set from step 7 says whether the small model holds its branch.
:::

## Fallback chains {#repli}

A provider goes down. A rate limit is hit. The model doesn't answer.

The **fallback chain** takes over.

```mermaid
sequenceDiagram
  participant A as Application
  participant P as Main model
  participant S as Fallback model
  participant H as Human
  participant J as Log
  A->>P: Request
  P-->>A: 429 error
  A->>P: New try after a wait
  P-->>A: 429 error
  A->>S: Same request (adapted prompt)
  S-->>A: Answer
  A->>J: Answer served by the fallback model
  Note over A,H: If the fallback fails too
  A->>H: Task queued or handed over
```

:::pourquoi
Without a fallback, your outage follows the provider's.
With a fallback, the service carries on, maybe a little less well.
:::

The rules:

1. **Retry** first, with growing waits.
2. **Switch** if the error persists.
3. **Fail cleanly** at the end of the chain.

:::pourquoi
A transient error often clears with waiting.
An endless chain becomes a costly loop.
A clean failure beats a made-up answer.
:::

The fallback is for **outages**, not **refusals**.

:::pourquoi
A safety refusal is a decision.
Replaying it elsewhere means hunting for the most permissive model.
:::

The fallback model goes through the **same test set**.

:::pourquoi
The same prompt doesn't work the same on every model.
The fallback is only used in a crisis: a bad time to discover its flaws.
:::

You **log** which model answered.

:::pourquoi
A drop in quality may come from a fallback left active all day.
Without a record, you look in the wrong place.
:::

## Key points

- Route: a small model for the simple, a big one for the hard.
- Router before, cascade after. Watch the escalation rate.
- Fallback: retry, switch, fail cleanly.
- Never a fallback to get around a refusal. The fallback is tested and logged.
