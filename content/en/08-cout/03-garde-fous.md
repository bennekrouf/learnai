---
id: "08-03"
titre: "Guardrails"
sous_rubriques:
  - id: budgets
    titre: "Budgets"
  - id: coupure
    titre: "Automatic cutoff"
  - id: quotas
    titre: "Quotas"
sources:
  - titre: "TechCrunch — Anthropic unveils new rate limits to curb Claude Code power users, July 28, 2025"
    url: "https://techcrunch.com/2025/07/28/anthropic-unveils-new-rate-limits-to-curb-claude-code-power-users/"
  - titre: "OWASP — LLM10:2025 Unbounded Consumption"
    url: "https://genai.owasp.org/llmrisk/llm102025-unbounded-consumption/"
qcm:
  - id: budgets-1
    sous_rubrique: budgets
    question: "Why does an agent need a budget, when a single model call often does without one?"
    choix:
      - "It is a legal obligation"
      - "An agent always costs less"
      - "Agents don't use tokens"
      - "An agent chains calls in a loop: nothing naturally bounds its cost"
    reponse: 3
    pourquoi: >-
      A single call has a predictable cost.
      An agent decides on its own to call the model again.
      A loop that doesn't end properly runs until it is stopped.
      The budget is that stop.

  - id: budgets-2
    sous_rubrique: budgets
    question: "At which levels do you set a budget?"
    choix:
      - "Only for the whole organization, once a year"
      - "Per task, per user or customer, and for the whole organization"
      - "Only per request"
      - "No level, you look at the bill"
    reponse: 1
    pourquoi: >-
      Per task: one loop doesn't ruin everything.
      Per user: an abuse stays local.
      For the organization: the month's bill stays under control.
      Each level stops a different problem.

  - id: budgets-3
    sous_rubrique: budgets
    question: "In what units do you express an agent task's budget?"
    choix:
      - "Several limits: tokens, number of steps, tool calls, duration, amount of money"
      - "Only the number of words in the answer"
      - "The number of users"
      - "A percentage of CPU"
    reponse: 0
    pourquoi: >-
      Tokens measure the model's cost.
      Steps and tool calls measure the loop.
      Duration measures the wait.
      A paid tool counts in money, not tokens.

  - id: budgets-4
    sous_rubrique: budgets
    question: "A task reaches 80% of its budget. What do you do?"
    choix:
      - "Nothing until 100%"
      - "Double the budget automatically"
      - "Alert, and let the agent wrap up with what it has"
      - "Delete the work done"
    reponse: 2
    pourquoi: >-
      An alert threshold before the limit gives time to react.
      The agent can summarize and return a partial result.
      At 100%, you cut it off.
      A budget that grows on its own isn't a budget.

  - id: coupure-1
    sous_rubrique: coupure
    question: "Which signal should automatically cut off an agent?"
    choix:
      - "A polite answer"
      - "The same tool call repeated several times with the same arguments"
      - "A short question"
      - "A user being logged in"
    reponse: 1
    pourquoi: >-
      An identical call repeated signals a loop.
      The agent is no longer making progress. It is consuming.
      Catching it early avoids paying a hundred times for the same failure.

  - id: coupure-2
    sous_rubrique: coupure
    question: "Where do you put the code that cuts off an agent?"
    choix:
      - "Nowhere, the agent stops on its own"
      - "In the prompt: “Stop if you are looping”"
      - "In the model's answer"
      - "In the orchestrator, outside the model"
    reponse: 3
    pourquoi: >-
      A model that is looping doesn't know it is looping.
      An instruction in the prompt guarantees nothing.
      The orchestrator counts the steps and the tokens.
      It cuts off, whether the model agrees or not.

  - id: coupure-3
    sous_rubrique: coupure
    question: "After an automatic cutoff, what should remain?"
    choix:
      - "An immediate automatic restart of the same task"
      - "Nothing, everything is wiped"
      - "A saved state, the reason for stopping, and a clear message for the user"
      - "An invoice sent to the customer"
    reponse: 2
    pourquoi: >-
      The state lets you resume after a fix.
      The reason helps fix the cause.
      The message avoids an endless wait.
      Restarting as is runs the same loop again.

  - id: coupure-4
    sous_rubrique: coupure
    question: "The whole organization's spending triples in an hour. What should a circuit breaker do?"
    choix:
      - "Suspend new non-critical tasks and alert a person"
      - "Wait for the end-of-month bill"
      - "Delete the users' accounts"
      - "Send all traffic to the biggest model"
    reponse: 0
    pourquoi: >-
      A sudden spike comes from a bug, a loop or an abuse.
      Waiting for the bill means paying before understanding.
      The circuit breaker limits the damage.
      A person then decides.

  - id: quotas-1
    sous_rubrique: quotas
    question: "What is the difference between a budget and a quota?"
    choix:
      - "A quota is always in dollars"
      - "No difference"
      - "A budget bounds spending, a quota bounds usage per period for a user or a customer"
      - "A budget only concerns hardware"
    reponse: 2
    pourquoi: >-
      Budget: how much a task or the organization may cost.
      Quota: how much a user may consume, per hour, per day, per week.
      The quota protects the service and the other users.

  - id: quotas-2
    sous_rubrique: quotas
    question: "Why did Anthropic add a weekly limit to the five-hour limit for Claude Code?"
    choix:
      - "The five-hour limit didn't stop continuous use, day and night"
      - "To slow down every user"
      - "Because the five-hour limit was too strict"
      - "To get rid of subscriptions"
    reponse: 0
    pourquoi: >-
      A five-hour window keeps resetting.
      A background agent uses it up, waits, starts again.
      A long window bounds total consumption.
      Two windows, two behaviors targeted.

  - id: quotas-3
    sous_rubrique: quotas
    question: "A user reaches their quota. What is the product's best response?"
    choix:
      - "Cut off the account without warning"
      - "Return a 500 error with no explanation"
      - "Pretend to answer"
      - "Say it clearly: what was reached, when it resets, and the paid option if there is one"
    reponse: 3
    pourquoi: >-
      A surprise quota looks like an outage.
      An explained quota is a rule.
      Anthropic lets Max subscribers buy extra usage at API rates.
      The user chooses knowingly.

  - id: quotas-4
    sous_rubrique: quotas
    question: "Which OWASP risk covers unbounded consumption of an LLM service?"
    choix:
      - "A mere spelling mistake"
      - "LLM10 “Unbounded Consumption”: exploding costs, denial of service, model theft through massive querying"
      - "The choice of font"
      - "None, it isn't a security risk"
    reponse: 1
    pourquoi: >-
      OWASP ranks unbounded consumption among the ten risks of LLM applications.
      An attacker can make the victim pay by flooding their service.
      Quotas and rate limits are the basic defense.

  - id: quotas-5
    sous_rubrique: quotas
    question: "A customer shares their access with ten people. Which guardrail detects it?"
    choix:
      - "The semantic cache"
      - "None, it can't be detected"
      - "A longer system prompt"
      - "A quota per account, and an alert on abnormal usage for a single user"
    reponse: 3
    pourquoi: >-
      Ten people consume like ten.
      The quota caps the account.
      The alert flags an abnormal usage profile.
      Anthropic cited sharing and reselling access among the abuses targeted.
---

## Budgets {#budgets}

A model call has a predictable cost. An **agent** doesn't.

:::pourquoi
The agent decides on its own to call the model again.
A loop that doesn't end properly runs as long as nobody stops it.
At Anthropic, a 200-dollar subscriber used tens of thousands of dollars of compute.
:::

A budget at **three levels**:

| Level | Example | What it stops |
|---|---|---|
| Task | 200,000 tokens, 30 steps, 10 minutes | A loop |
| User or customer | $20 a day | An abuse |
| Organization | $5,000 a month | A bill out of control |

:::pourquoi
Each level stops a different problem.
A single level lets the other two through.
:::

A task's budget combines several **units**: tokens, steps, tool calls, duration, money.

:::pourquoi
A paid tool, such as a web search, counts in money.
A loop of short calls shows up in steps, not in tokens.
:::

An **alert** before the limit. A **cutoff** at the limit.

:::pourquoi
At 80%, the agent can wrap up with what it has.
At 100%, you cut it off.
A budget that grows on its own isn't a budget.
:::

## Automatic cutoff {#coupure}

Some signals must **stop** the agent without waiting for the budget.

| Signal | What it reveals |
|---|---|
| Same tool call, same arguments, several times | A loop |
| Same tool error repeated | A misunderstood schema |
| No progress toward the goal after N steps | A lost agent |
| Overall spending triples in an hour | A bug, a loop or an abuse |

:::pourquoi
A looping agent no longer makes progress. It consumes.
Catching it early avoids paying a hundred times for the same failure.
:::

The cutoff lives in the **orchestrator**, not in the prompt.

```mermaid
sequenceDiagram
  participant O as Orchestrator
  participant M as Model
  participant T as Tool
  participant C as Counters
  participant U as User
  O->>M: Step 12
  M-->>O: search("invoice 512")
  O->>C: Same call already seen 3 times?
  C-->>O: Yes
  O->>O: Saves the state, reason “loop”
  O-->>U: “Task stopped: the agent was going round in circles. It can be resumed.”
  Note over O,T: The tool isn't called again
```

:::pourquoi
A looping model doesn't know it is looping.
An instruction “stop if you are looping” guarantees nothing.
The code counts, and cuts off.
:::

After the cutoff: a **saved state**, the **reason**, a **clear message**.

:::pourquoi
The state lets you resume after a fix.
The reason helps fix the cause.
Restarting as is runs the same loop again.
:::

At the organization level, a **circuit breaker** suspends what isn't critical and alerts a person.

:::pourquoi
Waiting for the bill means paying before understanding.
:::

## Quotas {#quotas}

A **quota** bounds a user's or a customer's usage over a period.

:::pourquoi
The budget protects your bill.
The quota also protects the service and the other users.
:::

Two **windows** complement each other:

| Window | What it stops |
|---|---|
| Short (a minute, a few hours) | Bursts |
| Long (a day, a week) | Continuous use |

:::pourquoi
Anthropic already had a limit that resets every five hours.
Agents ran day and night, from one window to the next.
The weekly limit, announced in July 2025, bounds the total.
:::

The quota also targets **sharing** and **reselling** access.

:::pourquoi
Ten people on one account consume like ten.
Anthropic cited these practices among the abuses targeted.
:::

A reached quota is **explained**: what, until when, and the paid option if there is one.

:::pourquoi
A surprise quota looks like an outage.
An explained quota is a rule.
:::

OWASP makes it a risk of its own: **LLM10, Unbounded Consumption**.

:::pourquoi
An attacker can flood your service and send you the bill.
Quotas and rate limits are the basic defense.
:::

## Key points

- An agent has no natural ceiling: you have to give it one.
- Budgets per task, per user, for the organization. Alert before, cut off at the limit.
- The cutoff lives in the orchestrator's code, not in the prompt.
- Quotas on two windows, explained to the user.
