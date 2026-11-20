---
id: "05-03"
titre: "Failures"
sous_rubriques:
  - id: timeouts
    titre: "Timeouts"
  - id: backoff
    titre: "Exponential backoff and jitter"
  - id: limites
    titre: "Capping retries"
sources:
  - titre: "Marc Brooker — Exponential Backoff And Jitter, AWS Architecture Blog, March 2015"
    url: "https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/"
  - titre: "AWS Builders' Library — Timeouts, retries, and backoff with jitter"
    url: "https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/"
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
qcm:
  - id: timeouts-1
    sous_rubrique: timeouts
    question: "Why put a timeout on every tool or model call?"
    choix:
      - "Because JSON requires it"
      - "To speed up the network"
      - "To lower the price of tokens"
      - "Without a timeout, a stuck call blocks the agent forever"
    reponse: 3
    pourquoi: >-
      A server may never answer.
      With no limit, the agent waits forever.
      The timeout turns that silence into an error.
      An error is something you can handle.

  - id: timeouts-2
    sous_rubrique: timeouts
    question: "How do you choose a timeout value?"
    choix:
      - "Always 1 second"
      - "From the measured latency: a high percentile, with a margin"
      - "Always 10 minutes"
      - "At random"
    reponse: 1
    pourquoi: >-
      Too short: you cut calls that were going to succeed.
      Too long: you wait for nothing.
      AWS recommends starting from the observed latency, at a high percentile.
      You accept cutting a small share of very slow calls.

  - id: timeouts-3
    sous_rubrique: timeouts
    question: "Each call has its timeout. Do you also need a timeout for the whole task?"
    choix:
      - "Yes: short but very many calls can last hours"
      - "No, it's redundant"
      - "Only at weekends"
      - "Only for small models"
    reponse: 0
    pourquoi: >-
      A hundred 30-second calls make 50 minutes.
      Each one respects its timeout.
      The whole task goes past any reasonable delay.
      An overall deadline protects the user who is waiting.

  - id: timeouts-4
    sous_rubrique: timeouts
    question: "A payment call goes past its timeout. Did the payment fail?"
    choix:
      - "No, never"
      - "Yes, necessarily"
      - "You don't know: it may have succeeded on the server side"
      - "Yes, and it is refunded automatically"
    reponse: 2
    pourquoi: >-
      The timeout only says: no answer in time.
      The server may have handled the request, then answered too late.
      Retrying without care can pay twice.
      Hence the idempotency key.

  - id: backoff-1
    sous_rubrique: backoff
    question: "What is exponential backoff?"
    choix:
      - "Retrying immediately, in a loop"
      - "Waiting longer and longer between attempts: 1 s, 2 s, 4 s, 8 s…"
      - "Always waiting 5 seconds"
      - "Shrinking the prompt on each try"
    reponse: 1
    pourquoi: >-
      An overloaded service needs time to recover.
      Retrying right away makes the overload worse.
      Doubling the wait leaves more and more room.
      And it limits the number of useless calls.

  - id: backoff-2
    sous_rubrique: backoff
    question: "Why add randomness (jitter) to backoff?"
    choix:
      - "To fool the server"
      - "To make tests unpredictable"
      - "To save tokens"
      - "So all the clients don't retry at the same instant"
    reponse: 3
    pourquoi: >-
      A thousand clients fail together.
      Without jitter, they all retry after 1 s, then 2 s, then 4 s.
      Each wave hits the server again at the same time.
      Jitter spreads the attempts over time.

  - id: backoff-3
    sous_rubrique: backoff
    question: "The “full jitter” described by Marc Brooker (AWS, 2015) is…"
    choix:
      - "A fixed wait of 0 seconds"
      - "The exponential delay plus one fixed second"
      - "A wait drawn at random between 0 and the capped exponential delay"
      - "A delay that shrinks with each try"
    reponse: 2
    pourquoi: >-
      You compute the delay: base × 2 to the power n, capped.
      You draw a wait at random between 0 and that delay.
      In his simulations, the version without jitter is clearly the worst.
      The principle is still a pillar at Amazon.

  - id: backoff-4
    sous_rubrique: backoff
    question: "Why cap the exponential delay?"
    choix:
      - "Without a cap, the 15th attempt would wait for hours"
      - "To comply with JSON-RPC"
      - "So the server doesn't get bored"
      - "No reason"
    reponse: 0
    pourquoi: >-
      1 s doubled 15 times is more than 9 hours.
      The cap keeps the wait reasonable.
      For example 30 or 60 seconds at most.

  - id: backoff-5
    sous_rubrique: backoff
    question: "The API answers 429 with a Retry-After: 20 header. What do you do?"
    choix:
      - "Give up for good"
      - "Retry right away"
      - "Wait at least 20 seconds before retrying"
      - "Switch API keys to get around the limit"
    reponse: 2
    pourquoi: >-
      429 means: too many requests.
      Retry-After says when to come back.
      The server knows better than you when it will be ready.
      Getting around the limit breaks the service's terms.

  - id: limites-1
    sous_rubrique: limites
    question: "Which error is worth retrying?"
    choix:
      - "A 503 or a timeout: often transient"
      - "A 400: malformed request"
      - "A 401: not authenticated"
      - "A 403: forbidden"
    reponse: 0
    pourquoi: >-
      A transient error may be gone on the next try.
      A malformed request will stay malformed.
      A refused access will stay refused.
      Retrying those only wastes time.

  - id: limites-2
    sous_rubrique: limites
    question: "How many times should you retry?"
    choix:
      - "As many as the model asks"
      - "Forever"
      - "Once only, never more"
      - "A set, small number. Then an explicit failure and escalation"
    reponse: 3
    pourquoi: >-
      After a few tries, the outage is no longer transient.
      Carrying on loads the service and delays the response.
      The state moves to “failed”.
      A human or an alert takes over.

  - id: limites-3
    sous_rubrique: limites
    question: "The agent retries 3 times. The tool retries 3 times. The API behind it retries 3 times. How many calls at worst?"
    choix:
      - "9"
      - "27"
      - "3"
      - "6"
    reponse: 1
    pourquoi: >-
      Attempts multiply from one layer to the next.
      3 × 3 × 3 = 27 calls for a single request.
      In the middle of an outage, that is what finishes the service off.
      Retry at one layer only, preferably.

  - id: limites-4
    sous_rubrique: limites
    question: "The tool returns “invalid date: expected YYYY-MM-DD format”. Retry the same call?"
    choix:
      - "No, and stop the agent without a word"
      - "Yes, five times with backoff"
      - "Yes, immediately"
      - "No: send the error back to the model so it fixes the argument"
    reponse: 3
    pourquoi: >-
      The same argument will give the same error.
      Backoff doesn't fix a badly written date.
      The model can fix it.
      Transient error: retry. Content error: send it back to the model.
---

## Timeouts {#timeouts}

Any call can go unanswered. Every call has a **timeout**.

:::pourquoi
With no limit, the agent waits forever.
The timeout turns silence into an error you can handle.
:::

Choose the value from the **measured latency**. A high percentile, with a margin.

:::pourquoi
Too short, you cut calls that were going to succeed.
Too long, you wait for nothing.
:::

Add an **overall deadline** for the whole task.

:::pourquoi
A hundred 30-second calls each respect their timeout.
Together, they make 50 minutes.
:::

A timeout doesn't say the operation failed.

:::pourquoi
The server may have handled it, then answered too late.
Before retrying an action, you need idempotency.
:::

## Exponential backoff and jitter {#backoff}

You don't retry right away. You wait, longer and longer: that is **exponential backoff**.

You add **jitter**: each client draws its wait at random.

```mermaid
sequenceDiagram
  participant C1 as Agent 1
  participant C2 as Agent 2
  participant C3 as Agent 3
  participant S as Overloaded API
  C1->>S: Call
  C2->>S: Call
  C3->>S: Call
  S-->>C1: 503
  S-->>C2: 503
  S-->>C3: 503
  Note over C1,C3: Without jitter: all three come back together, at 1 s, 2 s, 4 s
  C2->>S: Retry after 0.3 s
  C1->>S: Retry after 0.8 s
  C3->>S: Retry after 1.7 s
  Note over C1,S: With jitter: the attempts spread out
```

:::pourquoi
A thousand clients retrying at the same instant recreate the overload.
Marc Brooker (AWS, 2015) simulated it: without jitter is clearly the worst option.
:::

The “full jitter” formula:

```text
wait = random(0, min(cap, base × 2^attempt))
```

:::pourquoi
The exponential leaves the service more and more room.
The cap avoids waiting for hours.
The randomness spreads the clients out.
:::

The server says when to come back? Listen to it. A `429` with `Retry-After: 20` means: not before 20 seconds.

:::pourquoi
The server knows its load better than you do.
:::

## Capping retries {#limites}

Not every error is retried.

| Error | Retry? | Why |
|---|---|---|
| Timeout, 503, 429 | Yes, with backoff | Often transient |
| 400, invalid argument | No, send it back to the model | It will happen again |
| 401, 403 | No, escalate | A permission is missing |

:::pourquoi
Backoff doesn't fix a badly written date.
The model can fix it, if you tell it why.
:::

Set a **maximum number** of attempts. Then: “failed” state, alert, human.

:::pourquoi
After a few tries, the outage is no longer transient.
Insisting loads the service and delays the response.
:::

Retry at **one layer only**.

```mermaid
flowchart LR
  A["Agent: 3 tries"] --> O["Tool: 3 tries each"]
  O --> P["API: 3 tries each"]
  P --> R["At worst 27 calls for one request"]
```

:::pourquoi
Attempts multiply from one layer to the next.
In the middle of an outage, they are what finishes the service off.
:::

## Key points

- One timeout per call, plus an overall deadline. A timeout is not a sure failure.
- Exponential backoff, capped, with jitter. Respect `Retry-After`.
- Retry only transient errors. Content errors go to the model.
- Few attempts, at one layer only. Then an explicit failure.
