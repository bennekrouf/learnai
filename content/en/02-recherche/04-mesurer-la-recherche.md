---
id: "02-04"
titre: "Measuring search"
sous_rubriques:
  - id: metriques
    titre: "recall@k, MRR, nDCG"
  - id: jeu-de-test
    titre: "Building a test set"
sources:
  - titre: "Anthropic — Introducing Contextual Retrieval (measure: failures in the top 20)"
    url: "https://www.anthropic.com/engineering/contextual-retrieval"
  - titre: "Järvelin & Kekäläinen — Cumulated gain-based evaluation of IR techniques (nDCG), 2002"
    url: "https://dl.acm.org/doi/10.1145/582415.582418"
qcm:
  - id: metriques-1
    sous_rubrique: metriques
    question: "3 documents are relevant. 2 appear in the top 10 results. What is the recall@10?"
    choix:
      - "0.2"
      - "0.67"
      - "1"
      - "0.3"
    reponse: 1
    pourquoi: >-
      recall@k counts the share of relevant documents found in the top k.
      2 found out of 3 that exist: 2/3 ≈ 0.67.
      It ignores the order within the 10.

  - id: metriques-2
    sous_rubrique: metriques
    question: "For a RAG system, why is recall@k the key measure of the first stage?"
    choix:
      - "It is the simplest to compute"
      - "It measures the quality of the answer"
      - "What isn't in the k chunks sent to the LLM can't be used to answer"
      - "It measures latency"
    reponse: 2
    pourquoi: >-
      The LLM only sees the k chunks it is given.
      A good document at rank k+1 doesn't exist for it.
      Anthropic in fact measures its gains as failures in the top 20.
      That is 1 minus recall@20.

  - id: metriques-3
    sous_rubrique: metriques
    question: "Three questions. The first good result is at rank 1, then 2, then 4. What is the MRR?"
    choix:
      - "≈ 0.58"
      - "≈ 2.33"
      - "0.25"
      - "1"
    reponse: 0
    pourquoi: >-
      Take the inverse of the rank of the first good result.
      1/1 = 1. 1/2 = 0.5. 1/4 = 0.25.
      Average: 1.75 / 3 ≈ 0.58.
      MRR punishes hard a good result that doesn't come first.

  - id: metriques-4
    sous_rubrique: metriques
    question: "When do you prefer nDCG to MRR?"
    choix:
      - "When there is only one right answer"
      - "When the relevant documents are unknown"
      - "When several documents are relevant, to different degrees"
      - "Never"
    reponse: 2
    pourquoi: >-
      MRR only looks at the first good result.
      nDCG grades each document: very relevant, somewhat, not at all.
      It rewards the most relevant placed at the top.
      It equals 1 when the order is perfect.

  - id: metriques-5
    sous_rubrique: metriques
    question: "In nDCG, what does the “D”, for discounted, mean?"
    choix:
      - "Recent documents count more"
      - "Duplicates are removed"
      - "Long documents are penalized"
      - "A relevant document counts less when it is ranked lower"
    reponse: 3
    pourquoi: >-
      A good document at rank 1 is worth more than at rank 8.
      Its gain is divided by a function that grows with the rank.
      The ranking counts, not just presence.

  - id: jeu-de-test-1
    sous_rubrique: jeu-de-test
    question: "A test set for search is…"
    choix:
      - "A list of documents"
      - "Questions, each with the chunks that answer it"
      - "Answers written by the LLM"
      - "Latency logs"
    reponse: 1
    pourquoi: >-
      To measure recall, you need to know what should have been found.
      So each question carries the list of its relevant chunks.
      Without it, none of the measures above can be computed.

  - id: jeu-de-test-2
    sous_rubrique: jeu-de-test
    question: "You have an LLM generate the questions from each chunk. The trap?"
    choix:
      - "The questions will be in English"
      - "There will be too many questions"
      - "The questions reuse the chunk's words, which flatters BM25 and overrates the search"
      - "None"
    reponse: 2
    pourquoi: >-
      An LLM reading a chunk reuses its words.
      Real users don't know those words.
      The test becomes too easy.
      Mix in real user questions, and rephrase.

  - id: jeu-de-test-3
    sous_rubrique: jeu-de-test
    question: "Which cases must not be missing from your test set?"
    choix:
      - "IDs, synonyms, multi-document questions, and questions with no answer"
      - "Only easy questions"
      - "Only long questions"
      - "Only questions in English"
    reponse: 0
    pourquoi: >-
      IDs test BM25. Synonyms test vector search.
      Multi-document questions test recall.
      Questions with no answer test abstention, in step 3.
      A test made of easy cases will tell you nothing.

  - id: jeu-de-test-4
    sous_rubrique: jeu-de-test
    question: "You tune the chunk size until you get the best score on your test set. The risk?"
    choix:
      - "None"
      - "The score drops"
      - "The test set gets too large"
      - "Overfitting the test set: the score rises, the real quality doesn't"
    reponse: 3
    pourquoi: >-
      Tuning again and again on the same questions optimizes for them.
      Real questions are different.
      Keep part of the set aside, never used for tuning.
      That part tells the truth.
---

## recall@k, MRR, nDCG {#metriques}

Without measurement, you tune blind.

Three measures, three questions.

| Measure | Question | Use |
|---|---|---|
| recall@k | Are the right documents in the top k? | First stage of a RAG system |
| MRR | At what rank does the first right document come? | A single right answer |
| nDCG | Are the most relevant at the very top? | Graded relevance |

**recall@k**: relevant documents found in the top k, divided by relevant documents that exist.

:::pourquoi
The LLM only sees the k chunks it is given.
What sits at rank k+1 doesn't exist for it.
Anthropic measures Contextual Retrieval as failures in the top 20: 1 minus recall@20.
:::

**MRR**: the average of 1/rank of the first good result.

:::pourquoi
Rank 1 is worth 1. Rank 2 is worth 0.5. Rank 10 is worth 0.1.
It punishes hard a good result that doesn't come first.
:::

**nDCG**: each document has a relevance grade. Its gain shrinks with the rank. Divide by the score of a perfect order.

:::pourquoi
Some documents are very relevant, others somewhat.
nDCG rewards the best ones placed at the top.
1 means a perfect order.
:::

## Building a test set {#jeu-de-test}

A test set: questions, and for each one, the chunks that answer it.

:::pourquoi
To measure what you missed, you need to know what should have been found.
:::

Where do the questions come from?

- Real users: the best, but they need labeling.
- Generated by an LLM from the chunks: fast, but too easy.

:::pourquoi
An LLM reading a chunk reuses its words.
Keyword search finds them too well.
The score becomes flattering.
:::

Cover the hard cases:

- IDs and codes;
- synonyms and rephrasings;
- answers spread over several documents;
- questions with **no** answer in the corpus.

:::pourquoi
Each case tests a different weakness.
A set of easy cases shows 95% and teaches you nothing.
:::

Start small: 50 to 100 questions. Grow it with real failures.

Keep part of the set aside. Never tune on it.

:::pourquoi
Tuning again and again on the same questions optimizes for them.
The part set aside tells whether the gain is real.
:::

Run the measurement again on every change: chunking, model, filters, reranker.

```mermaid
flowchart LR
  Q["Test set"] --> S["Search system"]
  S --> R["Ranked results"]
  R --> M["Compute: recall@k, MRR, nDCG"]
  Q -->|"expected chunks"| M
  M --> D["Decision: keep or revert the change"]
```

## Key points

- recall@k for the first stage. MRR or nDCG for the order.
- A test set: questions and expected chunks.
- LLM-generated questions: too easy on their own.
- Part of the set kept aside, never used for tuning.
