---
id: "09-02"
titre: "Diagrams"
sous_rubriques:
  - id: flux-donnees
    titre: "Data flow"
  - id: topologie
    titre: "Agent topology"
sources:
  - titre: "Simon Brown — The C4 model for visualising software architecture"
    url: "https://c4model.com/"
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, June 12, 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "Anthropic — How we built our multi-agent research system, June 13, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
qcm:
  - id: flux-donnees-1
    sous_rubrique: flux-donnees
    question: "What does a data flow diagram show?"
    choix:
      - "The project schedule"
      - "The team's org chart"
      - "The classes in the code"
      - "Which data goes between which components, and where it is stored"
    reponse: 3
    pourquoi: >-
      It follows the data, not the code.
      From the user to the model, to the database, to the logs.
      That is where you see the possible leaks.
      And what must be protected.

  - id: flux-donnees-2
    sous_rubrique: flux-donnees
    question: "What is a trust boundary on a diagram?"
    choix:
      - "The edge of the page"
      - "The line where data leaves your control, for example toward a model provider's API"
      - "The split between two files"
      - "A decorative color"
    reponse: 1
    pourquoi: >-
      Beyond the boundary, other rules apply.
      An external provider, a third-party tool, a shared log.
      Each crossing is a question: what goes through, and should it be masked?
      That is the link with step 6.

  - id: flux-donnees-3
    sous_rubrique: flux-donnees
    question: "What do you write on a data flow arrow?"
    choix:
      - "What goes through: “question + 5 excerpts”, “masked card number”"
      - "Nothing, the arrow is enough"
      - "The developer's name"
      - "The component's color"
    reponse: 0
    pourquoi: >-
      An unlabeled arrow only says “these talk”.
      A labeled arrow says what, and so what to protect.
      The reviewer immediately sees personal data going out.

  - id: flux-donnees-4
    sous_rubrique: flux-donnees
    question: "What does Simon Brown's C4 model propose?"
    choix:
      - "A single diagram for everything"
      - "Four mandatory colors"
      - "Zoom levels: context, containers, components, code"
      - "A programming language"
    reponse: 2
    pourquoi: >-
      Context: the system and its users.
      Containers: applications and databases deployed separately.
      Components: the building blocks inside a container.
      You pick the zoom level for the reader.

  - id: flux-donnees-5
    sous_rubrique: flux-donnees
    question: "Why write your diagrams as text, with Mermaid for example?"
    choix:
      - "They look nicer than a drawing"
      - "They live in the repository, are reviewed in PRs and are updated with the code"
      - "They can't become wrong"
      - "They replace tests"
    reponse: 1
    pourquoi: >-
      An exported image goes stale silently.
      A text diagram is changed in the same PR as the code.
      The diff shows what changed.
      It can still become wrong: it has to be reviewed.

  - id: flux-donnees-6
    sous_rubrique: flux-donnees
    question: "Which data destination is often forgotten on an agent diagram?"
    choix:
      - "The main database"
      - "The model"
      - "The user"
      - "The logs and traces, which hold prompts and answers"
    reponse: 3
    pourquoi: >-
      Traces keep everything: questions, documents, answers.
      They often go to an external observability tool.
      It is a full copy of the data, outside the database.
      It is protected like the database.

  - id: topologie-1
    sous_rubrique: topologie
    question: "What does an agent topology diagram show?"
    choix:
      - "The price of each call"
      - "The size of the models"
      - "Who calls whom, in series or in parallel, and what context each one receives"
      - "The list of developers"
    reponse: 2
    pourquoi: >-
      The topology decides cost, latency and inconsistencies.
      The diagram makes those choices visible.
      The context each agent receives is the most forgotten point.

  - id: topologie-2
    sous_rubrique: topologie
    question: "According to Cognition, what should be passed to a subagent?"
    choix:
      - "The full context, including the other agents' traces, not only the text of the task"
      - "Only the title of the task"
      - "Nothing, it must guess"
      - "Only the expected result"
    reponse: 0
    pourquoi: >-
      In the Flappy Bird example, each subagent had its task.
      It didn't see the other's work.
      The implicit choices contradicted each other.
      The diagram must show that context being passed.

  - id: topologie-3
    sous_rubrique: topologie
    question: "How do you draw subagents working in parallel?"
    choix:
      - "Boxes with no arrows"
      - "A single straight line"
      - "Branches leaving the same lead agent, and coming back to it"
      - "You don't draw them"
    reponse: 2
    pourquoi: >-
      Parallelism shows in the shape.
      Branches going out and coming back: you see who synthesizes.
      A straight line would say the opposite: a sequence of steps.

  - id: topologie-4
    sous_rubrique: topologie
    question: "What information do you add to an agent topology to make it useful in a review?"
    choix:
      - "Each agent's tools, and where the state is stored"
      - "The agents' avatars"
      - "The team's favorite colors"
      - "Nothing more"
    reponse: 0
    pourquoi: >-
      The tools say what each agent can do: least privilege.
      The stored state says what survives an outage: durability.
      A reviewer checks steps 5 and 6 at a glance.

  - id: topologie-5
    sous_rubrique: topologie
    question: "A diagram tries to show everything: flow, topology, deployment, costs. Problem?"
    choix:
      - "Mermaid refuses"
      - "None, the more the better"
      - "It will be too small"
      - "It becomes unreadable: a diagram answers one question"
    reponse: 3
    pourquoi: >-
      The reader is looking for a precise answer.
      Too much information hides it.
      Three clear diagrams beat a single confusing one.
      Each with its title and its question.
---

## Data flow {#flux-donnees}

The **data flow** follows the data, not the code: where it comes from, where it goes, where it stays.

```mermaid
flowchart LR
  U[User] -->|question| A[Application]
  A -->|masked question| R[(Vector index)]
  R -->|5 excerpts| A
  subgraph External [Outside our control]
    M[Model API]
    O[Observability tool]
  end
  A -->|masked question + 5 excerpts| M
  M -->|answer| A
  A -->|full trace| O
  A -->|answer| U
  A -->|audit log| J[(Log)]
```

:::pourquoi
You see at a glance what leaves your premises.
Each arrow that crosses the boundary is a security question.
:::

Three rules:

1. Each arrow carries a **label**: what goes through.
2. **Trust boundaries** are drawn.
3. **Copies** are visible: logs, traces, caches.

:::pourquoi
An unlabeled arrow says “these talk”, not “what”.
The boundary shows where the rules change.
Traces are a full copy of the data, often forgotten.
:::

Simon Brown's **C4 model** offers zoom levels:

| Level | Shows | For whom |
|---|---|---|
| Context | The system, its users, its neighbors | Everyone |
| Containers | Applications and databases deployed separately | Technical team |
| Components | The building blocks inside a container | Developers |
| Code | The detail of a component (optional) | Rarely useful |

:::pourquoi
An executive doesn't need the components.
A developer needs more than the context.
You pick the zoom level for the reader.
:::

Diagrams are written as **text** (Mermaid) and live **in the repository**.

:::pourquoi
They are changed in the same PR as the code.
The diff shows what changed.
An exported image goes stale silently.
:::

## Agent topology {#topologie}

The **topology** shows who calls whom, in series or in parallel, with what context.

```mermaid
flowchart TB
  U[User] --> P[Lead agent]
  P -->|lead 1 + full plan| S1[Subagent 1]
  P -->|lead 2 + full plan| S2[Subagent 2]
  P -->|lead 3 + full plan| S3[Subagent 3]
  S1 -->|summary + sources| P
  S2 -->|summary + sources| P
  S3 -->|summary + sources| P
  P --> E[(Saved state)]
  S1 -.->|web search| W[Tools]
  S2 -.->|web search| W
  S3 -.->|web search| W
  P -->|answer| U
```

:::pourquoi
The shape shows the parallelism: branches going out and coming back.
You see who synthesizes, and what each one receives.
:::

The most important label: the **context passed on**.

:::pourquoi
Cognition: the Flappy Bird subagents didn't see each other's work.
Their implicit choices contradicted each other.
An arrow saying “task only” announces that risk.
:::

Add each agent's **tools** and where the **state** is stored.

:::pourquoi
The tools show the rights: least privilege, step 6.
The state shows what survives an outage: durability, step 5.
:::

A diagram answers **one question**.

:::pourquoi
Flow, topology, deployment, costs on a single chart: nobody reads it.
Three clear diagrams beat a single confusing one.
:::

## Key points

- Data flow: labels on arrows, trust boundaries, visible copies.
- C4: pick the zoom level for the reader.
- Topology: who calls whom, in parallel or not, with what context.
- Diagrams as text, in the repository. One diagram, one question.
