---
id: "09-01"
titre: "Architecture decisions"
sous_rubriques:
  - id: adr
    titre: "ADR"
  - id: compromis
    titre: "Justifying a trade-off"
sources:
  - titre: "Michael Nygard — Documenting Architecture Decisions, November 15, 2011"
    url: "https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions"
  - titre: "Anthropic — How we built our multi-agent research system, June 13, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, June 12, 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
qcm:
  - id: adr-1
    sous_rubrique: adr
    question: "What is an ADR?"
    choix:
      - "The API documentation"
      - "A class diagram"
      - "A bug ticket"
      - "A short document recording an architecture decision, its context and its consequences"
    reponse: 3
    pourquoi: >-
      ADR stands for Architecture Decision Record.
      Michael Nygard proposed the format in 2011.
      One decision per file, one or two pages at most.
      It answers the question “why is it done this way?”.

  - id: adr-2
    sous_rubrique: adr
    question: "Which sections does the ADR proposed by Michael Nygard have?"
    choix:
      - "Summary, Code, Tests"
      - "Title, Context, Decision, Status, Consequences"
      - "Problem, Solution, Price"
      - "Author, Date, Signature"
    reponse: 1
    pourquoi: >-
      The context gives the forces at play.
      The decision says what you do.
      The status says whether it still holds.
      The consequences say what it costs and what it brings.

  - id: adr-3
    sous_rubrique: adr
    question: "What should the Consequences section hold?"
    choix:
      - "The positive, negative and neutral effects of the decision"
      - "Only the advantages"
      - "The name of whoever decided"
      - "The code of the solution"
    reponse: 0
    pourquoi: >-
      A decision with only advantages isn't a decision.
      The accepted drawbacks are the most useful information.
      They tell the future reader what was known.
      And what must be watched.

  - id: adr-4
    sous_rubrique: adr
    question: "A decision is replaced six months later. What do you do with the old ADR?"
    choix:
      - "Rewrite it with the new decision"
      - "Delete it"
      - "Keep it, mark it “superseded” and point to the new ADR"
      - "Leave it as is without saying anything"
    reponse: 2
    pourquoi: >-
      The history explains the path taken.
      You know why you chose otherwise, and why you changed.
      Rewriting erases that trace.
      Numbers are never reused.

  - id: adr-5
    sous_rubrique: adr
    question: "Where do you keep ADRs?"
    choix:
      - "In the architect's head"
      - "In the project repository, in Markdown, next to the code"
      - "In an email"
      - "On a whiteboard"
    reponse: 1
    pourquoi: >-
      In the repository, the ADR follows the code.
      It is reviewed in a PR, like code.
      It can be found with a search.
      Nygard suggested doc/arch/adr-NNN.md.

  - id: adr-6
    sous_rubrique: adr
    question: "Which decision deserves an ADR?"
    choix:
      - "Changing a button's color"
      - "Renaming a local variable"
      - "Fixing a typo in a comment"
      - "Choosing a single agent rather than a multi-agent system"
    reponse: 3
    pourquoi: >-
      An ADR is for what is costly to change.
      The agent topology touches everything: cost, latency, reliability.
      Renaming a variable is undone in a minute.
      Too many ADRs drown the important ones.

  - id: compromis-1
    sous_rubrique: compromis
    question: "What does justifying a trade-off mean?"
    choix:
      - "Choosing the newest technology"
      - "Saying the solution has no flaws"
      - "Saying what you gain, what you lose, and why that price is acceptable here"
      - "Copying what a big company does"
    reponse: 2
    pourquoi: >-
      Every architecture pays for something.
      The trade-off says what, and against what.
      The “here” matters: the same price may be unacceptable elsewhere.

  - id: compromis-2
    sous_rubrique: compromis
    question: "Anthropic gains 90.2% with its multi-agent research system. Why not adopt it everywhere?"
    choix:
      - "It uses about 15 times more tokens, and it is a poor fit for shared-context tasks"
      - "Because it is forbidden"
      - "Because it is slower in every case"
      - "There is no reason, it should be adopted everywhere"
    reponse: 0
    pourquoi: >-
      The gain has a price: the volume of tokens.
      Anthropic says the task must be worth that price.
      And that tasks with many dependencies, like code, suit it poorly.
      The gain depends on the context.

  - id: compromis-3
    sous_rubrique: compromis
    question: "Cognition and Anthropic reach opposite conclusions. What explains the difference?"
    choix:
      - "They use different programming languages"
      - "One of them got it wrong"
      - "Their tasks: code with linked parts for one, research in independent leads for the other"
      - "Chance"
    reponse: 2
    pourquoi: >-
      Code has dependencies: the background and the bird must match.
      Research splits into independent leads.
      Same question, different contexts, different answers.
      That is why the context is written in the ADR.

  - id: compromis-4
    sous_rubrique: compromis
    question: "How do you make a trade-off verifiable?"
    choix:
      - "Put numbers on it: cost, latency, quality measured on the same test set for each option"
      - "Write it in capitals"
      - "Have the model approve it"
      - "Don't write it down"
    reponse: 0
    pourquoi: >-
      “More reliable” can't be argued with.
      “12% more successes for 3 times the cost” can.
      The numbers come from steps 7 and 8.
      A reader can redo the measurement.

  - id: compromis-5
    sous_rubrique: compromis
    question: "Why write down the rejected options in the ADR?"
    choix:
      - "It isn't useful"
      - "To make the document longer"
      - "To criticize their authors"
      - "So nobody proposes them again without knowing why they lost"
    reponse: 3
    pourquoi: >-
      In six months, someone will propose the rejected option.
      With the ADR, they see the reasons it was turned down.
      If the context has changed, they can reopen the decision.
      If not, you save a discussion.

  - id: compromis-6
    sous_rubrique: compromis
    question: "What is a revisit condition in an ADR?"
    choix:
      - "The date of the next meeting"
      - "The signal that should reopen the decision, for example “if the cost per task goes over $0.50”"
      - "The number of reviewers"
      - "A formatting rule"
    reponse: 1
    pourquoi: >-
      A decision holds as long as its context holds.
      Writing what would make it wrong prepares the next choice.
      A new model, a new price, a new volume can change everything.
---

## ADR {#adr}

An **ADR** (Architecture Decision Record) records an architecture decision. One decision per file.

:::pourquoi
In six months, someone will ask “why is it done this way?”.
Without a record, the answer leaves with the person who left.
:::

Michael Nygard's format (2011):

| Section | Content |
|---|---|
| Title | A number and a short phrase |
| Context | The forces at play, without bias |
| Decision | “We will…”, in the active voice |
| Status | Proposed, accepted, deprecated, superseded |
| Consequences | Positive, negative and neutral effects |

:::pourquoi
The context explains. The decision settles.
The consequences state the accepted price.
One or two pages: otherwise, nobody reads it.
:::

An example:

```markdown
# ADR 007: A single agent for customer support

Status: accepted

## Context
Support tasks chain linked steps:
identity, order, refund. Each step depends on the one before.

## Decision
We will use a single agent with a shared history.

## Rejected options
Supervisor and three subagents: 2.8 times more tokens,
and two inconsistencies over 50 test cases.

## Consequences
+ Continuous context, fewer inconsistencies.
- No parallelism: p95 latency of 14 s.
- The history grows: a summary is needed beyond 30 steps.

## Revisit condition
If p95 latency goes over 20 s, or if independent tasks appear.
```

A superseded ADR is **never deleted**. It is marked “superseded by ADR 012”.

:::pourquoi
The history shows the path taken.
You see why you chose otherwise, and what changed.
:::

ADRs live **in the repository**, next to the code.

:::pourquoi
They are reviewed in a PR, like code.
They can be found with a search.
:::

An ADR for what is **costly to change**: agent topology, model, vector database, evaluation strategy.

:::pourquoi
Too many ADRs drown the important ones.
Renaming a variable doesn't deserve a document.
:::

## Justifying a trade-off {#compromis}

Every architecture **pays** for something. The trade-off says what, against what, and why it is acceptable **here**.

:::pourquoi
Anthropic gains 90.2% with its multi-agent research system.
It pays about 15 times more tokens than a chat.
For high-value research, the price is justified.
:::

The same choice can be good here and bad there.

| | Cognition | Anthropic |
|---|---|---|
| Task | Writing code | Doing research |
| Parts | Linked together | Independent |
| Choice | One agent, one thread | Lead agent and subagents in parallel |
| Accepted price | No parallelism | Many more tokens |

:::pourquoi
Flappy Bird's background and bird must match.
Two research leads don't.
Anthropic itself notes that most coding tasks suit it poorly.
:::

A trade-off is **quantified**.

:::pourquoi
“More reliable” can't be argued with.
“12% more successes for 3 times the cost” can.
The numbers come from the test set (step 7) and the cost measurement (step 8).
:::

You also write down the **rejected options**, and a **revisit condition**.

:::pourquoi
The rejected option will come back on the table.
The revisit condition says when the decision must be reopened.
A new model or a new price can change everything.
:::

## Key points

- An ADR: title, context, decision, status, consequences. One or two pages.
- Never deleted, only superseded. Kept in the repository.
- A trade-off says what you gain, what you pay, and why here.
- Quantify, write down the rejected options, plan the revisit condition.
