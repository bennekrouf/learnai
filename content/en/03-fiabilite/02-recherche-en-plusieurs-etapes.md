---
id: "03-02"
titre: "Multi-step search"
sous_rubriques:
  - id: boucler
    titre: "Looping"
  - id: arreter
    titre: "Knowing when to stop"
sources:
  - titre: "Trivedi et al. — Interleaving Retrieval with Chain-of-Thought Reasoning (IRCoT), 2022"
    url: "https://arxiv.org/abs/2212.10509"
  - titre: "Asai et al. — Self-RAG: Learning to Retrieve, Generate, and Critique through Self-Reflection, 2023"
    url: "https://arxiv.org/abs/2310.11511"
  - titre: "Yan et al. — Corrective Retrieval Augmented Generation (CRAG), 2024"
    url: "https://arxiv.org/abs/2401.15884"
qcm:
  - id: boucler-1
    sous_rubrique: boucler
    question: "Why isn't a single search always enough?"
    choix:
      - "Indexes are slow"
      - "What you read in a first document often tells you what to search for next"
      - "Models forget quickly"
      - "A search is limited to one word"
    reponse: 1
    pourquoi: >-
      “Who runs the company that acquired Instagram?”
      The first document reveals: Facebook.
      Only now do you know what to search for: Facebook's CEO.
      The second query depends on the first reading.

  - id: boucler-2
    sous_rubrique: boucler
    question: "In agentic RAG, who decides to run a new search?"
    choix:
      - "The model, through a search tool it calls when it needs to"
      - "The user"
      - "The vector database"
      - "A timer"
    reponse: 0
    pourquoi: >-
      Search becomes a tool, as in step 0.
      The model calls it, reads, and decides whether it needs more.
      It can change the query on each turn.
      It is the tool loop, applied to search.

  - id: boucler-3
    sous_rubrique: boucler
    question: "CRAG (2024) adds an evaluator of the retrieved documents. If it judges them bad?"
    choix:
      - "Answer with them anyway"
      - "Stop everything"
      - "Set them aside and search elsewhere, for example on the web"
      - "Give them to the reranker"
    reponse: 2
    pourquoi: >-
      Answering from bad documents leads to a bad answer.
      The evaluator grades each batch: correct, ambiguous or incorrect.
      Incorrect: discard and search another source.
      It is the correction before generation.

  - id: boucler-4
    sous_rubrique: boucler
    question: "Self-RAG (2023) teaches the model to decide…"
    choix:
      - "Whether to search, and whether its answer is supported by what it read"
      - "Which language to use"
      - "How many tokens to write"
      - "Which embedding model to pick"
    reponse: 0
    pourquoi: >-
      Not every question needs a search.
      Self-RAG trains the model to emit reflection tokens.
      They say: search or not, passage useful or not, answer supported or not.
      The model critiques its own work as it goes.

  - id: arreter-1
    sous_rubrique: arreter
    question: "Which stopping criterion is the soundest?"
    choix:
      - "When the model says it is satisfied, with no other limit"
      - "Never: the more you search, the better"
      - "After exactly 10 searches"
      - "Every sub-question has evidence, OR a maximum number of turns is reached"
    reponse: 3
    pourquoi: >-
      The goal is evidence for each part of the question.
      But a model can search forever.
      The maximum number of turns protects cost and latency.
      Both criteria together: stop when done, or when it takes too long.

  - id: arreter-2
    sous_rubrique: arreter
    question: "The new search returns the same documents as the previous one. What do you do?"
    choix:
      - "Run the same query again"
      - "Stop, or change strategy outright: you are going in circles"
      - "Raise the temperature"
      - "Double k"
    reponse: 1
    pourquoi: >-
      No new information came in.
      Going on costs without adding anything.
      Either you have everything the corpus holds, or the query is badly framed.
      In the first case, answer or abstain.

  - id: arreter-3
    sous_rubrique: arreter
    question: "Why is piling up every document from every step risky?"
    choix:
      - "There is no risk"
      - "Documents expire"
      - "The context fills with distractors and duplicates, and quality drops"
      - "The model refuses more than 5 documents"
    reponse: 2
    pourquoi: >-
      That is the Context Rot of step 1.
      Each step adds passages, relevant or not.
      Deduplicate, and keep only what serves the answer.
---

## Looping {#boucler}

Simple RAG does: search, then answer. Once.

Multi-step RAG does: search, read, decide, search again.

```mermaid
flowchart LR
  Q["Question"] --> L["LLM"]
  L -->|"query"| R["Search"]
  R --> E["Document evaluator"]
  E -->|"good"| L
  E -->|"bad: search elsewhere"| W["Other source"]
  W --> L
  L -->|"enough evidence"| F["Answer"]
```

:::pourquoi
What you read changes what you need to search for.
The first document often reveals the name to search for next.
:::

Several families of approaches:

| Approach | Idea |
|---|---|
| IRCoT (2022) | Alternate a reasoning step and a search |
| Self-RAG (2023) | The model decides whether to search, and judges whether it is supported |
| CRAG (2024) | An evaluator grades the documents, and searches elsewhere if they are bad |
| Agentic RAG | Search is a tool the model calls freely |

:::pourquoi
All share one idea: search is no longer a fixed step.
It becomes a decision made along the way.
:::

## Knowing when to stop {#arreter}

An endless loop costs a lot. And it doesn't improve the answer.

Stop when:

- each part of the question has evidence;
- **or** the maximum number of turns is reached;
- **or** searches bring back nothing new.

:::pourquoi
The model can always find a reason to search again.
The limit protects the budget and the response time.
Identical results signal you are going in circles.
:::

Don't keep everything the steps brought back.

:::pourquoi
Each step adds passages.
Duplicates and distractors lower the quality.
Deduplicate. Keep what serves the answer.
:::

If the loop stops without enough evidence, don't guess. Abstain.

:::pourquoi
A loop that fails is useful information.
It says the corpus doesn't hold the answer.
That is the subject of the Abstention topic.
:::

## Key points

- What you read tells you what to search for next.
- Search becomes a decision, made by the model or by an evaluator.
- Stop: complete evidence, maximum turns, or nothing new.
- No evidence at the end: abstain.
