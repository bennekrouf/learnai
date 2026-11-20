---
id: "05-04"
titre: "Human in the loop"
sous_rubriques:
  - id: a-valider
    titre: "Steps to approve"
  - id: decision
    titre: "Approve or reject"
sources:
  - titre: "Anthropic — Project Vend, June 27, 2025"
    url: "https://www.anthropic.com/research/project-vend-1"
  - titre: "Anthropic — Building effective agents, December 19, 2024"
    url: "https://www.anthropic.com/engineering/building-effective-agents"
  - titre: "MCP — Tools, revision 2026-07-28 (human in the loop)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
qcm:
  - id: a-valider-1
    sous_rubrique: a-valider
    question: "Which steps of an agent should a human approve?"
    choix:
      - "All of them, without exception"
      - "The irreversible, money, what goes outside, production"
      - "None, the agent is autonomous"
      - "Only file reads"
    reponse: 1
    pourquoi: >-
      A reversible mistake can be fixed.
      A wire transfer, a sent email, a wiped database can't.
      You put the human where a mistake is costly.
      Elsewhere, they slow things down without protecting.

  - id: a-valider-2
    sous_rubrique: a-valider
    question: "Claudius gave in to discount requests on Slack. Which barrier would have helped?"
    choix:
      - "Any discount above a threshold goes into an “awaiting approval” state for a human"
      - "Asking it in the prompt to be firmer"
      - "Cutting off Slack"
      - "Forbidding customers to talk"
    reponse: 0
    pourquoi: >-
      Customer pressure worked on the model.
      A threshold in code can't be talked round.
      Small discounts go through on their own.
      Big ones wait for a human.

  - id: a-valider-3
    sous_rubrique: a-valider
    question: "Why not have everything approved, to be safe?"
    choix:
      - "Because humans always agree"
      - "Because it's forbidden"
      - "Too many requests, and the human ends up accepting everything without reading"
      - "Because it costs tokens"
    reponse: 2
    pourquoi: >-
      A hundred approvals a day become a reflex.
      The “approve” click no longer means anything.
      Fewer requests, better chosen: each one is really read.

  - id: a-valider-4
    sous_rubrique: a-valider
    question: "How do you set the approval threshold?"
    choix:
      - "The same for everything, once and for all"
      - "By risk: amount, reversibility, recipient, and the history of mistakes"
      - "The model chooses it"
      - "At random"
    reponse: 1
    pourquoi: >-
      A $5 refund and a $5,000 one don't carry the same risk.
      Nor do an internal email and an email to a customer.
      You start cautious.
      You loosen where approvals are always “yes”.

  - id: a-valider-5
    sous_rubrique: a-valider
    question: "The agent is waiting for approval. The human will answer in three hours. What happens to the task?"
    choix:
      - "It is cancelled after one minute"
      - "The process stays blocked for three hours"
      - "It carries on without waiting"
      - "It is saved in the “awaiting” state. The answer restarts it"
    reponse: 3
    pourquoi: >-
      A human pause can last hours.
      A checkpoint makes that wait free and safe.
      The answer arrives: you reload the state and carry on.
      Durability in the service of the human.

  - id: decision-1
    sous_rubrique: decision
    question: "What do you show the human who must approve?"
    choix:
      - "The whole raw conversation history"
      - "“The agent wants to act. OK?”"
      - "The exact action, its arguments, the reason the agent gave, and the expected effect"
      - "Nothing, to go fast"
    reponse: 2
    pourquoi: >-
      You can only judge what you see.
      “Refund” says neither how much nor to whom.
      The reason helps spot a flaw in the logic.
      The raw history drowns what matters.

  - id: decision-2
    sous_rubrique: decision
    question: "Which answers do you offer the human?"
    choix:
      - "Approve, reject with a reason, or edit the arguments"
      - "Only “OK”"
      - "Only approve or ignore"
      - "A free text field, no buttons"
    reponse: 0
    pourquoi: >-
      Approve: the action goes out as is.
      Reject: the reason guides the agent.
      Edit: the human fixes the amount without redoing everything.
      Three clear choices, a traceable decision.

  - id: decision-3
    sous_rubrique: decision
    question: "The human rejects the action. What does the agent do?"
    choix:
      - "It runs it anyway"
      - "It retries the same action"
      - "It gets the rejection and its reason, and proposes something else or stops"
      - "It crashes"
    reponse: 2
    pourquoi: >-
      The rejection is information.
      “Too expensive” invites a lower offer.
      “Wrong customer” invites a check.
      Retrying the same action ignores the human.

  - id: decision-4
    sous_rubrique: decision
    question: "Nobody answers the approval request for 48 hours. What do you do?"
    choix:
      - "Don't run it: expire the request and notify"
      - "Run it: silence means consent"
      - "Wait forever without notifying"
      - "Choose at random"
    reponse: 0
    pourquoi: >-
      No answer is not agreement.
      By default, the sensitive action doesn't happen.
      You expire the request, alert, record it.
      That is the fail-safe principle.

  - id: decision-5
    sous_rubrique: decision
    question: "The human approves a $50 refund. Then the agent changes the amount to $500. What do you do?"
    choix:
      - "Run both"
      - "Run it, it's already approved"
      - "Run $50 and silently ignore the change"
      - "Ask again: the approval was for $50, not $500"
    reponse: 3
    pourquoi: >-
      An approval holds for one precise action.
      You tie it to the exact arguments, for example with a hash.
      If the arguments change, the approval no longer holds.
      Otherwise you approve one thing and run another.

  - id: decision-6
    sous_rubrique: decision
    question: "What do you log for each approval?"
    choix:
      - "Nothing, it's private"
      - "Who decided, when, on which exact action, and which decision"
      - "Only the rejections"
      - "Only the time"
    reponse: 1
    pourquoi: >-
      After an incident, you need to know who approved what.
      Approvals that are always positive show where to loosen the threshold.
      Frequent rejections show where the agent goes wrong.
---

## Steps to approve {#a-valider}

Some steps go through a **human** before acting.

Which ones:

- the irreversible: deleting, sending, publishing;
- money: paying, refunding, granting a discount;
- what goes out: an email, a message, a request to the outside;
- production.

:::pourquoi
A reversible mistake can be fixed.
A transfer that has gone out can't.
The human goes where a mistake is costly.
:::

The threshold depends on the **risk**: amount, reversibility, recipient.

:::pourquoi
Claudius gave in to discount requests on Slack.
A threshold in code can't be talked round.
Above it, the human decides.
:::

No approval everywhere.

:::pourquoi
Too many requests, and the human clicks “yes” by reflex.
Approval loses all value.
:::

In the state machine, approval is a **state**: “awaiting”. The task is saved and the process freed.

:::pourquoi
A human may answer in three hours.
A checkpoint makes that wait free.
:::

## Approve or reject {#decision}

```mermaid
sequenceDiagram
  participant M as Model
  participant O as Orchestrator
  participant S as Storage
  participant H as Human
  M->>O: refund(customer=812, amount=$500)
  O->>O: Guard: amount > $100 threshold
  O->>S: “Awaiting” state, action and hash of the arguments
  O->>H: Action, arguments, agent's reason. Approve, reject, edit?
  H-->>O: Edited: $200
  O->>S: Loads the state, records the decision
  O->>M: Approved with an edit: $200
```

Show the **exact** action: tool, arguments, the reason the agent gave, expected effect.

:::pourquoi
“Refund” says neither how much nor to whom.
You can only judge what you see.
:::

Three answers: **approve**, **reject** with a reason, **edit**.

:::pourquoi
The reason for a rejection guides the agent.
An edit avoids starting all over.
:::

A rejection is sent back to the model. It proposes something else, or stops.

:::pourquoi
Retrying the same action ignores the human.
:::

The approval holds for **these** arguments. If they change, you ask again.

:::pourquoi
Otherwise you approve $50 and run $500.
A hash of the arguments ties the decision to the action.
:::

No answer? The action **doesn't happen**.

:::pourquoi
Silence is not agreement.
The request expires, you notify, you record it.
:::

Log every decision: who, when, what, which answer.

:::pourquoi
After an incident, you know who approved what.
And the numbers show where to adjust the threshold.
:::

## Key points

- Approve the irreversible, money, what goes outside, production. Not the rest.
- Approval is a saved state, not a process that waits.
- Show the exact action. Approve, reject, edit.
- Approval tied to the arguments. No answer, nothing goes out. Everything is logged.
