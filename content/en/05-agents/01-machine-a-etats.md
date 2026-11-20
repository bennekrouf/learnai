---
id: "05-01"
titre: "State machine"
sous_rubriques:
  - id: etats
    titre: "States and transitions"
  - id: boucle
    titre: "The agent loop"
sources:
  - titre: "Anthropic — Building effective agents, December 19, 2024"
    url: "https://www.anthropic.com/engineering/building-effective-agents"
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Anthropic — Project Vend, June 27, 2025"
    url: "https://www.anthropic.com/research/project-vend-1"
qcm:
  - id: etats-1
    sous_rubrique: etats
    question: "Why describe an agent as a state machine?"
    choix:
      - "Because the model requires it"
      - "So it answers faster"
      - "To save tokens"
      - "You always know where it stands, and which actions are allowed right now"
    reponse: 3
    pourquoi: >-
      An explicit state can be read, saved, resumed.
      Each state lists its allowed transitions.
      The code refuses the rest.
      Without a state, you have to guess where the agent is by rereading its whole history.

  - id: etats-2
    sous_rubrique: etats
    question: "A refund agent is in the “awaiting approval” state. The model asks to “refund”. What does the machine do?"
    choix:
      - "It refunds, the model must be right"
      - "It refuses: that transition doesn't exist from this state"
      - "It restarts the agent"
      - "It asks the model if it is sure"
    reponse: 1
    pourquoi: >-
      From “awaiting”, only “approved” and “rejected” exist.
      The model proposes. The machine decides.
      A transition missing from the graph can't happen.
      The code guards the door, not the model's goodwill.

  - id: etats-3
    sous_rubrique: etats
    question: "Who sets the list of allowed transitions?"
    choix:
      - "The code, written in advance. The model picks among the allowed transitions"
      - "The model, at each turn"
      - "The user, in natural language"
      - "Nobody, everything is allowed"
    reponse: 0
    pourquoi: >-
      The graph is the rule of the game.
      The model plays within that frame.
      You can test the graph without a model.
      You can't test a frame the model makes up.

  - id: etats-4
    sous_rubrique: etats
    question: "According to Anthropic (2024), what sets a workflow apart from an agent?"
    choix:
      - "The agent has no tools"
      - "The workflow uses a smaller model"
      - "The workflow follows predefined code paths. The agent directs its own process"
      - "No difference"
    reponse: 2
    pourquoi: >-
      A workflow: the code decides the order of the steps.
      An agent: the model decides the next step and which tools to use.
      A state machine sits between the two.
      The model chooses, but only among allowed paths.

  - id: etats-5
    sous_rubrique: etats
    question: "Which states should you always plan for?"
    choix:
      - "Only “running”"
      - "Final states: done, failed, cancelled"
      - "One state per tool"
      - "None, the agent stops on its own"
    reponse: 1
    pourquoi: >-
      Without a final state, nothing says the task is over.
      “Failed” lets you alert and resume.
      “Cancelled” records a cancellation.
      Every task must end in one of them.

  - id: etats-6
    sous_rubrique: etats
    question: "Claudius sold items below what it paid for them. Where do you put the rule “sale price ≥ cost”?"
    choix:
      - "Nowhere, the model will learn"
      - "In the system prompt, in capitals"
      - "In a Slack message"
      - "In a guard in the code, checked before the “change price” transition"
    reponse: 3
    pourquoi: >-
      An instruction in the prompt can be forgotten.
      A guard in the code applies every time.
      The transition is refused, and the model gets the reason.
      It can then propose another price.

  - id: boucle-1
    sous_rubrique: boucle
    question: "In an agent loop, when does the loop stop?"
    choix:
      - "After exactly one tool call"
      - "Never"
      - "When the model gives a final answer, or a limit is reached"
      - "When the model says “I'm tired”"
    reponse: 2
    pourquoi: >-
      The loop: the model calls a tool, reads the result, starts again.
      It stops when it answers without calling a tool.
      Or when the code cuts it: too many turns, too much cost, too much time.
      You need both conditions.

  - id: boucle-2
    sous_rubrique: boucle
    question: "Why set a maximum number of turns?"
    choix:
      - "To avoid an endless loop, and the bill that comes with it"
      - "To make the model smarter"
      - "Because the API requires it"
      - "To cut the latency of each call"
    reponse: 0
    pourquoi: >-
      A model can go round in circles.
      Each turn costs tokens and time.
      Anthropic recommends stopping conditions, such as a maximum number of iterations.
      Reaching the limit gives the “failed” state, not silence.

  - id: boucle-3
    sous_rubrique: boucle
    question: "The agent calls the same tool five times with the same arguments. What do you do?"
    choix:
      - "Raise the temperature"
      - "Let it go on, it will succeed eventually"
      - "Detect the repetition in code, then stop or escalate"
      - "Wipe its history without a word"
    reponse: 2
    pourquoi: >-
      The same call will give the same result.
      The agent is stuck.
      The code compares the latest calls and spots the loop.
      It can then warn the model, or hand over to a human.

  - id: boucle-4
    sous_rubrique: boucle
    question: "A tool fails. What do you send back to the model?"
    choix:
      - "A clear message about the failure, so it can adapt"
      - "Nothing, the error is hidden"
      - "A made-up result so as not to confuse it"
      - "The full stack trace"
    reponse: 0
    pourquoi: >-
      Anthropic saw it in its research system.
      Telling the agent a tool is failing lets it change strategy.
      A made-up result pushes it to build on something false.
      The stack trace can leak internal details.

  - id: boucle-5
    sous_rubrique: boucle
    question: "On top of the number of turns, which limits should an agent loop have?"
    choix:
      - "A limit on the size of the system prompt"
      - "A character limit per answer only"
      - "No other"
      - "A token or cost budget, and a maximum duration"
    reponse: 3
    pourquoi: >-
      Ten turns with huge results can cost a lot.
      A slow tool can make one turn last hours.
      Three counters: turns, cost, time.
      The first one reached stops the loop.
---

## States and transitions {#etats}

An agent in production has a **state**. A precise name, saved, readable.

```mermaid
stateDiagram-v2
  [*] --> Received
  Received --> Analysis
  Analysis --> AwaitingApproval : amount > threshold
  Analysis --> Refunded : amount ≤ threshold
  AwaitingApproval --> Refunded : approved
  AwaitingApproval --> Rejected : rejected
  Analysis --> Failed : error
  Refunded --> [*]
  Rejected --> [*]
  Failed --> [*]
```

Each state lists its allowed **transitions**. The rest is refused.

:::pourquoi
You always know where the agent stands.
You know what it is allowed to do now.
An action outside the graph can't happen.
:::

The **code** sets the graph. The **model** picks among the allowed transitions.

:::pourquoi
The model proposes. The machine decides.
The graph can be tested without a model.
:::

Anthropic tells two forms apart. A **workflow** follows “predefined code paths”. An **agent** “dynamically” directs its own process and tool usage.

:::pourquoi
The state machine takes the best of both.
The model keeps the initiative.
The code keeps the limits.
:::

Business rules go in **guards**: conditions checked before a transition.

:::pourquoi
Claudius sold below what it paid.
A “price ≥ cost” guard would have refused that change.
An instruction in the prompt can be forgotten.
:::

Always plan **final states**: done, failed, cancelled.

:::pourquoi
Without a final state, you can't tell whether a task is over.
“Failed” triggers an alert or a retry.
:::

## The agent loop {#boucle}

The heart of an agent is a loop.

```mermaid
sequenceDiagram
  participant O as Orchestrator (code)
  participant M as Model
  participant T as Tool
  loop While no limit is reached
    O->>M: Context + previous results
    M-->>O: Tool call requested
    O->>O: Checks the state and the guards
    O->>T: Runs the call
    T-->>O: Result or error
  end
  M-->>O: Final answer
  O->>O: Final state: done
```

It stops in two cases: the model answers without calling a tool, or a **limit** is reached.

:::pourquoi
The model can go round in circles.
Anthropic recommends stopping conditions, such as a maximum number of iterations.
:::

Three limits:

| Limit | Prevents |
|---|---|
| Number of turns | The endless loop |
| Token or cost budget | The surprise bill |
| Maximum duration | The task stuck for hours |

:::pourquoi
Each limit covers a different risk.
The first one reached stops the loop, with the “failed” state.
:::

Detect repetitions: same tool, same arguments, several times.

:::pourquoi
The same call gives the same result.
The agent is stuck. Better to say so than to pay for ten more turns.
:::

When a tool fails, **tell** the model. Clearly.

:::pourquoi
Anthropic saw it: an agent that knows a tool is failing changes strategy.
One that has the error hidden from it builds on nothing.
:::

## Key points

- An explicit state, listed transitions, final states.
- The code sets the graph and the guards. The model chooses within that frame.
- The loop stops on a final answer or on a limit: turns, cost, time.
- Tool failures are told to the model, not hidden.
