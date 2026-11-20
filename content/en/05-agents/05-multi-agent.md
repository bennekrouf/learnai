---
id: "05-05"
titre: "Multi-agent"
sous_rubriques:
  - id: topologies
    titre: "Topologies"
  - id: superviseur
    titre: "Supervisor"
  - id: etat-type
    titre: "Typed state"
  - id: conflits
    titre: "Conflicts"
  - id: un-seul
    titre: "When one agent is enough"
sources:
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Cognition (Walden Yan) — Don't Build Multi-Agents, June 12, 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "Anthropic — Building effective agents, December 19, 2024"
    url: "https://www.anthropic.com/engineering/building-effective-agents"
  - titre: "Cemri et al. — Why Do Multi-Agent LLM Systems Fail? (MAST), 2025"
    url: "https://arxiv.org/abs/2503.13657"
qcm:
  - id: topologies-1
    sous_rubrique: topologies
    question: "In the “orchestrator and workers” topology, what does the orchestrator do?"
    choix:
      - "It only relays messages"
      - "It runs everything itself"
      - "It votes with the other agents"
      - "It splits the task, delegates the pieces to subagents, then assembles their results"
    reponse: 3
    pourquoi: >-
      The split isn't known in advance.
      The orchestrator decides it based on the task.
      The subagents each work on one piece.
      The orchestrator gathers and concludes.

  - id: topologies-2
    sous_rubrique: topologies
    question: "Anthropic describes two forms of parallelization. Which ones?"
    choix:
      - "Several GPUs, or several data centers"
      - "Splitting into independent subtasks, or running the same task several times and voting"
      - "Several languages, or several system prompts"
      - "Synchronous, or asynchronous"
    reponse: 1
    pourquoi: >-
      Splitting: each agent handles a different part.
      Voting: several agents handle the same thing.
      The first is faster.
      The second gives more confidence in the answer.

  - id: topologies-3
    sous_rubrique: topologies
    question: "Anthropic's multi-agent research system against a single agent (Opus 4), on their internal evaluation?"
    choix:
      - "About 90% better"
      - "The same"
      - "Worse"
      - "10 times better"
    reponse: 0
    pourquoi: >-
      An Opus 4 orchestrator, with Sonnet 4 subagents.
      It beats Opus 4 alone by 90.2% on their research evaluation.
      Above all on broad questions that can be explored in parallel.

  - id: topologies-4
    sous_rubrique: topologies
    question: "According to Anthropic, how many tokens does a multi-agent system use compared with a plain chat?"
    choix:
      - "Half as many"
      - "As many"
      - "About 15 times more"
      - "About 2 times more"
    reponse: 2
    pourquoi: >-
      A single agent uses about 4 times more than a chat.
      A multi-agent system, about 15 times more.
      The task must be worth that price.

  - id: superviseur-1
    sous_rubrique: superviseur
    question: "What should a subagent get from the supervisor?"
    choix:
      - "A three-word sentence"
      - "An objective, an output format, the tools to use, and the boundaries of its task"
      - "The whole history of every agent"
      - "Nothing, it guesses"
    reponse: 1
    pourquoi: >-
      A vague instruction gives duplicated or off-target work.
      The objective says what to look for.
      The format says what to return.
      The boundaries say where to stop, and what the others are doing.

  - id: superviseur-2
    sous_rubrique: superviseur
    question: "Who decides the overall task is finished?"
    choix:
      - "Nobody"
      - "The first subagent to finish"
      - "The last subagent to finish"
      - "The supervisor, with its stopping conditions"
    reponse: 3
    pourquoi: >-
      Each subagent only sees one piece.
      Only the supervisor sees the whole.
      It judges whether the answer is complete, or whether to go again.
      It has its turn and cost limits, like any agent.

  - id: superviseur-3
    sous_rubrique: superviseur
    question: "In Anthropic's system, which agent adds the citations at the end?"
    choix:
      - "The user"
      - "Each subagent, at random"
      - "An agent dedicated to citations, which ties each claim to its source"
      - "None, there are no citations"
    reponse: 2
    pourquoi: >-
      The report is written first.
      A dedicated agent rereads the documents and the report.
      It attaches each claim to its source.
      A precise task, given to a precise agent.

  - id: etat-type-1
    sous_rubrique: etat-type
    question: "Why share a typed state between agents, rather than free text?"
    choix:
      - "Each field has a type and an owner. What comes in is validated"
      - "Free text costs more"
      - "Models can't read text"
      - "It looks nicer"
    reponse: 0
    pourquoi: >-
      A free-text summary can be read ten ways.
      A “sources: list of URLs” field can be checked.
      You know who writes what.
      You reject a malformed output before it contaminates the others.

  - id: etat-type-2
    sous_rubrique: etat-type
    question: "Two subagents write to the same state field at the same time. What do you plan for?"
    choix:
      - "Forbid parallelism"
      - "The last writer wins, silently"
      - "A merge rule for that field, or a single agent allowed to write it"
      - "Let the model choose afterwards"
    reponse: 2
    pourquoi: >-
      “Last writer wins” silently wipes out some work.
      A list can be concatenated.
      A status must have a single owner.
      The rule is decided in advance, not discovered in production.

  - id: etat-type-3
    sous_rubrique: etat-type
    question: "When do you validate a subagent's output?"
    choix:
      - "Before merging it into the shared state"
      - "Never"
      - "Only at the very end"
      - "Only if it is long"
    reponse: 0
    pourquoi: >-
      An invalid output that gets into the state contaminates the other agents.
      Finding it at the end means hunting for where it came from.
      At the boundary, you know exactly which agent produced it.
      It can be rerun with the error.

  - id: conflits-1
    sous_rubrique: conflits
    question: "Cognition (2025): one subagent draws a Mario-style background, the other a bird in another style. Which principle did they forget?"
    choix:
      - "Raise the temperature"
      - "Use a bigger model"
      - "Add more subagents"
      - "Share the full context: every action carries implicit decisions"
    reponse: 3
    pourquoi: >-
      The task: a Flappy Bird clone.
      Each subagent only got its own piece.
      Each made style choices without seeing the other's.
      The pieces no longer fit together.

  - id: conflits-2
    sous_rubrique: conflits
    question: "Two subagents report contradictory figures. What does the supervisor do?"
    choix:
      - "It keeps the first to arrive"
      - "It compares the sources, and decides or flags the disagreement"
      - "It takes the average"
      - "It hides the disagreement"
    reponse: 1
    pourquoi: >-
      The average of two wrong figures is still wrong.
      The sources say which one is reliable.
      If you can't decide, you say so.
      A visible disagreement beats a made-up certainty.

  - id: conflits-3
    sous_rubrique: conflits
    question: "For which kind of question does a majority vote between several agents work well?"
    choix:
      - "Any kind"
      - "A long written report"
      - "A graphic design"
      - "A short, comparable answer: a number, a label, yes or no"
    reponse: 3
    pourquoi: >-
      To vote, you have to count identical answers.
      Three different reports can't be counted.
      “42”, “42”, “41” can.
      The vote cuts down isolated mistakes.

  - id: conflits-4
    sous_rubrique: conflits
    question: "The MAST study (2025) classifies the failures of multi-agent systems. Into which categories?"
    choix:
      - "Network, disk, memory"
      - "System design, misalignment between agents, task verification"
      - "Prompt, temperature, model"
      - "Cost, latency, security"
    reponse: 1
    pourquoi: >-
      14 failure modes, grouped into 3 categories.
      Built from traces of 7 multi-agent frameworks.
      Most don't come from the model alone.
      They come from how the agents are organized.

  - id: un-seul-1
    sous_rubrique: un-seul
    question: "What does Cognition recommend instead of a multi-agent system?"
    choix:
      - "A single, linear agent, compressing the context when it gets long"
      - "Ten agents that vote"
      - "No agent at all"
      - "One agent per file"
    reponse: 0
    pourquoi: >-
      A single thread of execution sees all its decisions.
      Nothing is lost between two agents.
      When the context grows, you summarize it.
      It is simpler and more reliable for many tasks.

  - id: un-seul-2
    sous_rubrique: un-seul
    question: "According to Anthropic, which tasks is multi-agent a poor fit for?"
    choix:
      - "Questions with several independent leads"
      - "Broad research in parallel"
      - "Those where every agent must share the same context, and most coding"
      - "None"
    reponse: 2
    pourquoi: >-
      Code is highly interdependent.
      A decision in one file constrains ten others.
      Subagents don't see those links.
      Multi-agent shines when the leads are independent.

  - id: un-seul-3
    sous_rubrique: un-seul
    question: "Where should you start, according to “Building effective agents”?"
    choix:
      - "The most complete multi-agent system"
      - "The simplest solution. Add complexity only if it measurably improves things"
      - "A framework, above all"
      - "Ten subagents by default"
    reponse: 1
    pourquoi: >-
      Each added agent costs tokens, latency, possible failures.
      One well-designed call is often enough.
      Then a workflow. Then an agent.
      Multi-agent comes last, if it is justified.

  - id: un-seul-4
    sous_rubrique: un-seul
    question: "Anthropic measured that token usage explains 80% of the performance variance on BrowseComp. What do you conclude?"
    choix:
      - "Multi-agent is free"
      - "Tokens don't matter"
      - "Always use the smallest model"
      - "A good share of the multi-agent gain comes from spending more tokens"
    reponse: 3
    pourquoi: >-
      Several agents explore more, so they spend more.
      That spending explains most of the gain.
      The question becomes: is the task worth the cost?
      For a simple question, no.
---

## Topologies {#topologies}

Several agents can work together. Three forms come up often.

| Topology | Principle | When |
|---|---|---|
| Orchestrator and workers | One agent splits, delegates, assembles | Split not known in advance |
| Parallelization by sectioning | Each agent handles one part | Independent parts |
| Parallelization by voting | Several agents, same task | Need for confidence |

:::pourquoi
Anthropic describes these patterns in “Building effective agents”.
Each meets a different need: speed, coverage, reliability.
:::

Anthropic built a research system as orchestrator and workers. An Opus 4 leads Sonnet 4 subagents. Result: +90.2% over a single Opus 4, on their internal evaluation.

:::pourquoi
Broad research questions can be explored in parallel.
Each subagent digs into one lead with its own context.
:::

The price: about **15 times** the tokens of a chat. A single agent: about 4 times.

:::pourquoi
Each subagent has its own context, its calls, its results.
The task must be worth that cost.
:::

## Supervisor {#superviseur}

```mermaid
sequenceDiagram
  participant U as User
  participant L as Supervisor
  participant A as Subagent A
  participant B as Subagent B
  participant C as Citation agent
  U->>L: Research question
  L->>L: Plan, saved
  L->>A: Objective, format, tools, boundaries
  L->>B: Objective, format, tools, boundaries
  A-->>L: Results and sources
  B-->>L: Results and sources
  L->>L: Enough? If not, go again
  L->>C: Report and documents
  C-->>L: Report with citations
  L-->>U: Final answer
```

The supervisor **splits**, **delegates**, **gathers**, **decides** when it is over.

:::pourquoi
Each subagent only sees one piece.
Only the supervisor sees the whole.
:::

Each delegation spells out: the objective, the output format, the tools, the boundaries.

:::pourquoi
A vague instruction gives duplicated or off-target work.
The boundaries also say what the others are doing.
:::

A precise task can have its own precise agent. At Anthropic, a dedicated agent adds the citations at the end.

:::pourquoi
Tying each claim to its source is a job of its own.
A specialized agent does it better than in passing.
:::

## Typed state {#etat-type}

The agents share a **typed state**: named fields, each with a type and an owner.

```json
{
  "question": "string",
  "leads": [{ "id": "string", "agent": "string", "status": "todo | running | done" }],
  "sources": [{ "url": "string", "excerpt": "string", "lead": "string" }],
  "report": "string | null"
}
```

:::pourquoi
A free-text summary can be read ten ways.
A typed field can be validated.
:::

Each field has a write rule: a single owner, or a defined merge.

:::pourquoi
“Last writer wins” wipes out work without a sound.
A list of sources is concatenated. A status has one master.
:::

A subagent's output is validated **before** it enters the state.

:::pourquoi
At the boundary, you know which agent produced the error.
After the merge, it contaminates everything.
:::

## Conflicts {#conflits}

Cognition tells an example. Task: a Flappy Bird clone. One subagent makes a Mario-style background. The other, a bird that doesn't match.

:::pourquoi
“Actions carry implicit decisions”: a style, an assumption.
An agent that doesn't see the other's choices decides differently.
Cognition draws a principle from it: share the full context.
:::

Two results contradict each other? The supervisor compares the sources. It decides, or it flags the disagreement.

:::pourquoi
The average of two wrong figures is still wrong.
A visible disagreement beats a made-up certainty.
:::

For a short, comparable answer, a **vote** works well: several agents, the majority wins.

:::pourquoi
“42”, “42”, “41” can be counted.
Three written reports can't.
:::

The MAST study (2025) classifies 14 failure modes into 3 categories: system design, misalignment between agents, task verification.

:::pourquoi
Most failures don't come from the model alone.
They come from how the agents are organized.
:::

## When one agent is enough {#un-seul}

Often.

Anthropic says it: multi-agent is a poor fit for tasks where everyone must share the same context, and for most coding.

:::pourquoi
In code, one decision constrains ten others.
Isolated subagents don't see those links.
:::

Cognition recommends a single, linear agent. When the context gets long, you compress it.

:::pourquoi
A single thread sees all its decisions.
Nothing is lost between two agents.
:::

On BrowseComp, token usage alone explains 80% of the performance variance Anthropic measured.

:::pourquoi
A good share of the gain comes from the spending.
The real question: is the task worth the cost?
:::

```mermaid
flowchart TD
  Q["New task"] --> S{"Is one well-designed call enough?"}
  S -->|"yes"| A1["One call"]
  S -->|"no"| W{"Steps known in advance?"}
  W -->|"yes"| A2["Workflow"]
  W -->|"no"| P{"Independent leads, high value, too big for one context?"}
  P -->|"no"| A3["A single agent"]
  P -->|"yes"| A4["Multi-agent"]
```

:::pourquoi
Anthropic advises starting with the simplest solution.
You add complexity only if it measurably improves the result.
:::

## Key points

- Orchestrator and workers, sectioning, voting: each topology has its use.
- The supervisor delegates with an objective, a format, tools, boundaries. It decides when it is over.
- Typed state, one owner per field, validation at the boundary.
- Conflicts come from unshared context. Compare sources, vote on what is comparable.
- One agent is often enough. Multi-agent costs about 15 times a chat.
