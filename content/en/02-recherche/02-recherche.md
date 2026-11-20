---
id: "02-02"
titre: "Search"
sous_rubriques:
  - id: bm25
    titre: "BM25"
  - id: vectorielle
    titre: "Vector search"
  - id: hybride
    titre: "Hybrid fusion (RRF)"
sources:
  - titre: "Thakur et al. — BEIR: A Heterogeneous Benchmark for Zero-shot Evaluation of Information Retrieval Models, NeurIPS 2021"
    url: "https://arxiv.org/abs/2104.08663"
  - titre: "Cormack, Clarke, Büttcher — Reciprocal Rank Fusion outperforms Condorcet and individual Rank Learning Methods, SIGIR 2009"
    url: "https://plg.uwaterloo.ca/~gvcormac/cormacksigir09-rrf.pdf"
qcm:
  - id: bm25-1
    sous_rubrique: bm25
    question: "In BM25, which word of the question weighs the most?"
    choix:
      - "The most frequent in the corpus"
      - "The rarest in the corpus"
      - "The first in the question"
      - "The longest"
    reponse: 1
    pourquoi: >-
      A word present everywhere distinguishes no document.
      “the”, “of”, “contract” appear throughout a legal corpus.
      “early termination” is rare, so it discriminates.
      BM25 gives more weight to rare words. That is the IDF.

  - id: bm25-2
    sous_rubrique: bm25
    question: "A word appears 50 times in a document. Is the BM25 score 50 times higher than with a single occurrence?"
    choix:
      - "Yes, it is proportional"
      - "No, only the first occurrence counts"
      - "No, the effect saturates"
      - "Yes, and even more"
    reponse: 2
    pourquoi: >-
      Repeating a word doesn't make a document 50 times more relevant.
      BM25 raises the score fast at first, then less and less.
      That also limits documents stuffed with a keyword.

  - id: bm25-3
    sous_rubrique: bm25
    question: "Which question does BM25 miss most easily?"
    choix:
      - "“how do I cancel my subscription” when the document says “terminate the contract”"
      - "“invoice INV-2024-0042”"
      - "“article L1234-5”"
      - "“Dupont-Moretti”"
    reponse: 0
    pourquoi: >-
      BM25 counts shared words.
      “cancel” and “terminate” mean the same, but share no word.
      BM25 doesn't see synonyms.
      It excels, on the other hand, at IDs, codes and proper names.

  - id: bm25-4
    sous_rubrique: bm25
    question: "The question says “mice”, the document says “mouse”. What does BM25 need to link them?"
    choix:
      - "Nothing, it does it on its own"
      - "A higher temperature"
      - "An embedding"
      - "A language analyzer that brings words back to their root"
    reponse: 3
    pourquoi: >-
      To BM25, “mice” and “mouse” are two different words.
      A language analyzer normalizes: root forms, accents, stop words.
      With the wrong analyzer, the index misses these links.

  - id: vectorielle-1
    sous_rubrique: vectorielle
    question: "The strong point of vector search?"
    choix:
      - "Exact IDs"
      - "Synonyms and rephrasings"
      - "Version numbers"
      - "It costs nothing to index"
    reponse: 1
    pourquoi: >-
      The embedding captures meaning, not words.
      “cancel” and “terminate” end up close.
      With a multilingual model, a question in English finds a French text.

  - id: vectorielle-2
    sous_rubrique: vectorielle
    question: "The BEIR benchmark (2021) tests models on domains they haven't seen. What does it show?"
    choix:
      - "Vector search always beats BM25"
      - "All models are equal"
      - "BM25 is obsolete"
      - "BM25 remains a solid baseline, and some vector models do worse outside their domain"
    reponse: 3
    pourquoi: >-
      An embedding model learns on certain kinds of text.
      On a new domain, medical or legal, it can lose its footing.
      BM25 learned nothing, so it has nothing to forget.
      Before dropping BM25, measure on your data.

  - id: vectorielle-3
    sous_rubrique: vectorielle
    question: "Some embedding models ask for a different prefix for the question and the document. Why?"
    choix:
      - "A short question and a long passage don't look alike, even when one answers the other"
      - "For security"
      - "To count tokens"
      - "To switch languages"
    reponse: 0
    pourquoi: >-
      “What is the notice period?” doesn't look like a paragraph of a contract.
      The model learned to bring a question close to its answer.
      The prefix tells it which role each text plays.
      Forgetting it degrades results with no error message.

  - id: hybride-1
    sous_rubrique: hybride
    question: "Why combine BM25 and vector search?"
    choix:
      - "To double the number of results"
      - "They fail differently: one catches what the other misses"
      - "Because it is faster"
      - "To lower the cost"
    reponse: 1
    pourquoi: >-
      BM25 misses synonyms. Vector search misses IDs.
      A question can contain both.
      Fusing gives both kinds of match a chance.

  - id: hybride-2
    sous_rubrique: hybride
    question: "Why not simply add the BM25 score and the cosine score?"
    choix:
      - "It is too slow"
      - "The cosine is always zero"
      - "The two scores don't have the same scale"
      - "BM25 has no score"
    reponse: 2
    pourquoi: >-
      A BM25 score can be 3 or 30, depending on the question.
      A cosine stays between -1 and 1.
      Adding them would give BM25 all the power.
      So you fuse ranks, not scores.

  - id: hybride-3
    sous_rubrique: hybride
    question: "RRF with k = 60. Document A: 1st in BM25, absent from vector search. Document B: 2nd in both. Who wins?"
    choix:
      - "B"
      - "A"
      - "A tie"
      - "Impossible to say"
    reponse: 0
    pourquoi: >-
      A: 1/(60+1) ≈ 0.0164.
      B: 1/(60+2) + 1/(60+2) ≈ 0.0323.
      B wins clearly.
      RRF rewards agreement between the two searches.

  - id: hybride-4
    sous_rubrique: hybride
    question: "Where does RRF's value k = 60 come from?"
    choix:
      - "A mathematical proof"
      - "The number of results returned"
      - "A pilot experiment in 2009, and the choice turned out not to be critical"
      - "An ISO standard"
    reponse: 2
    pourquoi: >-
      Cormack, Clarke and Büttcher tried several values on TREC data.
      60 was close to the best, without the choice being decisive.
      The value stuck as the default almost everywhere.
      k dampens the gap between the 1st and 2nd ranks.
---

## BM25 {#bm25}

BM25 searches by **words**. It is the heir of 1990s search engines.

It favors three things:

- words that are **rare** in the corpus (they discriminate);
- words **present** in the document, with an effect that saturates;
- **short** documents, all else being equal.

:::pourquoi
A word present everywhere distinguishes nothing.
Repeating a word 50 times doesn't make a text 50 times more relevant.
A long document contains many words by chance.
:::

Strong at: IDs, codes, proper names, technical terms.

Weak at: synonyms, rephrasings, other languages.

:::pourquoi
BM25 compares words, not ideas.
“cancel” and “terminate” share no word.
:::

Configure a language analyzer for your corpus's language.

:::pourquoi
Without it, “mice” and “mouse” are two unrelated words.
The analyzer brings words back to their root and handles accents.
:::

## Vector search {#vectorielle}

Encode the question. Look for the chunks whose vector is closest.

Strong at: synonyms, rephrasings, multilingual search.

Weak at: IDs, negation, domains the model doesn't know.

:::pourquoi
The embedding captures the general meaning.
An invoice number has no “meaning”.
A model trained on general web text knows your business jargon poorly.
:::

In 2021, the BEIR benchmark tests models on 18 datasets. BM25 remains a solid baseline. Several vector models do worse outside their domain.

:::pourquoi
A model trained on one domain loses its footing on another.
BM25 learned nothing, so it has nothing to forget.
Measure on your data before choosing.
:::

Follow the model's instructions: some want one prefix for the question and another for the document.

:::pourquoi
A short question doesn't look like a long passage.
The prefix tells the model which role each text plays.
Forgetting it degrades results silently.
:::

## Hybrid fusion (RRF) {#hybride}

Run both searches. Fuse their rankings.

```mermaid
flowchart LR
  Q["Question"] --> B["BM25"]
  Q --> V["Vector search"]
  B -->|"ranking 1"| R["RRF fusion"]
  V -->|"ranking 2"| R
  R --> T["Fused top-k"]
```

:::pourquoi
They fail differently.
What one misses, the other often finds.
:::

Fuse the **ranks**, not the scores.

:::pourquoi
A BM25 score has no fixed scale. A cosine stays between -1 and 1.
Adding them would give BM25 all the power.
:::

**RRF** (Reciprocal Rank Fusion, 2009): each document earns 1/(k + rank) in each list. Add them up. k is often 60.

| Document | BM25 rank | Vector rank | RRF score |
|---|---|---|---|
| A | 1 | absent | 1/61 ≈ 0.0164 |
| B | 2 | 2 | 2 × 1/62 ≈ 0.0323 |

:::pourquoi
A document ranked well by both searches is a safe bet.
RRF rewards that agreement.
k = 60 dampens the gap between the 1st and 2nd ranks.
:::

## Key points

- BM25: exact words, IDs. Misses synonyms.
- Vector: meaning, synonyms. Misses IDs and unknown domains.
- Hybrid: fuse the ranks with RRF.
- Measure on your data before ruling out BM25.
