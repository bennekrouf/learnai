---
id: "04-04"
titre: "Permissions"
sous_rubriques:
  - id: portee
    titre: "Tool scope"
  - id: sandbox
    titre: "Sandbox"
  - id: confirmation
    titre: "Human confirmation"
sources:
  - titre: "Invariant Labs — GitHub MCP Exploited: Accessing private repositories via MCP, May 26, 2025"
    url: "https://invariantlabs.ai/blog/mcp-github-vulnerability"
  - titre: "Simon Willison — The lethal trifecta for AI agents, June 16, 2025"
    url: "https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/"
  - titre: "MCP — Tools, revision 2026-07-28 (human in the loop, security considerations)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
  - titre: "Fortune — Replit wiped database, July 2025"
    url: "https://fortune.com/2025/07/23/ai-coding-tool-replit-wiped-database-called-it-a-catastrophic-failure/"
qcm:
  - id: portee-1
    sous_rubrique: portee
    question: "May 2025: a booby-trapped issue in a public GitHub repository leaks a private repository through the GitHub MCP server. What condition made it possible?"
    choix:
      - "A flaw in GitHub's encryption"
      - "A JSON-RPC bug"
      - "A weak password"
      - "The agent had a token with access to every repository, public and private"
    reponse: 3
    pourquoi: >-
      The agent reads the public issue, which holds hidden instructions.
      Its token gives it access to the user's private repositories.
      It copies private data into a public pull request.
      Invariant Labs recommends one repository per session and fine-grained rights.

  - id: portee-2
    sous_rubrique: portee
    question: "According to Simon Willison (2025), which trio makes an agent vulnerable to data theft?"
    choix:
      - "Speed, cost, latency"
      - "Access to private data, exposure to untrusted content, ability to communicate externally"
      - "Too many tools, too many tokens, too much memory"
      - "Python, JSON, HTTP"
    reponse: 1
    pourquoi: >-
      Untrusted content brings the malicious instruction.
      Private data is the target.
      External communication is how it gets out.
      Removing just one of the three breaks the attack.

  - id: portee-3
    sous_rubrique: portee
    question: "Your agent only needs to read tickets. Which token do you give it?"
    choix:
      - "A read-only token, limited to the project concerned"
      - "An admin token, to avoid getting blocked"
      - "The developer's personal token"
      - "No token, the model will manage"
    reponse: 0
    pourquoi: >-
      Least privilege limits the possible damage.
      A tricked agent with a read token can't delete anything.
      Limited to one project, it doesn't see the others.
      The rights are enforced by the service, not by the model.

  - id: portee-4
    sous_rubrique: portee
    question: "Is one “run_sql” tool better, or three precise tools such as “read_order”?"
    choix:
      - "run_sql: more flexible"
      - "No difference"
      - "Precise tools: each can only do what it is meant to do"
      - "run_sql: fewer tokens"
    reponse: 2
    pourquoi: >-
      run_sql can also do DROP TABLE.
      read_order can only read an order.
      A tool's flexibility is also an attacker's.
      A precise tool is easy to validate and to monitor.

  - id: sandbox-1
    sous_rubrique: sandbox
    question: "A tool runs code written by the model. Where should it run?"
    choix:
      - "Directly on the developer's machine"
      - "In an isolated environment: container or VM, no secrets, limited network, bounded time and memory"
      - "On the production server"
      - "Anywhere, the model is careful"
    reponse: 1
    pourquoi: >-
      Generated code can be wrong, or driven by an injection.
      Isolated, it sees neither your files nor your keys.
      Without network access, it can't send anything out.
      With limits, an infinite loop doesn't freeze the machine.

  - id: sandbox-2
    sous_rubrique: sandbox
    question: "Replit, July 2025. Which architecture weakness made the deletion possible?"
    choix:
      - "A model that was too small"
      - "A network that was too slow"
      - "A bad JSON schema"
      - "The same database served development, testing and production"
    reponse: 3
    pourquoi: >-
      The agent was working in the development environment.
      But the production database was the same one.
      A development mistake became a production loss.
      Replit then separated the two automatically.

  - id: sandbox-3
    sous_rubrique: sandbox
    question: "You install a stdio MCP server found on GitHub. With which rights does it run?"
    choix:
      - "None, MCP isolates it"
      - "Read-only"
      - "Yours: it can do anything your account can do"
      - "The model's"
    reponse: 2
    pourquoi: >-
      A stdio server is a program launched on your machine.
      MCP doesn't isolate it.
      It can read your files and your keys.
      Installing a server means running its code: check the source.

  - id: confirmation-1
    sous_rubrique: confirmation
    question: "What does the MCP specification recommend for tool calls?"
    choix:
      - "A human in the loop, able to deny a call"
      - "Running them all automatically"
      - "Only logging them"
      - "Limiting them to one per minute"
    reponse: 0
    pourquoi: >-
      The specification says there should always be a human able to deny.
      The application shows which tools are exposed.
      It signals when a tool is called.
      It asks for confirmation on sensitive operations.

  - id: confirmation-2
    sous_rubrique: confirmation
    question: "Should you ask for confirmation on every file read?"
    choix:
      - "Yes, always"
      - "Yes, twice"
      - "No: too many confirmations, and the user ends up accepting everything without reading"
      - "No, never anything"
    reponse: 2
    pourquoi: >-
      A hundred prompts an hour become a reflex.
      The user clicks “yes” without looking.
      The confirmation loses all value.
      Keep it for the irreversible, for sending outside, for money.

  - id: confirmation-3
    sous_rubrique: confirmation
    question: "What should the human see before confirming?"
    choix:
      - "The tool and its exact arguments: the SQL query, the recipient, the amount"
      - "“The agent wants to act. OK?”"
      - "The system prompt"
      - "Nothing, to go fast"
    reponse: 0
    pourquoi: >-
      You can't judge what you can't see.
      “Send an email” doesn't say to whom.
      The specification recommends showing the inputs before the call.
      It is also what reveals a data leak.

  - id: confirmation-4
    sous_rubrique: confirmation
    question: "How do you guarantee that a sensitive action really waits for the human?"
    choix:
      - "Tell the model: “always ask before deleting”"
      - "Trust the model"
      - "Raise the temperature"
      - "Block execution in code until approval is received"
    reponse: 3
    pourquoi: >-
      Replit's freeze was an instruction to the model.
      It was ignored.
      A barrier in code can't be ignored.
      The model can ask. Only code can prevent.
---

## Tool scope {#portee}

Each tool grants a power. Grant only the power needed.

May 2025. Invariant Labs shows an attack on GitHub's official MCP server.

```mermaid
sequenceDiagram
  participant A as Attacker
  participant P as Public repository
  participant G as Agent + GitHub MCP server
  participant V as Private repository
  A->>P: Opens an issue with hidden instructions
  G->>P: Reads the issues
  G->>V: Reads private data (the token allows it)
  G->>P: Publishes that data in a public pull request
```

:::pourquoi
The server had no bug in the classic sense.
The problem: a token that opened every repository, and an agent that read public content.
Invariant recommends one repository per session and fine-grained rights.
:::

Simon Willison names the general pattern the **lethal trifecta**: private data, untrusted content, external communication.

:::pourquoi
Untrusted content carries the order. Private data is the target. External output is how it is stolen.
Remove just one of the three, and the attack fails.
:::

Rules:

- a read-only token if the agent only reads;
- limited to one project, one folder, one repository;
- precise tools rather than one tool that does everything.

:::pourquoi
`run_sql` can also do `DROP TABLE`.
`read_order` can only read an order.
The rights are enforced by the service. The model can't get around them.
:::

## Sandbox {#sandbox}

A tool that runs code or commands runs in a **sandbox**.

| Limit | Why |
|---|---|
| Container or VM | No access to your files |
| No secrets in the environment | Nothing to steal |
| Network off or allowlisted | Nothing to send out |
| Bounded time, memory, disk | An infinite loop blocks nothing |
| Only the working folder writable | No changes elsewhere |

:::pourquoi
Generated code can be wrong.
It can also be dictated by an injection.
Isolation limits the damage in both cases.
:::

Separate development and production.

:::pourquoi
At Replit, the same database served testing and production.
An agent mistake in development wiped production.
:::

A stdio MCP server runs with **your** rights.

:::pourquoi
It is a program launched on your machine.
MCP doesn't isolate it.
Installing it means running its code.
:::

## Human confirmation {#confirmation}

The MCP specification recommends a human able to deny a tool call.

```mermaid
sequenceDiagram
  participant L as LLM
  participant C as Client (barrier)
  participant H as Human
  participant S as MCP server
  L->>C: delete_files(folder=/prod/data)
  C->>H: Exact tool and arguments. Approve?
  H-->>C: Denied
  C-->>L: Denied by the user
  Note over C,S: The call never reaches the server
```

What to confirm:

- the irreversible: deleting, sending, publishing;
- what goes out: an email, a message, a request to the outside;
- money;
- production.

:::pourquoi
Too many confirmations kill confirmation.
The user clicks “yes” by reflex.
Keep it for what matters.
:::

Show the tool and its **exact** arguments.

:::pourquoi
“The agent wants to send an email” doesn't say to whom.
Seeing the recipient reveals a leak.
:::

The barrier is in the **code**, not in the prompt.

:::pourquoi
Replit's freeze was an instruction. It was ignored.
A call blocked in code never reaches the server.
:::

Log every call: who, which tool, which arguments, which decision.

:::pourquoi
Without a trace, an incident can't be understood.
That is the topic of step 6.
:::

## Key points

- Least privilege: read-only, narrow scope, precise tools.
- The lethal trifecta: private data, untrusted content, external output. Remove one.
- Generated code in a sandbox. Development separate from production.
- Human confirmation for the irreversible, enforced by code.
