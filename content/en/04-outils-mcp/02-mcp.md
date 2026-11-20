---
id: "04-02"
titre: "MCP"
sous_rubriques:
  - id: versions
    titre: "Versions: initialize, then stateless"
  - id: outils
    titre: "tools/list and tools/call"
  - id: ressources
    titre: "Resources and prompts"
  - id: transports
    titre: "stdio and HTTP"
sources:
  - titre: "MCP — Lifecycle, revision 2025-11-25 (the initialize handshake)"
    url: "https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle"
  - titre: "MCP — Overview and Versioning, revision 2026-07-28 (stateless protocol)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/index"
  - titre: "MCP — Tools, revision 2026-07-28"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
  - titre: "MCP — stdio transport, revision 2026-07-28"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio"
  - titre: "MCP — Transports, revision 2025-06-18 (Streamable HTTP and security warnings)"
    url: "https://modelcontextprotocol.io/specification/2025-06-18/basic/transports"
  - titre: "MCP Blog — MCP joins the Agentic AI Foundation, December 9, 2025"
    url: "https://blog.modelcontextprotocol.io/posts/2025-12-09-mcp-joins-agentic-ai-foundation/"
qcm:
  - id: versions-1
    sous_rubrique: versions
    question: "Which problem does MCP solve first?"
    choix:
      - "Every application had to code an integration for every tool: N × M connectors"
      - "Model speed"
      - "Token prices"
      - "Translating prompts"
    reponse: 0
    pourquoi: >-
      10 applications and 20 tools: 200 custom integrations.
      With a common protocol, each tool writes one server.
      Each application writes one client.
      10 + 20 = 30 pieces, which all work together.

  - id: versions-2
    sous_rubrique: versions
    question: "In versions up to 2025-11-25, what is the first thing an MCP client sends?"
    choix:
      - "tools/call"
      - "A ping"
      - "An initialized notification"
      - "An initialize request with its version, capabilities and identity"
    reponse: 3
    pourquoi: >-
      Before working, client and server agree.
      Which protocol version? Which features on each side?
      The server responds with its version, capabilities and identity.
      The client then confirms with the initialized notification.

  - id: versions-3
    sous_rubrique: versions
    question: "The client proposes version 2025-11-25. The server doesn't know it. What does the server do, in initialize mode?"
    choix:
      - "It crashes"
      - "It responds with another version it supports; if the client doesn't support it, the client disconnects"
      - "It accepts anyway"
      - "It returns -32601"
    reponse: 1
    pourquoi: >-
      Negotiation takes one round trip.
      The server proposes its most recent version.
      The client accepts or disconnects.
      Nobody speaks a version the other doesn't understand.

  - id: versions-4
    sous_rubrique: versions
    question: "Revision 2026-07-28: what replaces the initialize handshake?"
    choix:
      - "Nothing, the server guesses"
      - "An API key"
      - "A session cookie"
      - "Each request carries its version and the client's capabilities in _meta"
    reponse: 3
    pourquoi: >-
      The protocol becomes stateless.
      Each request holds everything needed to process it.
      If the version isn't supported, the server returns error -32022 with the list of accepted versions.
      server/discover lets you learn them in advance.

  - id: versions-5
    sous_rubrique: versions
    question: "Why make MCP stateless?"
    choix:
      - "To save bytes"
      - "Any instance of the server can handle any request"
      - "To remove tools"
      - "To forbid HTTP"
    reponse: 1
    pourquoi: >-
      With a session, the request must find the right server.
      Stateless, a load balancer sends it wherever it likes.
      A server that restarts loses no session.
      Needed state goes through explicit IDs, such as a cart number.

  - id: outils-1
    sous_rubrique: outils
    question: "tools/list returns, for each tool…"
    choix:
      - "Its name, description, input schema, and optionally an output schema and annotations"
      - "Its source code"
      - "Its price"
      - "Its logs"
    reponse: 0
    pourquoi: >-
      That is all the model will see of the tool.
      The name and description tell it when to use it.
      The input schema tells it what to provide.
      The output schema lets the result be validated.

  - id: outils-2
    sous_rubrique: outils
    question: "A tool call fails because the date is in the past. How should the MCP server report it?"
    choix:
      - "JSON-RPC error -32603"
      - "An empty result"
      - "A normal result with isError set to true and a clear message"
      - "Drop the connection"
    reponse: 2
    pourquoi: >-
      It is an execution error, not a protocol error.
      It must reach the model so it can correct itself.
      “The date must be in the future” lets it try again.
      JSON-RPC errors are for malformed requests or unknown tools.

  - id: outils-3
    sous_rubrique: outils
    question: "The client calls a tool that doesn't exist. What does the server return?"
    choix:
      - "isError set to true"
      - "A JSON-RPC error -32602"
      - "An empty result"
      - "Nothing"
    reponse: 1
    pourquoi: >-
      The request itself is invalid.
      It is a protocol error.
      -32602 signals invalid params, here an unknown tool name.

  - id: outils-4
    sous_rubrique: outils
    question: "A server declares readOnlyHint as true for a tool. Can the client rely on it?"
    choix:
      - "Yes, always"
      - "No, never"
      - "Yes, the protocol checks it"
      - "Only if the server is trusted: annotations are hints, not guarantees"
    reponse: 3
    pourquoi: >-
      An annotation is declared by the server itself.
      A malicious server can call itself “read-only” and delete everything.
      The specification requires treating them as untrusted.
      Unless the server comes from a trusted source.

  - id: outils-5
    sous_rubrique: outils
    question: "Two servers each expose a “search” tool. What should the client do?"
    choix:
      - "Keep the first"
      - "Refuse both"
      - "Tell them apart, for example by prefixing the name with the server"
      - "Merge them"
    reponse: 2
    pourquoi: >-
      A tool name is only unique within its server.
      The model wouldn't know which one to call.
      A prefix such as “github.search” removes the ambiguity.

  - id: ressources-1
    sous_rubrique: ressources
    question: "Who controls what in MCP?"
    choix:
      - "Tools: the model. Resources: the application. Prompts: the user"
      - "The model controls everything"
      - "The user controls everything"
      - "The server controls everything"
    reponse: 0
    pourquoi: >-
      The model decides to call a tool.
      The application decides which resources to put in context.
      The user picks a prompt, often through a slash command.
      Three primitives, three deciders.

  - id: ressources-2
    sous_rubrique: ressources
    question: "An MCP resource is identified by…"
    choix:
      - "A number"
      - "A tool name"
      - "A URI, such as file:///project/README.md"
      - "A hash"
    reponse: 2
    pourquoi: >-
      A URI names a piece of data unambiguously.
      resources/list gives the list. resources/read gives the content.
      URI templates allow parameterized resources.

  - id: ressources-3
    sous_rubrique: ressources
    question: "A server serves files through file://. Which check is mandatory?"
    choix:
      - "Sanitizing paths to prevent leaving the allowed folder"
      - "None"
      - "Encrypting the files"
      - "Limiting to 10 files"
    reponse: 0
    pourquoi: >-
      “../../.ssh/id_rsa” leaves the intended folder.
      That is directory traversal.
      The specification requires preventing it.

  - id: transports-1
    sous_rubrique: transports
    question: "With the stdio transport, your server prints a debug message with print(). What happens?"
    choix:
      - "Nothing serious"
      - "The server restarts"
      - "The message shows in the application"
      - "The client receives a line that isn't JSON-RPC, and communication breaks"
    reponse: 3
    pourquoi: >-
      stdout is reserved for MCP messages.
      A debug line there is read as an invalid message.
      Logs go to stderr.
      It is the most common mistake when writing a first server.

  - id: transports-2
    sous_rubrique: transports
    question: "In stdio, how are messages separated?"
    choix:
      - "By a length header"
      - "By a newline: one message per line, with no newline inside"
      - "By a null character"
      - "By a delay"
    reponse: 1
    pourquoi: >-
      The client reads line by line.
      Each line is a complete JSON message.
      A newline inside the JSON would split the message in two.
      Compact JSON serializers don't produce any.

  - id: transports-3
    sous_rubrique: transports
    question: "Your HTTP MCP server runs locally. Which precautions does the specification ask for?"
    choix:
      - "None, it is local"
      - "Turn off HTTPS"
      - "Listen on 0.0.0.0 to keep it simple"
      - "Listen on 127.0.0.1, check the Origin header, authenticate"
    reponse: 3
    pourquoi: >-
      A malicious web page can target your local server.
      That is the DNS rebinding attack.
      Checking Origin blocks it.
      Listening on localhost avoids exposure to the network.

  - id: transports-4
    sous_rubrique: transports
    question: "In stdio, where does the server get its credentials (API keys)?"
    choix:
      - "From a full OAuth flow"
      - "From its environment, for example environment variables"
      - "From the model"
      - "From the user, on each request"
    reponse: 1
    pourquoi: >-
      The stdio server is launched by the client, on the same machine.
      The specification asks it to take its credentials from the environment.
      The OAuth authorization framework is meant for HTTP transports.
---

## Why a protocol {#versions}

November 2024. Anthropic publishes the **Model Context Protocol**.

The problem: every AI application coded its own connectors to every tool.

:::pourquoi
10 applications, 20 tools: 200 integrations.
With a common protocol: 10 clients and 20 servers.
Every new server works with every application.
:::

In December 2025, Anthropic donates MCP to the Agentic AI Foundation, under the Linux Foundation. Anthropic, Block and OpenAI co-found it.

:::pourquoi
A standard backed by a single player worries its competitors.
A neutral foundation reassures those who adopt it.
:::

Three roles:

```mermaid
flowchart LR
  subgraph H["Host application"]
    L["LLM"]
    C1["MCP client 1"]
    C2["MCP client 2"]
  end
  C1 <-->|"JSON-RPC"| S1["GitHub server"]
  C2 <-->|"JSON-RPC"| S2["Files server"]
  L --- C1
  L --- C2
```

| Role | Example | Does what |
|---|---|---|
| Host | Claude Desktop, an IDE | Holds the model and the interface |
| Client | One per server, inside the host | Speaks the protocol |
| Server | GitHub, files, a database | Exposes tools, resources, prompts |

### Versions evolve

Up to revision **2025-11-25**, a connection starts with a handshake.

```mermaid
sequenceDiagram
  participant C as Client
  participant S as Server
  C->>S: initialize (version, capabilities, client identity)
  S-->>C: chosen version, capabilities, server identity
  C--)S: notifications/initialized
  C->>S: tools/list, tools/call...
```

:::pourquoi
Both sides agree on the version and the features available.
If the client doesn't support the proposed version, it disconnects.
Before that agreement, only pings are allowed.
:::

Revision **2026-07-28** removes this handshake. The protocol becomes **stateless**.

Each request carries its version and the client's capabilities in `_meta`. An unsupported version returns error -32022, with the list of accepted versions.

:::pourquoi
Without a session, any instance of the server handles any request.
A load balancer can send it wherever it likes.
A restart breaks nothing.
Needed state goes through explicit IDs: a cart number, a transaction number.
:::

Many servers still speak the old version. A client can speak both.

## tools/list and tools/call {#outils}

`tools/list`: the client discovers the tools.

`tools/call`: the client calls one, with its arguments.

The result contains `content` (text, image…), optionally `structuredContent` validated by the output schema, and `isError`.

Two kinds of errors:

| Error | Example | Form | Goes to the model? |
|---|---|---|---|
| Protocol | Unknown tool, malformed request | JSON-RPC error | Rarely useful |
| Execution | Date in the past, API unavailable | Result with `isError: true` | Yes, so it can correct itself |

:::pourquoi
“The date must be in the future” is actionable information.
The model can fix the argument and try again.
A protocol error signals a bug in the client, not a model mistake.
:::

**Annotations** (`readOnlyHint`, `destructiveHint`…) are hints. Not guarantees.

:::pourquoi
The server declares them.
A malicious server can lie.
The specification requires treating them as untrusted, unless the server is trusted.
:::

Return the tools in a stable order.

:::pourquoi
The tool list goes into the prompt.
A stable order lets the prompt cache reuse it.
:::

## Resources and prompts {#ressources}

MCP has three primitives. Each has its decider.

| Primitive | Who decides | Example |
|---|---|---|
| Tool | The model | Create a GitHub issue |
| Resource | The application | The project's README file |
| Prompt | The user | The /code-review command |

:::pourquoi
The model shouldn't decide everything.
The application picks the context. The user picks the task template.
The model acts within that frame.
:::

A **resource** is identified by a URI. You list it, read it, and can subscribe to its changes.

A **prompt** is a template of messages, with arguments. It often shows up as a slash command.

A file server must prevent directory traversal.

:::pourquoi
“../../.ssh/id_rsa” leaves the allowed folder.
The specification explicitly forbids it.
:::

## stdio and HTTP {#transports}

Two standard transports.

**stdio**: the client launches the server as a subprocess. They talk through standard input and output.

- One JSON message per line. No newline inside.
- `stdout`: MCP messages only.
- `stderr`: the logs.
- Shutdown: the client closes standard input, waits, then forces the stop if needed.

:::pourquoi
The client reads stdout line by line.
A single debug print() there breaks communication.
It is the number-one bug of first servers.
:::

**Streamable HTTP**: the server is a web service, with a single endpoint. It can stream its responses (SSE). It replaces the older HTTP+SSE transport from 2024.

Locally, the specification requires checking the `Origin` header, and recommends listening on 127.0.0.1 and authenticating.

:::pourquoi
A web page can target a local server through DNS rebinding.
Checking Origin blocks the attack.
Listening on 0.0.0.0 exposes the server to the whole network.
:::

Credentials: over HTTP, MCP's OAuth authorization framework. Over stdio, the environment.

## Key points

- MCP turns N × M integrations into N + M.
- Up to 2025-11-25: the initialize handshake. Since 2026-07-28: stateless, version in every request.
- Execution error: `isError`, so the model can correct itself. Protocol error: JSON-RPC.
- Annotations untrusted. stdout reserved for messages. Origin checked over HTTP.
