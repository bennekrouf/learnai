---
id: "04-03"
titre: "Strict schemas"
sous_rubriques:
  - id: json-schema
    titre: "JSON Schema"
  - id: valider
    titre: "Validating inputs and outputs"
sources:
  - titre: "MCP — Overview, revision 2026-07-28, JSON Schema Usage section"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/basic/index"
  - titre: "MCP — Tools, revision 2026-07-28 (security considerations, outputSchema)"
    url: "https://modelcontextprotocol.io/specification/2026-07-28/server/tools"
  - titre: "Pydantic — JSON Schema"
    url: "https://docs.pydantic.dev/latest/concepts/json_schema/"
qcm:
  - id: json-schema-1
    sous_rubrique: json-schema
    question: "What does additionalProperties set to false do in a tool schema?"
    choix:
      - "It rejects any field that isn't declared"
      - "It makes every field required"
      - "It allows empty fields"
      - "It turns validation off"
    reponse: 0
    pourquoi: >-
      Without it, an unknown argument slips through silently.
      The model may make up “force: true” or “admin: true”.
      The server may ignore it, or worse, use it.
      Rejecting the unknown is the default of a strict schema.

  - id: json-schema-2
    sous_rubrique: json-schema
    question: "The “priority” parameter only accepts low, normal or high. How do you write it?"
    choix:
      - "type: string, with a sentence in the description"
      - "type: number"
      - "type: string, enum: [low, normal, high]"
      - "type: object"
    reponse: 2
    pourquoi: >-
      A description is read. An enum is checked.
      The model sees the three possible values.
      The validator rejects “urgent” without ambiguity.

  - id: json-schema-3
    sous_rubrique: json-schema
    question: "What is each property's description in the schema for?"
    choix:
      - "Nothing, only the type counts"
      - "The model reads it to know what to put in the field"
      - "Internal documentation only"
      - "Validation"
    reponse: 1
    pourquoi: >-
      “date” doesn't say which format is expected.
      “Departure date, YYYY-MM-DD, in the future” does.
      The model only sees the schema and its descriptions.
      A good description avoids half the errors.

  - id: json-schema-4
    sous_rubrique: json-schema
    question: "A schema contains a $ref to an external URL. What does the MCP specification say?"
    choix:
      - "Download it automatically"
      - "Replace it with {}"
      - "Ignore it and accept everything"
      - "Don't resolve it automatically: that is off by default"
    reponse: 3
    pourquoi: >-
      Resolving a $ref means making a network request.
      A booby-trapped schema could target an internal service.
      The specification forbids automatic resolution.
      A schema that can't be resolved must be rejected, not accepted by default.

  - id: json-schema-5
    sous_rubrique: json-schema
    question: "With Pydantic, why generate the schema from a Python class?"
    choix:
      - "It runs faster"
      - "Pydantic is required for MCP"
      - "The same class describes the tool and validates the arguments: a single source of truth"
      - "To avoid JSON"
    reponse: 2
    pourquoi: >-
      A hand-written schema and separate validation code drift apart.
      With Pydantic, model_json_schema() produces the schema.
      The same class validates the arguments received.
      Changing a field changes both at once.

  - id: valider-1
    sous_rubrique: valider
    question: "The MCP client has already validated the arguments against the schema. Should the server validate again?"
    choix:
      - "Yes: the specification requires it, the client may be buggy or malicious"
      - "No, it is redundant"
      - "Only in production"
      - "Only for numbers"
    reponse: 0
    pourquoi: >-
      The server doesn't control its clients.
      A client may check nothing, or be written by an attacker.
      The specification says: servers must validate all inputs.
      The trust boundary is at the server.

  - id: valider-2
    sous_rubrique: valider
    question: "“The departure date must be in the future.” Can the schema guarantee it?"
    choix:
      - "Yes, with format: date"
      - "Yes, with minimum"
      - "No: it is a business rule, to check in code"
      - "Yes, with enum"
    reponse: 2
    pourquoi: >-
      The schema checks the shape: it is a date.
      It doesn't know today's date.
      Nor whether the user owns this booking.
      These rules live in code, after the schema.

  - id: valider-3
    sous_rubrique: valider
    question: "The arguments are invalid. Which message do you send back to the model?"
    choix:
      - "“departure_date invalid: 2024-03-01 is in the past. Today: 2026-11-19. Format YYYY-MM-DD.”"
      - "“Error.”"
      - "The full stack trace"
      - "Nothing"
    reponse: 0
    pourquoi: >-
      The model corrects itself with what it is told.
      “Error” tells it nothing.
      The field, the value, the rule and an example are enough.
      A stack trace, on the other hand, can leak internal paths.

  - id: valider-4
    sous_rubrique: valider
    question: "A tool declares an outputSchema. Who must follow it and who must check?"
    choix:
      - "Nobody, it is a hint"
      - "Only the client"
      - "The model must follow it"
      - "The server must follow it, the client should check it"
    reponse: 3
    pourquoi: >-
      The server commits to the shape of its result.
      The client checks before passing the result to the model or the code.
      An invalid output caught early avoids an error further on.

  - id: valider-5
    sous_rubrique: valider
    question: "Your LLM API offers a strict mode that forces arguments to match the schema. Can you drop server-side validation?"
    choix:
      - "Yes"
      - "No: the server serves other clients, and the schema doesn't cover business rules"
      - "Yes, if the temperature is 0"
      - "Yes, in development"
    reponse: 1
    pourquoi: >-
      Strict mode protects one client, not the server.
      Other clients call the same server.
      And a valid date can still be a past date.
      Each layer validates what it receives.
---

## JSON Schema {#json-schema}

An MCP tool describes its arguments with **JSON Schema**. By default, version 2020-12.

```json
{
  "type": "object",
  "properties": {
    "departure_date": { "type": "string", "format": "date",
                        "description": "Departure date, YYYY-MM-DD, in the future" },
    "priority": { "type": "string", "enum": ["low", "normal", "high"] },
    "passengers": { "type": "integer", "minimum": 1, "maximum": 9 }
  },
  "required": ["departure_date", "passengers"],
  "additionalProperties": false
}
```

The keywords that matter:

| Keyword | Role |
|---|---|
| `required` | Required fields |
| `enum` | Closed list of values |
| `minimum`, `maximum`, `pattern` | Bounds and format |
| `additionalProperties: false` | Rejects any undeclared field |
| `description` | What the model reads to fill the field |

:::pourquoi
A schema serves two readers.
The model reads it to know what to write.
The validator reads it to reject what is wrong.
:::

A tool with no parameters: `{"type": "object", "additionalProperties": false}`.

:::pourquoi
The specification recommends it.
It only accepts an empty object.
:::

Never resolve a remote `$ref` automatically.

:::pourquoi
It would be a network request triggered by a schema.
A booby-trapped schema could target an internal service.
:::

With **Pydantic**, a Python class produces the schema (`model_json_schema()`) and validates the arguments. `extra="forbid"` gives `additionalProperties: false`.

:::pourquoi
A single definition to describe and to validate.
They can no longer drift apart.
:::

## Validating inputs and outputs {#valider}

The server **always** validates its inputs. Even if the client already did.

```mermaid
flowchart LR
  M["Model"] -->|"arguments"| C["MCP client"]
  C -->|"tools/call"| S1["Server: schema"]
  S1 -->|"valid shape"| S2["Server: business rules"]
  S2 -->|"allowed"| X["Execution"]
  X --> S3["Server: output matches outputSchema"]
  S3 -->|"result"| C
  S1 -.->|"isError, precise message"| C
  S2 -.->|"isError, precise message"| C
```

:::pourquoi
The server doesn't control its clients.
The specification requires it to validate everything that comes in.
:::

Two layers:

1. **The schema**: types, fields, bounds.
2. **The business rules**: date in the future, resource owned by the user, amount under the cap.

:::pourquoi
A schema knows neither today's date nor the user's rights.
These rules live in code.
:::

On error, return a message the model can act on: the field, the value, the rule, an example.

:::pourquoi
The model corrects itself with what it is told.
“Error” teaches it nothing.
:::

Validate the output too. The server follows its `outputSchema`. The client checks.

:::pourquoi
A malformed output breaks the code that reads it.
Catching it at the boundary avoids a later error that is harder to understand.
:::

An LLM API's strict mode doesn't replace server validation.

:::pourquoi
It protects one client. The server has others.
And it doesn't check business rules.
:::

## Key points

- Strict schema: `required`, `enum`, bounds, `additionalProperties: false`.
- The model reads the descriptions. Take care over them.
- The server always validates: schema, then business rules.
- Precise error messages. Outputs validated too.
