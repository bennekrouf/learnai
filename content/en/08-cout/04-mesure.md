---
id: "08-04"
titre: "Measurement"
sous_rubriques:
  - id: cout-tache
    titre: "Cost per task"
  - id: latence
    titre: "p50 and p95 latency"
sources:
  - titre: "Dean and Barroso — The Tail at Scale, Communications of the ACM, 2013"
    url: "https://research.google/pubs/pub40801/"
  - titre: "Anthropic — Prompt caching (documentation)"
    url: "https://platform.claude.com/docs/en/build-with-claude/prompt-caching"
qcm:
  - id: cout-tache-1
    sous_rubrique: cout-tache
    question: "Why measure the cost per task rather than the monthly bill alone?"
    choix:
      - "The monthly bill is always wrong"
      - "The bill says how much, the cost per task says where and why"
      - "Cost per task is mandatory"
      - "There is no difference"
    reponse: 1
    pourquoi: >-
      The bill adds everything up.
      A task that costs 50 times the average drowns in it.
      Per task, you see which flow, which customer, which prompt costs.
      You know what to optimize.

  - id: cout-tache-2
    sous_rubrique: cout-tache
    question: "What do you add up for the cost of an agent task?"
    choix:
      - "Every model call (input, cache, output, reasoning), paid tools and retries"
      - "Only the last model call"
      - "Only the tokens of the final answer"
      - "Only the subscription price"
    reponse: 0
    pourquoi: >-
      An agent often makes ten or thirty calls for one task.
      Each call has its input and output tokens.
      Failed tries and paid tools count too.
      Forgetting a line means underestimating the cost.

  - id: cout-tache-3
    sous_rubrique: cout-tache
    question: "100 tasks cost $50 in total. 80 succeed. What is the cost per successful task?"
    choix:
      - "$0.40"
      - "$0.50"
      - "$0.625: failures are paid for too"
      - "Failures don't count"
    reponse: 2
    pourquoi: >-
      $50 divided by 80 successes = $0.625.
      The 20 failures cost money and brought nothing.
      A cheaper model that fails more can cost more per success.
      That is the figure that matters for comparing.

  - id: cout-tache-4
    sous_rubrique: cout-tache
    question: "At each step, the agent sends the whole history back to the model. How does the cost evolve?"
    choix:
      - "Each step costs the same"
      - "Each step costs more than the last: total cost grows faster than the number of steps"
      - "The cost drops at each step"
      - "Only the first step costs anything"
    reponse: 1
    pourquoi: >-
      Step 20 sends back the 19 previous steps.
      Input tokens pile up.
      Prompt caching cuts this cost, since the start of the history doesn't change.
      So does summarizing the history.

  - id: cout-tache-5
    sous_rubrique: cout-tache
    question: "How do you attribute each expense to the right task, customer and feature?"
    choix:
      - "Ask the model"
      - "Guess at the end of the month"
      - "Split the bill evenly"
      - "Tag each call with the task, customer and feature id"
    reponse: 3
    pourquoi: >-
      Without a tag, a cost has no owner.
      With a tag, you add up per task, customer, feature.
      It is also what feeds budgets and quotas.

  - id: cout-tache-6
    sous_rubrique: cout-tache
    question: "Output tokens cost several times more than input tokens at most providers. Consequence?"
    choix:
      - "No consequence"
      - "Answers should be made longer"
      - "Asking for concise answers and structured formats cuts the cost quickly"
      - "The system prompt should be removed"
    reponse: 2
    pourquoi: >-
      A wordy answer costs a lot on every call.
      A short JSON instead of a paragraph: less output.
      Reasoning tokens are often billed as output.
      Look at input and output prices separately.

  - id: latence-1
    sous_rubrique: latence
    question: "What does a p95 latency of 8 seconds mean?"
    choix:
      - "95% of requests answer in 8 seconds or less, 5% take longer"
      - "The average is 8 seconds"
      - "95 requests took 8 seconds"
      - "The slowest request took 8 seconds"
    reponse: 0
    pourquoi: >-
      p50: half the requests are faster.
      p95: 19 out of 20 are faster.
      The last twentieth is the tail.
      An active user runs into it often.

  - id: latence-2
    sous_rubrique: latence
    question: "Why not settle for the average latency?"
    choix:
      - "The average is always higher than p95"
      - "The average can't be computed"
      - "A few very slow requests hide inside a decent average"
      - "There is no reason"
    reponse: 2
    pourquoi: >-
      99 requests at 1 second and one at 60 seconds: average 1.6 seconds.
      The average looks good.
      One user in a hundred waited a minute.
      Percentiles show what the average smooths over.

  - id: latence-3
    sous_rubrique: latence
    question: "In “The Tail at Scale” (2013), a request queries 100 servers. Each is slow once in 100 times. What share of requests is slow?"
    choix:
      - "About 63%"
      - "1%"
      - "10%"
      - "100%"
    reponse: 0
    pourquoi: >-
      The request waits for the slowest of the 100.
      Probability that none is slow: 0.99 to the power 100, about 37%.
      So about 63% of requests are slow.
      The more steps, the more the tail dominates.

  - id: latence-4
    sous_rubrique: latence
    question: "An agent chains 10 model calls, one after the other. Effect on latency?"
    choix:
      - "No effect"
      - "The latency is that of a single call"
      - "Latency drops with each call"
      - "Latencies add up, and the risk of hitting a slow call grows"
    reponse: 3
    pourquoi: >-
      Ten calls in series are ten waits.
      Each call has its chance of being slow.
      The task's p95 is much worse than one call's p95.
      Fewer steps, or steps in parallel, cut the wait.

  - id: latence-5
    sous_rubrique: latence
    question: "Which measure best reflects the wait felt in a chat with streaming?"
    choix:
      - "The size of the model"
      - "Time to first token"
      - "The number of tokens in the system prompt"
      - "The price of the request"
    reponse: 1
    pourquoi: >-
      With streaming, the user reads from the first word.
      That first word sets the impression of speed.
      Total time matters for an agent that must finish before acting.
      You measure both.

  - id: latence-6
    sous_rubrique: latence
    question: "Where do you measure an agent's latency?"
    choix:
      - "Nowhere"
      - "Only on the provider's side"
      - "Only on the developer's machine"
      - "Per step (model, each tool) and end to end, at p50 and p95"
    reponse: 3
    pourquoi: >-
      End to end: what the user experiences.
      Per step: where the time goes.
      A slow tool hides behind a model wrongly blamed.
      The agent's trace carries these measurements.
---

## Cost per task {#cout-tache}

The bill says **how much**. The cost per task says **where** and **why**.

:::pourquoi
A task that costs 50 times the average drowns in the bill.
Per task, you see which flow, which customer, which prompt costs.
:::

An agent task is several calls. You add everything up:

| Line | Example |
|---|---|
| Input tokens | Prompt, history, tool results |
| Tokens read from cache | Stable prefix, at a reduced price |
| Output tokens | Answers, tool calls |
| Reasoning tokens | Often billed as output |
| Paid tools | Web search, external API |
| Retries | Errors, fallbacks, cascade |

:::pourquoi
Forgetting a line means underestimating the cost.
Failed tries cost as much as successful ones.
:::

The figure that matters: the **cost per successful task**.

:::pourquoi
100 tasks, $50, 80 successes: $0.625 per success.
A cheaper model that fails more can cost more per success.
:::

The cost of a step **grows** with the history.

:::pourquoi
Step 20 sends back the 19 before it.
Prompt caching and summarizing the history slow that growth.
:::

You **tag** each call: task, customer, feature.

```mermaid
sequenceDiagram
  participant A as Agent
  participant M as Model
  participant T as Paid tool
  participant C as Cost counter
  participant B as Dashboard
  A->>M: Call 1 (task 812, customer 42)
  M-->>A: 3,200 input tokens, 400 output
  A->>C: Adds the cost to task 812
  A->>T: Web search
  A->>C: Adds the tool's price
  A->>M: Call 2 (cached prefix)
  A->>C: Adds the cost to task 812
  C->>B: Cost per task, per customer, per feature
```

:::pourquoi
Without a tag, a cost has no owner.
The same counters feed the budgets and the quotas.
:::

## p50 and p95 latency {#latence}

The **average** hides the tail. You measure **percentiles**.

| Measure | Meaning |
|---|---|
| p50 | Half the requests are faster |
| p95 | 19 requests out of 20 are faster |
| p99 | 99 out of 100 are faster |

:::pourquoi
99 requests at 1 second, one at 60 seconds: average 1.6 seconds.
One user in a hundred waited a minute.
:::

The tail **dominates** when steps are chained.

:::pourquoi
Dean and Barroso (2013): a request queries 100 servers, each slow once in 100 times.
About 63% of requests hit at least one slow server.
An agent that chains calls lives the same effect.
:::

Two measures for two uses:

- **Time to first token**: the wait felt in a chat.
- **Total time**: what matters for an agent that must finish before acting.

:::pourquoi
With streaming, the user reads from the first word.
An agent only acts once the answer is complete.
:::

You measure **per step** and **end to end**.

:::pourquoi
End to end: what the user experiences.
Per step: where the time goes.
A slow tool often hides behind a model wrongly blamed.
:::

To shrink the tail:

- fewer steps in series;
- steps **in parallel** when they are independent;
- prompt **caching**;
- a **small model** for simple steps;
- a **maximum delay** per call, with a fallback.

:::pourquoi
Each step in series adds its wait and its risk of being slow.
Caching and the small model shorten each step.
The maximum delay cuts off the longest tail.
:::

## Key points

- Measure the cost per successful task, every line included.
- Tag each call: task, customer, feature.
- Latency at p50 and p95, not on average. Per step and end to end.
- Steps in series make the tail grow.
