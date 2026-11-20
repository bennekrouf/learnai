---
id: "01-03"
titre: "Three-tier memory"
sous_rubriques:
  - id: travail
    titre: "Working memory"
  - id: episodique
    titre: "Episodic memory"
  - id: semantique
    titre: "Semantic memory"
sources:
  - titre: "Packer et al. — MemGPT: Towards LLMs as Operating Systems, 2023"
    url: "https://arxiv.org/abs/2310.08560"
  - titre: "Tulving — Episodic and semantic memory, 1972 (a distinction used in cognitive psychology)"
    url: "https://en.wikipedia.org/wiki/Episodic_memory"
qcm:
  - id: travail-1
    sous_rubrique: travail
    question: "During a task, the agent writes down its 5-step plan and ticks off the ones done. Which memory?"
    choix:
      - "Working memory"
      - "Episodic memory"
      - "Semantic memory"
      - "None, it is the answer"
    reponse: 0
    pourquoi: >-
      The plan serves the task at hand.
      It changes at every step.
      It is of no use once the task is done.
      That is the definition of working memory.

  - id: travail-2
    sous_rubrique: travail
    question: "Why give an agent a scratchpad, rather than keeping everything in the history?"
    choix:
      - "It looks nicer"
      - "The model doesn't read the history"
      - "The scratchpad costs no tokens"
      - "The useful state stays short and current, even when the history is cut"
    reponse: 3
    pourquoi: >-
      The history grows and ends up summarized or cut.
      The plan may go with it.
      A scratchpad keeps the useful state in one short place.
      You can cut the history without losing where you are.

  - id: episodique-1
    sous_rubrique: episodique
    question: "“On March 3, the user canceled order 48213.” Which memory?"
    choix:
      - "Working memory"
      - "Episodic memory"
      - "Semantic memory"
      - "None"
    reponse: 1
    pourquoi: >-
      It is a dated event.
      It tells what happened, once.
      Episodic memory keeps episodes.

  - id: episodique-2
    sous_rubrique: episodique
    question: "Why compress episodes instead of keeping the raw conversations?"
    choix:
      - "Raw conversations would blow the token budget"
      - "To hide the data"
      - "The model doesn't read conversations"
      - "To translate them"
    reponse: 0
    pourquoi: >-
      A raw conversation is thousands of tokens.
      A hundred conversations fit in no window.
      A dated summary keeps the essentials in a few lines.
      Then you can recall many of them.

  - id: semantique-1
    sous_rubrique: semantique
    question: "“The user is vegetarian.” Which memory?"
    choix:
      - "Working memory"
      - "Episodic memory"
      - "None"
      - "Semantic memory"
    reponse: 3
    pourquoi: >-
      It isn't an event, it is a lasting fact.
      It has no date.
      Semantic memory keeps what is known.

  - id: semantique-2
    sous_rubrique: semantique
    question: "The user ordered fish once. The agent records “likes fish”. The problem?"
    choix:
      - "None"
      - "A hasty generalization from a single episode"
      - "The fact is too short"
      - "It should have been written in English"
    reponse: 1
    pourquoi: >-
      One episode isn't a habit.
      Maybe it was a gift, or an exception.
      A false lasting fact skews every session that follows.
      Keep the link to the episodes that justify each fact.

  - id: semantique-3
    sous_rubrique: semantique
    question: "The user has 500 recorded facts. Which ones go into the context?"
    choix:
      - "All 500"
      - "The most recent"
      - "The ones relevant to the current question"
      - "None"
    reponse: 2
    pourquoi: >-
      Injecting everything is costly and drowns the model.
      The most recent aren't necessarily useful.
      You search for facts close to the question, often by embedding.
      It is search, just as for documents.

  - id: semantique-4
    sous_rubrique: semantique
    question: "MemGPT (2023) compares an agent's memory to…"
    choix:
      - "A human brain"
      - "A web browser"
      - "A SQL database"
      - "An operating system: the context is RAM, external storage is the disk"
    reponse: 3
    pourquoi: >-
      RAM is fast but small. The disk is large but slow.
      An operating system moves data from one to the other.
      MemGPT does the same: the model moves its own memories.
      It calls functions to store or recall them.
---

## Three memories

The model has no memory. We build one for it.

We borrow a distinction from psychology: work, episodes, facts.

| Memory | Holds | Example | Lifetime |
|---|---|---|---|
| Working | The state of the task at hand | “Step 3 of 5: rerun the tests” | The task |
| Episodic | What happened | “March 3, order canceled” | Weeks, months |
| Semantic | What is known | “Vegetarian” | Long, revisable |

:::pourquoi
Each memory answers a different question.
Where am I? What happened? What do I know?
Mixing them gives a heap nobody can sort.
:::

The three feed and recall one another.

```mermaid
flowchart LR
  C["Conversation"] -->|"during the task"| T["Working memory"]
  C -->|"end of session: dated summary"| E["Episodic memory"]
  E -->|"consolidation"| S["Semantic memory"]
  Q["New question"] -->|"search"| S
  Q -->|"search"| E
  S --> A["Context assembler"]
  E --> A
  T --> A
```

## Working memory {#travail}

It is the scratchpad of the task at hand: plan, steps done, decisions.

It lives as long as the task.

:::pourquoi
The history ends up cut or summarized.
The plan may vanish with it.
A short scratchpad keeps the state, whatever happens to the history.
:::

## Episodic memory {#episodique}

It keeps events. Dated, compressed.

“March 3: the user cancels order 48213. Reason: delivery too slow.”

:::pourquoi
A raw conversation is thousands of tokens.
A compressed episode is a few dozen.
You can recall many more.
:::

## Semantic memory {#semantique}

It keeps lasting facts. With no date.

“Vegetarian.” “Prefers short answers.” “Works in Rust.”

These facts come from the episodes. That is **consolidation**.

:::pourquoi
A fact drawn from ten episodes is solid.
A fact drawn from one is a hypothesis.
Keep the link to the episodes that justify it.
:::

You don't inject every fact. You search for the ones that serve the question.

:::pourquoi
Injecting everything is costly and drowns the model.
Recalling a memory is a search, just as for a document.
:::

In 2023, MemGPT takes the idea further. The context is RAM. Storage is the disk. The model stores and recalls on its own, with tools.

:::pourquoi
An operating system already moves data between RAM and disk.
MemGPT gives that role to the model.
It decides what must stay in front of it.
:::

## Key points

- Working: where I am. Episodic: what happened. Semantic: what I know.
- Episodes get compressed. Facts get consolidated.
- A fact must keep its sources.
- Recalling is searching. Only what is relevant goes in.
