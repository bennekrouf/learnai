---
id: "04-01"
titre: "JSON-RPC"
sous_rubriques:
  - id: requete
    titre: "Request and response"
  - id: notification
    titre: "Notification"
  - id: erreurs
    titre: "Errors"
sources:
  - titre: "JSON-RPC 2.0 Specification"
    url: "https://www.jsonrpc.org/specification"
  - titre: "Model Context Protocol — Overview, revision 2026-07-28 (messages and error codes)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/index"
qcm:
  - id: requete-1
    sous_rubrique: requete
    question: "A JSON-RPC 2.0 request contains…"
    choix:
      - "token, method, result"
      - "url, HTTP verb, body"
      - "only method"
      - "jsonrpc, id, method, and optionally params"
    reponse: 3
    pourquoi: >-
      jsonrpc is always “2.0”.
      method says what to do, for example “tools/call”.
      params carries the arguments.
      id links the response to the request.

  - id: requete-2
    sous_rubrique: requete
    question: "What is a request's id for?"
    choix:
      - "Authenticating the client"
      - "Linking each response to its request, even if responses arrive out of order"
      - "Counting billed requests"
      - "Encrypting the message"
    reponse: 1
    pourquoi: >-
      A client can send three requests without waiting.
      The third may be answered first.
      The response carries the same id as its request.
      That way the client knows what it answers.

  - id: requete-3
    sous_rubrique: requete
    question: "A valid JSON-RPC response contains…"
    choix:
      - "result OR error, never both"
      - "result and error together"
      - "neither"
      - "only the id"
    reponse: 0
    pourquoi: >-
      Either the operation succeeded: result.
      Or it failed: error.
      Both at once would be ambiguous.
      The client checks which one is present.

  - id: requete-4
    sous_rubrique: requete
    question: "Does JSON-RPC require a transport (HTTP, socket…)?"
    choix:
      - "Yes, HTTP only"
      - "Yes, WebSocket only"
      - "No, it defines the messages, not the channel"
      - "Yes, TCP only"
    reponse: 2
    pourquoi: >-
      JSON-RPC describes the shape of the messages.
      They can go over HTTP, a socket, or a process's standard input.
      That is what lets MCP have several transports.

  - id: notification-1
    sous_rubrique: notification
    question: "What sets a notification apart from a request?"
    choix:
      - "It is shorter"
      - "It has no id, and the recipient must not respond"
      - "It is encrypted"
      - "It goes through another channel"
    reponse: 1
    pourquoi: >-
      Without an id, there is nothing to link a response to.
      A notification is a one-way message.
      Example: “the tool list has changed”.
      The sender expects nothing back.

  - id: notification-2
    sous_rubrique: notification
    question: "A client wants to cancel a request in progress in MCP. It sends…"
    choix:
      - "A new request with the same id"
      - "An error -32603"
      - "Nothing, it closes the connection"
      - "A cancellation notification that names the request's id"
    reponse: 3
    pourquoi: >-
      Cancellation expects no response: it is a notification.
      It carries the id of the request to cancel.
      The server stops the work as soon as it can.
      It sends nothing more for that request.

  - id: erreurs-1
    sous_rubrique: erreurs
    question: "The client calls a method the server doesn't know. Which code?"
    choix:
      - "-32700"
      - "-32602"
      - "-32601"
      - "-32603"
    reponse: 2
    pourquoi: >-
      -32601: method not found.
      -32700: unreadable JSON.
      -32602: invalid params.
      -32603: internal server error.

  - id: erreurs-2
    sous_rubrique: erreurs
    question: "The server receives text that isn't valid JSON. Which code?"
    choix:
      - "-32700, parse error"
      - "-32600, invalid request"
      - "-32601, method not found"
      - "-32603, internal error"
    reponse: 0
    pourquoi: >-
      The server couldn't even read the message.
      That is a parse error: -32700.
      -32600 is for JSON that is readable but isn't shaped like a request.

  - id: erreurs-3
    sous_rubrique: erreurs
    question: "What does a response's error object contain?"
    choix:
      - "A full stack trace, required"
      - "Only free text"
      - "An integer code, a message, and optionally data"
      - "An HTTP status code"
    reponse: 2
    pourquoi: >-
      The code lets the program react.
      The message helps the human reading the logs.
      data can add detail, for example the supported versions.
      Never put a secret in it: it goes back to the client.
---

## Request and response {#requete}

MCP speaks **JSON-RPC 2.0**. A simple format to call a function remotely.

```json
{ "jsonrpc": "2.0", "id": 7, "method": "tools/call",
  "params": { "name": "weather", "arguments": { "city": "Lyon" } } }
```

```json
{ "jsonrpc": "2.0", "id": 7, "result": { "content": [ { "type": "text", "text": "12 °C" } ] } }
```

The same id in the request and the response.

:::pourquoi
Several requests can be in flight at once.
Responses can come back out of order.
The id says which request each response answers.
:::

A response contains `result` **or** `error`. Never both.

MCP adds two rules: the id is never `null`, and it is unique among requests in progress.

:::pourquoi
A null or reused id would make the match ambiguous.
:::

JSON-RPC says nothing about the transport.

:::pourquoi
It describes the messages, not the pipe.
So MCP can go through standard input or through HTTP.
:::

## Notification {#notification}

A notification is a message **with no id**. Nobody responds to it.

```json
{ "jsonrpc": "2.0", "method": "notifications/tools/list_changed" }
```

:::pourquoi
Some messages inform without asking for anything.
“The tool list has changed.” “Cancel request 7.”
Waiting for a response would be pointless.
:::

## Errors {#erreurs}

JSON-RPC's standard codes:

| Code | Meaning | Example |
|---|---|---|
| -32700 | Parse error | Unreadable JSON |
| -32600 | Invalid request | `method` is missing |
| -32601 | Method not found | `tools/cal` instead of `tools/call` |
| -32602 | Invalid params | Unknown tool, missing argument |
| -32603 | Internal error | Exception on the server |

MCP reserves its own codes, for example -32022 for an unsupported protocol version.

:::pourquoi
An integer code can be tested in the program.
A free-text message can't be tested reliably.
:::

Never put a secret in `message` or `data`.

:::pourquoi
The error goes back to the client, and often to the model.
A file path or a key in an error is a leak.
:::

```mermaid
sequenceDiagram
  participant H as Host application
  participant C as MCP client
  participant S as MCP server
  H->>C: Call the weather tool
  C->>S: Request id 7: tools/call
  C->>S: Request id 8: resources/read
  S-->>C: Response id 8
  S-->>C: Response id 7
  C->>S: Notification: cancel id 9
  C-->>H: Result of the weather tool
```

## Key points

- Request: id, method, params. Response: same id, result or error.
- Notification: no id, no response.
- Error codes are testable. Messages carry no secrets.
- JSON-RPC describes the messages, not the transport.
