---
id: "01-04"
titre: "Memory lifecycle"
sous_rubriques:
  - id: importance
    titre: "Importance score"
  - id: ttl
    titre: "TTL and eviction"
  - id: persistance
    titre: "Persistence across sessions"
sources:
  - titre: "Park et al. — Generative Agents: Interactive Simulacra of Human Behavior, 2023"
    url: "https://arxiv.org/abs/2304.03442"
  - titre: "The Hacker News — ChatGPT macOS flaw could've enabled long-term spyware via memory function (SpAIware, Johann Rehberger), September 2024"
    url: "https://thehackernews.com/2024/09/chatgpt-macos-flaw-couldve-enabled-long.html"
qcm:
  - id: importance-1
    sous_rubrique: importance
    question: "In Generative Agents (Stanford, 2023), who rates a memory's importance?"
    choix:
      - "The LLM itself, on a scale of 1 to 10"
      - "A human"
      - "The memory's word count"
      - "The memory's time of day"
    reponse: 0
    pourquoi: >-
      Each memory is rated by the model when it is created.
      1: mundane, like brushing teeth.
      10: poignant, like a breakup or a college acceptance.
      The model judges the meaning. A fixed rule couldn't.

  - id: importance-2
    sous_rubrique: importance
    question: "In Generative Agents, which three criteria decide which memories are recalled?"
    choix:
      - "Length, language, date"
      - "Popularity, author, source"
      - "Recency, importance, relevance"
      - "Cost, latency, size"
    reponse: 2
    pourquoi: >-
      Recency: a recently used memory comes back more easily.
      Importance: a striking event weighs more.
      Relevance: it must be about the current situation.
      The three scores are normalized, then added up.

  - id: importance-3
    sous_rubrique: importance
    question: "Why not recall only the most relevant memories?"
    choix:
      - "Relevance can't be computed"
      - "A relevant but old and mundane memory can crowd out a recent, important event"
      - "Relevance costs too much"
      - "Models don't like relevance"
    reponse: 1
    pourquoi: >-
      Relevance alone ignores time and weight.
      “He had a coffee on Monday” may seem close to the question.
      “He quit yesterday” matters far more.
      Combining the three criteria avoids that trap.

  - id: ttl-1
    sous_rubrique: ttl
    question: "A TTL on a memory is…"
    choix:
      - "Its size in tokens"
      - "Its importance score"
      - "How long it lives before it expires"
      - "Its ID"
    reponse: 2
    pourquoi: >-
      TTL stands for time to live.
      Past that time, the memory expires.
      Without expiry, the memory grows forever.
      And old facts that became false stay in place.

  - id: ttl-2
    sous_rubrique: ttl
    question: "Which memory should have the shortest TTL?"
    choix:
      - "Working memory"
      - "Episodic memory"
      - "Semantic memory"
      - "All the same"
    reponse: 0
    pourquoi: >-
      Working memory only serves during the task.
      It expires when the task ends.
      Episodes last weeks or months.
      Facts last a long time, but stay revisable.

  - id: ttl-3
    sous_rubrique: ttl
    question: "The user lived in Lyon. They say they have moved to Geneva. What do you do?"
    choix:
      - "Add “lives in Geneva” next to “lives in Lyon”"
      - "Delete the whole memory"
      - "Ignore it, the memory is fixed"
      - "Replace the old fact with the new one"
    reponse: 3
    pourquoi: >-
      Two contradictory facts sow doubt.
      The model may pick the wrong one.
      Update the fact, and keep the episode of the move.
      The history stays, the current fact is unique.

  - id: ttl-4
    sous_rubrique: ttl
    question: "The memory is full. What goes first?"
    choix:
      - "The most recent"
      - "The memories with the lowest combined score"
      - "At random"
      - "The most important"
    reponse: 1
    pourquoi: >-
      Remove what is least likely to be useful.
      The combined score measures it: old, mundane, rarely recalled.
      That is eviction.

  - id: persistance-1
    sous_rubrique: persistance
    question: "September 2024: a researcher shows an attack on ChatGPT's memory. How does it work?"
    choix:
      - "A booby-trapped page gets an instruction saved to memory, replayed in every session"
      - "He steals the user's password"
      - "He erases the memory remotely"
      - "He fills the memory until it is full"
    reponse: 0
    pourquoi: >-
      The user has a booby-trapped page or document read.
      Hidden text pushes ChatGPT to save an instruction.
      It comes back in every conversation that follows.
      Johann Rehberger called it SpAIware.

  - id: persistance-2
    sous_rubrique: persistance
    question: "Which rule protects persistent memory?"
    choix:
      - "Everything the model proposes to save is saved"
      - "The memory is erased every day"
      - "The memory is encrypted, so it is safe"
      - "What comes from an untrusted source doesn't enter memory unchecked"
    reponse: 3
    pourquoi: >-
      Memory turns a one-off attack into a permanent one.
      Encryption protects the storage, not the content.
      What comes from a page, an email or a tool is data.
      Before it becomes a memory, it needs a check, or the user's consent.

  - id: persistance-3
    sous_rubrique: persistance
    question: "Two users use the same agent. What must you guarantee?"
    choix:
      - "A shared memory, to learn better"
      - "A memory isolated per user"
      - "No memory"
      - "One memory per day"
    reponse: 1
    pourquoi: >-
      One user's memory must never show up for another.
      That would be a personal data leak.
      Each memory carries its owner's ID.
      Each search filters on that ID.

  - id: persistance-4
    sous_rubrique: persistance
    question: "A European user asks the agent to forget everything about them. What do you do?"
    choix:
      - "Refuse, the memory is technical"
      - "Hide their memories from display"
      - "Erase their memories at every tier and in every copy"
      - "Wait for the TTL to expire"
    reponse: 2
    pourquoi: >-
      The GDPR grants a right to erasure.
      Memories about a person are personal data.
      Hiding isn't enough. Neither is waiting.
      Plan for erasure from the design stage, backups included.
---

## Importance score {#importance}

Not everything deserves remembering.

In 2023, Stanford builds *Generative Agents*: 25 simulated characters in a small village. Each one has a memory.

Each memory gets an importance rating, from 1 to 10. The model does the rating.

:::pourquoi
A fixed rule doesn't know a breakup matters more than a coffee.
The model understands the meaning.
One rating when the memory is saved, and it is settled for good.
:::

To recall a memory, they combine three scores.

| Score | Question | Computation |
|---|---|---|
| Recency | Used recently? | Decays by 0.5% per game hour |
| Importance | Striking? | The model's rating, from 1 to 10 |
| Relevance | Related to the situation? | Embedding similarity |

:::pourquoi
Relevance alone would bring back last Monday's coffee.
Recency alone would forget the big events.
The three together imitate human memory fairly well.
:::

## TTL and eviction {#ttl}

A memory that only grows becomes a problem.

It costs. It slows the search. It keeps facts that became false.

The **TTL** gives each memory a lifetime.

| Memory | Typical TTL |
|---|---|
| Working | End of the task |
| Episodic | Weeks or months |
| Semantic | Long, but revisable |

:::pourquoi
A task plan is worthless once the task is done.
An episode loses value over time.
A fact stays true until it is contradicted.
:::

A contradicted fact is **replaced**. Not duplicated.

:::pourquoi
“Lives in Lyon” and “lives in Geneva” side by side: the model may pick the wrong one.
Update the fact. The episode of the move keeps the history.
:::

When space runs short, **evict** the memories with the lowest combined score.

## Persistence across sessions {#persistance}

A persistent memory outlives the end of the session.

It is stored outside the model: a database, files.

```mermaid
sequenceDiagram
  participant U as User
  participant A as Application
  participant M as Memory
  participant L as LLM
  U->>A: Opens a session
  A->>M: Useful memories for this user?
  M-->>A: Relevant facts and episodes
  A->>L: Context + memories + question
  L-->>A: Answer + facts to remember
  A->>A: Checks the proposed facts
  A->>M: Saves the approved facts
```

:::pourquoi
The model proposes. The application decides what enters memory.
A false memory comes back in every session.
:::

In September 2024, Johann Rehberger shows the *SpAIware* attack on ChatGPT. A booby-trapped page gets an instruction saved to memory. It comes back in every conversation that follows.

:::pourquoi
Memory makes an attack permanent.
What comes from a page, an email or a tool is data.
It doesn't enter memory unchecked.
:::

One memory per user. Always isolated.

:::pourquoi
A memory that leaks to someone else is a personal data leak.
Each search filters by owner.
:::

Erasure must be possible, everywhere.

:::pourquoi
The GDPR grants a right to erasure.
Memories about a person are personal data.
:::

## Key points

- Recall by three criteria: recency, importance, relevance.
- Each memory has its lifetime. A contradicted fact is replaced.
- The model proposes memories. The application checks them.
- Memory isolated per user, and erasable.
