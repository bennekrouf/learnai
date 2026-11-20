---
id: "02-03"
titre: "Reranking"
sous_rubriques:
  - id: encodeurs
    titre: "Bi-encoder and cross-encoder"
  - id: compromis
    titre: "Latency versus precision"
sources:
  - titre: "Nogueira & Cho — Passage Re-ranking with BERT, 2019"
    url: "https://arxiv.org/abs/1901.04085"
  - titre: "Khattab & Zaharia — ColBERT: Efficient and Effective Passage Search via Contextualized Late Interaction over BERT, 2020"
    url: "https://arxiv.org/abs/2004.12832"
  - titre: "Thakur et al. — BEIR, NeurIPS 2021"
    url: "https://arxiv.org/abs/2104.08663"
qcm:
  - id: encodeurs-1
    sous_rubrique: encodeurs
    question: "A bi-encoder encodes the question and the document…"
    choix:
      - "Together, in a single pass"
      - "Only the document"
      - "Only the question"
      - "Separately, then compares their vectors"
    reponse: 3
    pourquoi: >-
      That is how a standard embedding model works.
      The documents' vectors are computed once, at indexing time.
      At search time, only the question is encoded.
      It is fast, but the two texts never “see” each other.

  - id: encodeurs-2
    sous_rubrique: encodeurs
    question: "Why is a cross-encoder more precise?"
    choix:
      - "It uses a larger vocabulary"
      - "It reads the question and the document together, word by word"
      - "It has more dimensions"
      - "It ignores rare words"
    reponse: 1
    pourquoi: >-
      The bi-encoder sums up each text as a vector, on its own.
      The cross-encoder reads both texts side by side.
      It sees whether the document really answers the question.
      It catches a negation or a condition the vector flattened.

  - id: encodeurs-3
    sous_rubrique: encodeurs
    question: "Why not use a cross-encoder on the whole corpus?"
    choix:
      - "It is less precise on large corpora"
      - "It doesn't support French"
      - "It needs one pass of the model per question-document pair, which can't be precomputed"
      - "Licenses forbid it"
    reponse: 2
    pourquoi: >-
      The cross-encoder needs the question to compute anything.
      Nothing can be prepared in advance.
      A million documents means a million passes per question.
      So it is kept for a short list of candidates.

  - id: encodeurs-4
    sous_rubrique: encodeurs
    question: "ColBERT (2020) is a trade-off. How?"
    choix:
      - "It keeps one precomputed vector per token, and compares token by token"
      - "It uses BM25 internally"
      - "It encodes only the titles"
      - "It keeps only one vector per corpus"
    reponse: 0
    pourquoi: >-
      One vector per token keeps more detail than one vector per text.
      These vectors are precomputed, as with a bi-encoder.
      The fine comparison happens at search time, token by token.
      More precise than a bi-encoder, faster than a cross-encoder. But more storage.

  - id: compromis-1
    sous_rubrique: compromis
    question: "The reranker gets the top 50 candidates. The right document is 80th. What happens?"
    choix:
      - "The reranker moves it to the top"
      - "The reranker widens the search"
      - "The LLM will find it anyway"
      - "It is lost: the reranker only sees what it is given"
    reponse: 3
    pourquoi: >-
      The reranker sorts. It doesn't search.
      A document missing from the candidates has no chance.
      So the first stage must aim for recall: miss nothing.
      The second aims for precision: sort well.

  - id: compromis-2
    sous_rubrique: compromis
    question: "You go from 50 to 200 candidates sent to the reranker. Consequence?"
    choix:
      - "No change"
      - "Better possible recall, but about 4 times the latency for reranking"
      - "Less precision"
      - "Latency divided by 4"
    reponse: 1
    pourquoi: >-
      More candidates, more chances of having the right one.
      But the cross-encoder makes one pass per candidate.
      Reranking time grows with their number.
      The right setting is measured: recall gained against milliseconds lost.

  - id: compromis-3
    sous_rubrique: compromis
    question: "According to BEIR (2021), which models do best on average, out of domain?"
    choix:
      - "Bi-encoders alone"
      - "The smallest models"
      - "BM25 alone"
      - "Rerankers and late-interaction models, at a high compute cost"
    reponse: 3
    pourquoi: >-
      Reading the question and the document together generalizes better.
      But it costs a lot on every query.
      Hence the funnel: fast and wide first, slow and fine next.

  - id: compromis-4
    sous_rubrique: compromis
    question: "Which latency should you watch for reranking?"
    choix:
      - "The p95: the latency that 95% of requests don't exceed"
      - "The average only"
      - "The fastest"
      - "None, reranking is instant"
    reponse: 0
    pourquoi: >-
      The average hides slow requests.
      One user in twenty may wait three times longer.
      The p95 shows what the hard cases go through.
      That is the topic of step 8.
---

## Bi-encoder and cross-encoder {#encodeurs}

Two ways to measure whether a document answers a question.

| | Bi-encoder | Cross-encoder |
|---|---|---|
| Reading | Question and document separately | Question and document together |
| Precomputing | Yes, at indexing time | Impossible |
| Speed | Very fast | Slow |
| Precision | Good | Better |
| Use | First search | Second sort |

:::pourquoi
The bi-encoder sums up each text alone, as a vector.
The cross-encoder sees both texts side by side.
It notices that a document is on the right topic without answering the question.
:::

In 2019, Nogueira and Cho apply BERT as a cross-encoder to re-sort passages. The gain is clear on the MS MARCO benchmark.

:::pourquoi
That is when neural reranking becomes the norm.
The idea hasn't changed since: a finer second sort on few candidates.
:::

**ColBERT** (2020) is a trade-off: one vector per token, precomputed, compared token by token.

:::pourquoi
More detail than one vector per text.
Precomputable, unlike the cross-encoder.
The price: much more storage.
:::

## Latency versus precision {#compromis}

Search is organized as a **funnel**.

```mermaid
flowchart LR
  C["Corpus: millions of chunks"] --> H["Hybrid search: top 100"]
  H --> R["Reranker: top 10"]
  R --> L["LLM"]
```

Stage 1: wide and fast. It aims for **recall**: miss nothing.

Stage 2: narrow and slow. It aims for **precision**: sort well.

:::pourquoi
The reranker sorts. It doesn't search.
A document missing from the candidates is lost for good.
:::

The number of candidates is the main setting.

:::pourquoi
More candidates, more chances of having the right one.
But the reranker makes one pass per candidate.
Four times the candidates, about four times the reranking time.
:::

Watch the p95 latency, not the average.

:::pourquoi
The average hides slow requests.
The p95 tells what the hard cases go through.
:::

## Key points

- Bi-encoder: fast, precomputed, for searching.
- Cross-encoder: precise, costly, for re-sorting.
- Funnel: recall first, precision next.
- The reranker never recovers a missing document.
