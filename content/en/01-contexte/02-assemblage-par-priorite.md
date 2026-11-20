---
id: "01-02"
titre: "Assembly by priority"
sous_rubriques:
  - id: couper
    titre: "What to cut first"
  - id: resume
    titre: "Truncation and summary"
  - id: debordement
    titre: "Preventing overflow"
sources:
  - titre: "Anthropic — Prompting best practices, long context section (documentation)"
    url: "https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices"
  - titre: "Microsoft Bing Blog (via Tom's Guide) — a limit of 5 turns per session, February 2023"
    url: "https://www.tomsguide.com/news/bing-with-chatgpt-is-getting-limited-to-just-5-questions-per-session"
qcm:
  - id: couper-1
    sous_rubrique: couper
    question: "The context overflows. What must you never cut?"
    choix:
      - "The system instructions and the current question"
      - "The old conversation turns"
      - "The least relevant documents"
      - "The old tool results"
    reponse: 0
    pourquoi: >-
      The instructions tell the model who it is and what it must respect.
      The question says what is expected now.
      Without either one, the answer no longer makes sense.
      The rest can be summarized or cut.

  - id: couper-2
    sous_rubrique: couper
    question: "What do you cut first?"
    choix:
      - "The user's question"
      - "The safety instructions"
      - "The last message"
      - "Old turns and large tool results already used"
    reponse: 3
    pourquoi: >-
      An old turn has already served.
      A tool result already used has already given its conclusion.
      These are the pieces worth the least for what comes next.
      Yet they often take the most room.

  - id: couper-3
    sous_rubrique: couper
    question: "A long document and a question. In what order do you place them?"
    choix:
      - "The question first, the document after"
      - "The document first, the question at the end"
      - "The question in the middle of the document"
      - "It doesn't matter"
    reponse: 1
    pourquoi: >-
      The end of the context is a well-read zone.
      A question at the end stays fresh when it is time to answer.
      Anthropic's documentation recommends this order.
      In its tests, quality can improve by up to 30%.

  - id: resume-1
    sous_rubrique: resume
    question: "Truncation or summary: what is truncation's advantage?"
    choix:
      - "It keeps the essentials"
      - "It improves the answer"
      - "It is free and predictable"
      - "It keeps the exact figures of what is cut"
    reponse: 2
    pourquoi: >-
      Cutting needs no call to the model.
      You know exactly what is left.
      But what is cut is lost entirely.

  - id: resume-2
    sous_rubrique: resume
    question: "You summarize an old conversation. What is likely to disappear?"
    choix:
      - "The overall topic"
      - "Nothing, a summary keeps everything"
      - "The language of the conversation"
      - "The exact details: figures, IDs, names"
    reponse: 3
    pourquoi: >-
      A summary keeps the meaning, not the letter.
      “Order 48213 for $312.50” becomes “an order”.
      Keep the precise items still needed aside, word for word.

  - id: resume-3
    sous_rubrique: resume
    question: "You cut the history. You remove a tool result but not the tool request that goes with it. Consequence?"
    choix:
      - "The API may reject the request"
      - "The model asks for the tool again"
      - "None"
      - "The model makes up the result"
    reponse: 0
    pourquoi: >-
      A tool request and its result form a pair.
      APIs check that each result answers a request.
      A broken pair makes the history invalid.
      Cut whole turns, never half a pair.

  - id: debordement-1
    sous_rubrique: debordement
    question: "When should summarizing the history kick in?"
    choix:
      - "When the API returns an error"
      - "Never, let the API handle it"
      - "At a threshold set in advance, before reaching the limit"
      - "On every message"
    reponse: 2
    pourquoi: >-
      Waiting for the error means failing in front of the user.
      Summarizing on every message costs too much.
      A threshold, say 80% of the budget, leaves time to act.

  - id: debordement-2
    sous_rubrique: debordement
    question: "A tool returns 300,000 tokens of logs. What do you do?"
    choix:
      - "Send it all to the model"
      - "Cap the result's size, and offer a way to read on"
      - "Ignore the tool"
      - "Switch models"
    reponse: 1
    pourquoi: >-
      A single result would overflow the window.
      Cap the size and say that it is truncated.
      Give a way to read on: next page, filter, search.
      The model will ask for what it needs.

  - id: debordement-3
    sous_rubrique: debordement
    question: "February 2023: Bing Chat gives disturbing answers. What does Microsoft do?"
    choix:
      - "It switches models"
      - "It deletes the history"
      - "It caps each session at 5 turns"
      - "It shuts the service down"
    reponse: 2
    pourquoi: >-
      Microsoft explains that very long sessions can confuse the model.
      It caps them at 5 turns per session and 50 per day.
      A few days later, it moves to 6 turns.
      It is the bluntest prevention: cut before the problem.
---

## What to cut first {#couper}

The context is too large. You have to choose.

Rank each piece by priority.

| Priority | Piece | Why |
|---|---|---|
| Never cut | System instructions, current question | Without them, there is no task |
| High | Latest turns, recent tool results | The task at hand depends on them |
| Medium | Relevant documents, useful memory | They help, but can be sorted |
| Low | Old turns, tool results already used | They have already served |

:::pourquoi
A piece is worth what it brings to the next answer.
An old tool result is often large and already summed up by the model itself.
:::

Several sources feed the context. An **assembler** sorts them.

```mermaid
flowchart LR
  C["Instructions"] --> A["Assembler"]
  H["History"] --> A
  M["Memory"] --> A
  R["Search"] --> A
  O["Tool results"] --> A
  A -->|"sort, cut, summarize"| P["Final prompt"]
  P --> L["LLM"]
```

:::pourquoi
Each source knows nothing of the others.
The assembler is the only one that sees the total budget.
It is the one that arbitrates.
:::

Order matters: long documents at the top, the question at the end.

:::pourquoi
The end is well read. The question stays fresh when it is time to answer.
The needle in the haystack showed that presentation changes the result.
:::

## Truncation and summary {#resume}

Two ways to shrink.

**Truncation**: cut. Free, predictable. What is cut is lost.

**Summary**: ask the model to condense. It costs a call. The exact details go.

:::pourquoi
A summary keeps the meaning, not the figures.
“Order 48213, $312.50” becomes “an order”.
Keep IDs, amounts and dates that are still needed word for word.
:::

The two are often combined: summarize old turns, truncate large results.

Cut by whole turns. Never a tool request without its result.

:::pourquoi
APIs check that each result answers a request.
A broken pair makes the history invalid.
:::

## Preventing overflow {#debordement}

Count **before** sending. Not after the error.

Set a threshold, say 80% of the budget. Beyond it, compact.

:::pourquoi
The error lands in front of the user.
The threshold leaves time to summarize cleanly.
:::

Cap the size of each tool result. Say that it is truncated. Give a way to read on.

:::pourquoi
A single huge result can push everything out.
The model knows to ask for another page if told that it can.
:::

In February 2023, Microsoft caps Bing Chat at 5 turns per session.

:::pourquoi
According to Microsoft, very long sessions could confuse the model.
Cutting the session is the simplest prevention.
But the user pays the price.
A good assembler does better: it compacts without erasing everything.
:::

## Key points

- Never cut: instructions and the current question.
- Cut first: old turns and tool results already used.
- Summary for meaning, word for word for exact details.
- Count before sending. Compact at a threshold.
