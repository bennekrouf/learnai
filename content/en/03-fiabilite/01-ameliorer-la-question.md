---
id: "03-01"
titre: "Improving the question"
sous_rubriques:
  - id: reformulation
    titre: "Rewriting"
  - id: decomposition
    titre: "Decomposition"
  - id: hyde
    titre: "HyDE"
sources:
  - titre: "Ma et al. — Query Rewriting for Retrieval-Augmented Large Language Models (Rewrite-Retrieve-Read), 2023"
    url: "https://arxiv.org/abs/2305.14283"
  - titre: "Press et al. — Measuring and Narrowing the Compositionality Gap in Language Models (Self-Ask), 2022"
    url: "https://arxiv.org/abs/2210.03350"
  - titre: "Gao, Ma, Lin, Callan — Precise Zero-Shot Dense Retrieval without Relevance Labels (HyDE), 2022"
    url: "https://arxiv.org/abs/2212.10496"
qcm:
  - id: reformulation-1
    sous_rubrique: reformulation
    question: "The user asks “What are the delivery times in France?”, then “And for Spain?”. What do you search for the second one?"
    choix:
      - "“Delivery times in Spain”"
      - "“And for Spain?”"
      - "“Spain”"
      - "The whole conversation"
    reponse: 0
    pourquoi: >-
      “And for Spain?” only makes sense with what came before.
      The search doesn't see the history.
      An LLM rewrites the question so it stands on its own.
      Sending the whole conversation would drown the search.

  - id: reformulation-2
    sous_rubrique: reformulation
    question: "Why generate several rewrites of the same question?"
    choix:
      - "To keep the model busy"
      - "To lower the cost"
      - "To translate"
      - "Each wording finds different documents, which are then fused"
    reponse: 3
    pourquoi: >-
      The user uses their words. The document uses its own.
      Three wordings cover more vocabulary than one.
      The results are fused, for example with RRF.
      The price: several searches instead of one.

  - id: reformulation-3
    sous_rubrique: reformulation
    question: "The user writes “PTO FMLA 2024”. What can rewriting do?"
    choix:
      - "Nothing"
      - "Expand the acronyms and add the corpus's terms: “paid time off”, “Family and Medical Leave Act”"
      - "Fix the spelling only"
      - "Remove the acronyms"
    reponse: 1
    pourquoi: >-
      An acronym may be written out in full in the document.
      Keyword search doesn't make the link on its own.
      Keeping the acronym AND its long form covers both cases.

  - id: decomposition-1
    sous_rubrique: decomposition
    question: "“Compare the refund terms of 2023 and 2024.” Why split it?"
    choix:
      - "The question is too long"
      - "The model can't compare"
      - "The answer is in two separate documents: one search per sub-question finds each"
      - "To save tokens"
    reponse: 2
    pourquoi: >-
      A single search brings back mostly documents from one of the two years.
      Two sub-questions, two searches, two documents.
      The model then compares what it has in front of it.

  - id: decomposition-2
    sous_rubrique: decomposition
    question: "Self-Ask (2022) speaks of a “compositionality gap”. It is…"
    choix:
      - "The price gap between two models"
      - "The gap between two languages"
      - "A model answers the sub-questions right, but fails on the question that combines them"
      - "The difference between BM25 and vector search"
    reponse: 2
    pourquoi: >-
      The model knows who acquired Instagram.
      It knows who runs Meta.
      But it misses “who runs the company that acquired Instagram”.
      Asking the sub-questions explicitly narrows that gap.

  - id: decomposition-3
    sous_rubrique: decomposition
    question: "When should you not decompose?"
    choix:
      - "When the question is simple: it adds cost and latency for nothing"
      - "Never, you should always decompose"
      - "When the question is in French"
      - "When the corpus is large"
    reponse: 0
    pourquoi: >-
      Each sub-question costs an LLM call and a search.
      “What is the notice period?” has only one part.
      A simple classifier can decide whether to split.

  - id: hyde-1
    sous_rubrique: hyde
    question: "The principle of HyDE?"
    choix:
      - "Hiding the question from the user"
      - "Translating the question"
      - "Searching by keyword only"
      - "Having an imaginary answer written, and searching with it"
    reponse: 3
    pourquoi: >-
      HyDE stands for Hypothetical Document Embeddings.
      The LLM writes a plausible answer, even an inaccurate one.
      That answer is encoded, not the question.
      You search for the real documents that look like it.

  - id: hyde-2
    sous_rubrique: hyde
    question: "Why does an imaginary answer find better than a question?"
    choix:
      - "It is shorter"
      - "It looks more like the documents: same shape, same vocabulary"
      - "It contains the real answer"
      - "It is in English"
    reponse: 1
    pourquoi: >-
      A question and a paragraph answering it don't look alike.
      Two answer paragraphs do.
      Even if the imagined facts are wrong, the style and vocabulary guide the search.

  - id: hyde-3
    sous_rubrique: hyde
    question: "Can HyDE's imaginary answer be shown to the user?"
    choix:
      - "Yes, it is the final answer"
      - "Yes, with a warning"
      - "Yes, if it is short"
      - "No, it is only for searching: it may be made up"
    reponse: 3
    pourquoi: >-
      It is written without any document.
      It often contains made-up facts.
      It only serves as a probe for the search.
      The final answer rests on the real documents retrieved.
---

## Rewriting {#reformulation}

The user's question isn't always a good search query.

It depends on the history. It uses their words, not the documents' words.

```mermaid
sequenceDiagram
  participant U as User
  participant A as Application
  participant L as LLM
  participant R as Search
  U->>A: And for Spain?
  A->>L: History + question: rewrite it so it stands alone
  L-->>A: Delivery times in Spain
  A->>R: Delivery times in Spain
  R-->>A: Documents
```

:::pourquoi
The search doesn't see the history.
“And for Spain?” alone finds nothing useful.
:::

You can generate several rewrites and fuse the results.

:::pourquoi
Each wording catches other documents.
You cover more vocabulary.
The price: several searches.
:::

Expand acronyms. Keep the short form too.

:::pourquoi
The document may use either one.
BM25 doesn't make the link on its own.
:::

## Decomposition {#decomposition}

A complex question hides several simple ones.

“Compare 2023 and 2024” → “Rules in 2023?” + “Rules in 2024?”

:::pourquoi
A single search favors one of the two topics.
One search per sub-question finds each piece.
:::

In 2022, Self-Ask measures a **compositionality gap**: the model answers the sub-questions, but misses the combined question.

:::pourquoi
Combining two facts silently is hard.
Writing out the steps makes each fact explicit.
:::

Don't split simple questions.

:::pourquoi
Each sub-question costs a call and a search.
:::

## HyDE {#hyde}

HyDE (2022) flips the problem. You don't search with the question. You search with an **imagined answer**.

```mermaid
flowchart LR
  Q["Question"] --> L["LLM: write a plausible answer"]
  L --> H["Imaginary answer"]
  H --> E["Embedding"]
  E --> R["Vector search"]
  R --> D["Real documents"]
  D --> F["Final answer, based on the real documents"]
```

:::pourquoi
A question doesn't look like an answer paragraph.
An imagined answer does.
Even when wrong, it has the right vocabulary and shape.
:::

The imaginary answer is **never** shown.

:::pourquoi
It is written without a source.
It is a probe, not an answer.
:::

## Key points

- Rewrite the question so it stands on its own.
- Several wordings, results fused.
- Split compound questions, not simple ones.
- HyDE: search with an imagined answer, never show it.
