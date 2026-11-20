---
id: "01-01"
titre: "Token budget"
sous_rubriques:
  - id: repartir
    titre: "Splitting the window"
  - id: sortie
    titre: "Reserving the output"
sources:
  - titre: "Liu et al. — Lost in the Middle: How Language Models Use Long Contexts, TACL 2024"
    url: "https://arxiv.org/abs/2307.03172"
  - titre: "Hong, Troynikov, Huber (Chroma) — Context Rot: How Increasing Input Tokens Impacts LLM Performance, July 2025"
    url: "https://research.trychroma.com/context-rot"
qcm:
  - id: repartir-1
    sous_rubrique: repartir
    question: "Your model accepts 200,000 tokens. Should you fill the window to the max?"
    choix:
      - "Yes, more context gives better answers"
      - "No, the API refuses beyond 50%"
      - "Yes, the price doesn't depend on length"
      - "No, quality often drops as the input grows"
    reponse: 3
    pourquoi: >-
      In 2025, Chroma tests 18 models on simple tasks.
      The longer the input, the less reliable the results.
      Each extra token also costs money and time.
      The window is a ceiling, not a target.

  - id: repartir-2
    sous_rubrique: repartir
    question: "The key information is in a long context. Where does the model use it worst?"
    choix:
      - "In the middle"
      - "At the start"
      - "At the end"
      - "The same everywhere"
    reponse: 0
    pourquoi: >-
      The Lost in the Middle study measures a U-shaped curve.
      Models make good use of the start and the end.
      They make worse use of what is in the middle.
      So put the essentials at the edges.

  - id: repartir-3
    sous_rubrique: repartir
    question: "You add 10 barely relevant documents next to the right one. The right answer is still there. What happens?"
    choix:
      - "Nothing, the model finds the right document"
      - "The model refuses to answer"
      - "The model gets it wrong more often"
      - "The answer becomes more complete"
    reponse: 2
    pourquoi: >-
      A document that is close but wrong is a distractor.
      Chroma shows that the more distractors, the lower the quality.
      Having the right answer in the context isn't enough.
      You also have to remove what blurs it.

  - id: repartir-4
    sous_rubrique: repartir
    question: "Do the definitions of your 40 tools use up tokens?"
    choix:
      - "No, they are free"
      - "Yes, on every call"
      - "Only on the first call"
      - "Only if a tool is used"
    reponse: 1
    pourquoi: >-
      Tools are described in the model's input.
      The model has no memory, so they are sent again on every call.
      40 detailed tools can cost thousands of tokens.
      Load only the tools the task needs.

  - id: repartir-5
    sous_rubrique: repartir
    question: "How do you measure your prompt's size in tokens?"
    choix:
      - "Number of characters divided by 4"
      - "Number of words"
      - "With the model's tokenizer or token-counting API"
      - "Number of lines times 10"
    reponse: 2
    pourquoi: >-
      Dividing by 4 works on average, in English.
      Other languages, code and JSON stray far from it.
      Each model has its own tokenizer.
      For a reliable budget, count with the model's tool.

  - id: sortie-1
    sous_rubrique: sortie
    question: "A 200,000-token window. Your prompt takes 190,000. You expect a 15,000-token answer. A problem?"
    choix:
      - "The answer doesn't fit in the window"
      - "None"
      - "The prompt will be summarized"
      - "The answer will be faster"
    reponse: 0
    pourquoi: >-
      The answer grows in the same window as the prompt.
      Only 10,000 tokens are left.
      The answer will be cut off, or the request refused.
      Always compute: input plus maximum output.

  - id: sortie-2
    sous_rubrique: sortie
    question: "A model that “thinks” before answering. What is the impact on your budget?"
    choix:
      - "None, thinking is free"
      - "Thinking happens outside the window"
      - "Thinking shrinks the input"
      - "Thinking also uses output tokens"
    reponse: 3
    pourquoi: >-
      Reasoning is generated text.
      It is counted and billed as output.
      It takes room even before the visible answer.
      Plan a larger reserve.

  - id: sortie-3
    sous_rubrique: sortie
    question: "Who should reserve room for the output?"
    choix:
      - "The provider, automatically"
      - "Your application, before sending the request"
      - "The model, by shortening its answer"
      - "Nobody"
    reponse: 1
    pourquoi: >-
      The provider doesn't know how long an answer you expect.
      The model doesn't know how much room it has left.
      Only your application knows both.
      It computes the budget before each call.
---

## Splitting the window {#repartir}

The window is shared between several occupants.

```mermaid
flowchart LR
  C["System instructions"] --> F["Context window"]
  O["Tool definitions"] --> F
  M["Memory"] --> F
  D["Retrieved documents"] --> F
  H["History"] --> F
  F --> R["Reserve for the answer"]
```

Each occupant gets a share. Together, they must not exceed the window.

:::pourquoi
Without a budget, the biggest one wins.
A single huge tool result can push out the whole history.
A budget per occupant keeps the balance.
:::

More context doesn't mean better.

:::pourquoi
In 2025, Chroma tests 18 models.
The longer the input, the less reliable the answers. Even on simple tasks.
Passages that are close but wrong, distractors, make it worse.
:::

Position matters too. The middle is the least well-read zone.

:::pourquoi
The Lost in the Middle study (2023) draws a U-shaped curve.
The start and the end are used well. The middle, less so.
:::

Tools take room. Their definitions are sent again on every call.

:::pourquoi
The model has no memory.
Everything it needs to know, it receives every time.
:::

Measure in tokens, with the model's tokenizer. Not in characters.

:::pourquoi
The “4 characters = 1 token” rule holds for everyday English.
Code, JSON and other languages stray from it.
:::

## Reserving the output {#sortie}

The answer lives in the same window as the prompt.

Reserve its room **before** filling the rest.

:::pourquoi
If the input takes everything, the answer is cut off or refused.
A truncated answer looks like a normal one.
:::

The formula: input + maximum output ≤ window.

Models that think need a larger reserve.

:::pourquoi
Their reasoning is generated text.
It counts as output, before the visible answer.
:::

Your application is what does this computation.

:::pourquoi
Only it knows the input's size and the expected length.
:::

## Key points

- The window is a ceiling, not a target.
- More input, more distractors: less reliability.
- The middle is the least well-read zone.
- Reserve the output first. Count with the model's tokenizer.
