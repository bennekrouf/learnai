---
id: "05-02"
titre: "Durability"
sous_rubriques:
  - id: checkpoint
    titre: "Checkpoint"
  - id: reprise
    titre: "Resuming after a crash"
  - id: idempotence
    titre: "Idempotency"
sources:
  - titre: "Anthropic — How we built our multi-agent research system, 2025 (resuming, checkpoints, rainbow deployments)"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Stripe — Idempotent requests"
    url: "https://docs.stripe.com/api/idempotent_requests"
  - titre: "AWS Builders' Library — Making retries safe with idempotent APIs"
    url: "https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/"
qcm:
  - id: checkpoint-1
    sous_rubrique: checkpoint
    question: "What is a checkpoint for an agent?"
    choix:
      - "A log message"
      - "A save of its state, after each step, in durable storage"
      - "A unit test"
      - "A summary sent to the user"
    reponse: 1
    pourquoi: >-
      The process can die at any moment.
      Whatever is only in memory disappears with it.
      The checkpoint writes the state to disk or to a database.
      You can start again from there.

  - id: checkpoint-2
    sous_rubrique: checkpoint
    question: "What should a checkpoint hold?"
    choix:
      - "The system prompt only"
      - "Only the model's last message"
      - "Only the time"
      - "The current state, the useful context, the tool results already obtained, and the format version"
    reponse: 3
    pourquoi: >-
      The state says where you stand.
      The tool results avoid redoing the work.
      The context lets the model carry on.
      The version lets you read an old checkpoint after an update.

  - id: checkpoint-3
    sous_rubrique: checkpoint
    question: "Where do you store checkpoints?"
    choix:
      - "In a variable of the process"
      - "In durable storage: a database or persistent files"
      - "In the model's context"
      - "In the browser cache"
    reponse: 1
    pourquoi: >-
      The checkpoint must survive the crash.
      A variable dies with the process.
      The model's context is not storage.
      A database is.

  - id: checkpoint-4
    sous_rubrique: checkpoint
    question: "The agent is waiting for approval from a human, who will answer tomorrow. What do you do with the process?"
    choix:
      - "Save the state and free the process. It resumes on the answer"
      - "Keep the process waiting until tomorrow"
      - "Cancel the task"
      - "Approve in the human's place"
    reponse: 0
    pourquoi: >-
      A process that waits a whole day may restart in the meantime.
      It also holds resources for nothing.
      With a checkpoint, waiting costs nothing.
      The human's answer restarts the task from the save.

  - id: reprise-1
    sous_rubrique: reprise
    question: "A two-hour task crashes at 1:50. Without a checkpoint, what happens?"
    choix:
      - "The model remembers everything"
      - "It resumes at 1:50"
      - "Everything starts again from the beginning"
      - "Nothing, the task counts as finished"
    reponse: 2
    pourquoi: >-
      Without a save, there is nothing to resume.
      You pay the time and the tokens again.
      Anthropic writes it: for a long agent, restarting from scratch is expensive and frustrating.
      They resume from where the error happened.

  - id: reprise-2
    sous_rubrique: reprise
    question: "What does the agent do when it resumes after a crash?"
    choix:
      - "It redoes every step, to be sure"
      - "It reloads the last checkpoint and goes on with the next step"
      - "It asks the model to guess where it was"
      - "It skips the task"
    reponse: 1
    pourquoi: >-
      Saved steps aren't redone.
      Their results are already there.
      Only the interrupted step is replayed.
      That is the step that needs idempotency.

  - id: reprise-3
    sous_rubrique: reprise
    question: "You deploy a new version while agents are running. How did Anthropic do it?"
    choix:
      - "Change the code under the running agents"
      - "Shut everything down and restart"
      - "Wait for the night"
      - "Old and new versions run together. Traffic moves to the new one gradually"
    reponse: 3
    pourquoi: >-
      A running agent follows the code it started with.
      Changing it on the way breaks its state.
      Anthropic calls these “rainbow deployments”.
      Running tasks finish on the old version.

  - id: reprise-4
    sous_rubrique: reprise
    question: "A checkpoint from yesterday has an old state format. The new code must read it. What do you plan for?"
    choix:
      - "Let the model interpret it"
      - "Delete the old checkpoints"
      - "A version number in the checkpoint, and a migration"
      - "Nothing, formats never change"
    reponse: 2
    pourquoi: >-
      The code changes. The saves stay.
      The version says how to read the checkpoint.
      A migration converts it to the new format.
      Without that, resuming crashes at the worst moment.

  - id: idempotence-1
    sous_rubrique: idempotence
    question: "What is an idempotent operation?"
    choix:
      - "Doing it twice has the same effect as doing it once"
      - "It is very fast"
      - "It can never fail"
      - "It costs nothing"
    reponse: 0
    pourquoi: >-
      “Set the status to paid”: twice, same result.
      “Add $10”: twice, $20.
      The first is idempotent. The second isn't.
      Replaying a step is only safe if it is.

  - id: idempotence-2
    sous_rubrique: idempotence
    question: "The agent sends a payment. It crashes before saving the checkpoint. On resuming, what is the risk?"
    choix:
      - "Losing the model"
      - "None"
      - "Paying twice"
      - "A timeout"
    reponse: 2
    pourquoi: >-
      The payment went out.
      The checkpoint doesn't know.
      Resuming replays the step.
      Without protection, the customer is charged twice.

  - id: idempotence-3
    sous_rubrique: idempotence
    question: "How does Stripe avoid double payments when a client retries?"
    choix:
      - "The client sends an idempotency key. Same key, same result, no new charge"
      - "It blocks every retry"
      - "It asks for a confirmation by text message"
      - "It refunds the double automatically"
    reponse: 0
    pourquoi: >-
      The key travels with the request, in a header.
      Stripe stores the result tied to the key.
      A second request with the same key gets that result.
      The operation isn't done again.

  - id: idempotence-4
    sous_rubrique: idempotence
    question: "How do you build the idempotency key of an agent step?"
    choix:
      - "The text of the model's answer"
      - "A new random number on each attempt"
      - "The current time"
      - "Generate it once and save it before the call, or derive it from the task id and the step"
    reponse: 3
    pourquoi: >-
      The key must stay the same from one attempt to the next.
      A new key on each try protects nothing.
      Task id plus step number is stable.
      The model's text changes with every generation.

  - id: idempotence-5
    sous_rubrique: idempotence
    question: "The email-sending API doesn't accept an idempotency key. What do you do?"
    choix:
      - "Send and hope"
      - "Record “send planned” with an id, check before sending, then mark it “sent”"
      - "Never retry any step"
      - "Send twice to be sure"
    reponse: 1
    pourquoi: >-
      You build idempotency on your side.
      Before sending, you note the intent with an id.
      On resuming, you check whether that id is already marked sent.
      A small window of doubt remains: you shrink it, you log it.
---

## Checkpoint {#checkpoint}

An agent can run for hours. The process can die at any moment.

A **checkpoint** saves the state after each step. In durable storage.

:::pourquoi
Whatever is only in memory dies with the process.
A database survives the crash.
:::

What it holds:

| Item | Used to |
|---|---|
| Current state | Know where you stand |
| Tool results | Not redo the work |
| Useful context | Let the model carry on |
| Format version | Read it after an update |

:::pourquoi
Each missing item forces you to redo or to guess.
:::

A checkpoint also makes **waiting** free. The agent is waiting for a human? Save, and free the process.

:::pourquoi
A process that waits a whole day holds resources.
And it may restart in the meantime.
:::

## Resuming after a crash {#reprise}

Resuming means reloading the last checkpoint and carrying on.

```mermaid
sequenceDiagram
  participant A as Agent (process 1)
  participant S as Checkpoint storage
  participant B as Agent (process 2)
  A->>S: Step 1 done, save
  A->>S: Step 2 done, save
  A->>A: Crash during step 3
  B->>S: Loads the last checkpoint
  S-->>B: State after step 2
  B->>B: Replays step 3 only
  B->>S: Step 3 done, save
```

:::pourquoi
Anthropic writes it about its research system.
Restarting a long agent from scratch is “expensive and frustrating”.
They resume from where the error happened.
:::

Deployments interrupt too. Anthropic runs the old and the new version side by side, and moves traffic over gradually.

:::pourquoi
A running agent follows the code it started with.
Changing that code on the way can break its state.
:::

Put a **version number** in every checkpoint.

:::pourquoi
The code changes. The saves stay.
The version says how to read it, and which migration to apply.
:::

## Idempotency {#idempotence}

On resuming, the interrupted step is **replayed**. It may already have acted.

```mermaid
sequenceDiagram
  participant A as Agent
  participant S as Storage
  participant P as Payment API
  A->>S: Step “pay”, key task-42-step-3
  A->>P: Payment $50, key task-42-step-3
  P-->>A: Payment accepted
  A->>A: Crash before the save
  A->>P: Resume: same payment, same key
  P-->>A: Result already known, no new charge
  A->>S: Step “pay” done
```

An **idempotent** operation has the same effect done once or twice.

:::pourquoi
“Set the status to paid”: twice, same result.
“Add $10”: twice, $20.
Only an idempotent step can be replayed safely.
:::

For APIs that allow it, send an **idempotency key**. Stripe, for example, takes it in a header and returns the same result for the same key.

:::pourquoi
The server recognizes the request it already handled.
It doesn't do it again.
:::

The key stays the **same** from one attempt to the next: task id plus step number, or generated once and saved before the call.

:::pourquoi
A new key on each try protects nothing.
The server would see two different requests.
:::

With no key on the API side, build idempotency yourself: note the intent, check before acting, mark it done.

:::pourquoi
The record says whether the action already happened.
A small window of doubt remains. You log it.
:::

## Key points

- A checkpoint after each step, in durable storage, with a version.
- Resuming: reload, then replay only the interrupted step.
- Every step that can be replayed must be idempotent.
- A stable idempotency key: the same key for each attempt.
