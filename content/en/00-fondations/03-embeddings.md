---
id: "00-03"
titre: "Embeddings"
sous_rubriques:
  - id: vecteur
    titre: "A text becomes a vector"
  - id: cosinus
    titre: "Cosine similarity"
  - id: limites
    titre: "What an embedding captures, and what it misses"
sources:
  - titre: "Mikolov et al. — Efficient Estimation of Word Representations in Vector Space (word2vec), 2013"
    url: "https://arxiv.org/abs/1301.3781"
  - titre: "Nissim, van Noord, van der Goot — Fair is Better than Sensational: Man is to Doctor as Woman is to Doctor, Computational Linguistics, 2020"
    url: "https://arxiv.org/abs/1905.09866"
qcm:
  - id: vecteur-1
    sous_rubrique: vecteur
    question: "An embedding is…"
    choix:
      - "A fixed-size list of numbers that represents meaning"
      - "A summary of the text"
      - "The list of the text's tokens"
      - "A keyword index"
    reponse: 0
    pourquoi: >-
      An embedding model reads a text.
      It outputs a list of numbers, always the same length.
      Three words or three paragraphs: same size.
      Texts with close meanings give close lists.

  - id: vecteur-2
    sous_rubrique: vecteur
    question: "You switch embedding models. What do you do with the vectors already stored?"
    choix:
      - "Nothing, they stay compatible"
      - "Recompute only the new documents"
      - "Recompute all of them"
      - "Normalize them"
    reponse: 2
    pourquoi: >-
      Each model has its own space.
      One model's numbers mean nothing to another.
      Mixing the two gives random results.
      Switching models means reindexing the whole corpus.

  - id: vecteur-3
    sous_rubrique: vecteur
    question: "The question and the documents must be encoded…"
    choix:
      - "With two different models, for diversity"
      - "With the same embedding model"
      - "The question by the LLM, the documents by the embedding model"
      - "It doesn't matter"
    reponse: 1
    pourquoi: >-
      You compare positions within one space.
      Two models, two spaces.
      The comparison would mean nothing.

  - id: cosinus-1
    sous_rubrique: cosinus
    question: "Cosine similarity measures…"
    choix:
      - "The distance in number of words"
      - "The length of the vectors"
      - "The angle between two vectors"
      - "The number of shared tokens"
    reponse: 2
    pourquoi: >-
      It checks whether two vectors point the same way.
      Same direction: 1. Perpendicular: 0.
      The vector's length is ignored.
      Only the direction counts, that is, the meaning.

  - id: cosinus-2
    sous_rubrique: cosinus
    question: "Normalized vectors (length 1). Cosine similarity equals…"
    choix:
      - "The dot product"
      - "The Euclidean distance"
      - "Zero"
      - "The sum of the components"
    reponse: 0
    pourquoi: >-
      The cosine is the dot product divided by the lengths.
      If the lengths are 1, only the dot product is left.
      It is faster to compute.
      Many vector databases normalize for this reason.

  - id: cosinus-3
    sous_rubrique: cosinus
    question: "A document scores 0.82. Is it relevant?"
    choix:
      - "Yes, above 0.8 is always relevant"
      - "Yes, any positive score is relevant"
      - "No, it takes 0.95"
      - "It depends on the model and your data"
    reponse: 3
    pourquoi: >-
      Each model has its own scale.
      For one, 0.82 is excellent. For another, ordinary.
      A threshold doesn't copy from one project to the next.
      You calibrate it on examples of your own.

  - id: limites-1
    sous_rubrique: limites
    question: "“I like this movie” and “I don't like this movie”. Their embeddings are…"
    choix:
      - "Opposite"
      - "Very close"
      - "Perpendicular"
      - "Impossible to compare"
    reponse: 1
    pourquoi: >-
      Both sentences are about the same topic, with the same words.
      The embedding mostly captures the topic.
      The negation weighs little.
      So an embedding search can return the opposite opinion.

  - id: limites-2
    sous_rubrique: limites
    question: "You search for invoice “INV-2024-0042” by embedding. The risk?"
    choix:
      - "Finding other similar invoices but not that one"
      - "None"
      - "An API error"
      - "Too many tokens"
    reponse: 0
    pourquoi: >-
      An embedding captures meaning, not exact characters.
      To it, all invoices look alike.
      IDs, codes and rare names are searched by keyword.
      Hence the hybrid search of step 2.

  - id: limites-3
    sous_rubrique: limites
    question: "“king − man + woman ≈ queen” (word2vec, 2013). What is often left out?"
    choix:
      - "The result was “princess”"
      - "It only worked in English"
      - "The code excludes the question's words from the possible answers"
      - "The computation was done on tokens"
    reponse: 2
    pourquoi: >-
      The original code forbids answering king, man or woman.
      Without that filter, the answer is often one of the starting words.
      Researchers showed this in 2020.
      A nice demo, but less magical than advertised.
---

## A text becomes a vector {#vecteur}

An **embedding model** turns a text into a list of numbers.

The list always has the same size, whatever the text.

It is called a **vector**. It is a position in a space of meaning.

:::pourquoi
A computer is bad at comparing sentences.
It is very good at comparing numbers.
The vector makes meaning computable.
:::

Two texts with close meanings have close vectors.

```mermaid
flowchart LR
  D["Documents"] --> E["Embedding model"]
  Q["Question"] --> E
  E --> V["Vectors"]
  V --> B["Vector database"]
  B --> R["Closest documents"]
```

:::pourquoi
The question and the documents go through **the same** model.
Each model has its own space. Two spaces don't compare.
Switching models means recomputing everything.
:::

## Cosine similarity {#cosinus}

To compare two vectors, you measure the angle between them.

Same direction: 1. Unrelated: close to 0.

:::pourquoi
The direction carries the meaning.
The length depends on other things, such as the length of the text.
The cosine ignores the length.
:::

If the vectors are normalized, the cosine is a plain dot product.

:::pourquoi
Less computation, same result.
Over millions of documents, that matters.
:::

A score has no absolute meaning.

:::pourquoi
Each model has its own scale.
A threshold is calibrated on your own examples.
:::

## What an embedding captures, and what it misses {#limites}

It captures the **topic** well. It often misses the **precision**.

It handles these poorly:

- negation: “I like” and “I don't like” are neighbors;
- identifiers: invoice numbers, references, codes;
- rare names and exact figures.

:::pourquoi
The embedding sums up a text as a single position.
What barely changes the topic barely moves the position.
A negation or a number barely changes the topic.
:::

For these cases, you combine it with a keyword search. That is step 2.

The famous “king − man + woman ≈ queen” from 2013 has a small trick. The code excludes the starting words from the possible answers.

:::pourquoi
Without that filter, the answer is often one of the starting words.
Embeddings have real structure. But demos dress it up.
:::

## Key points

- An embedding is a position in a space of meaning.
- The same model for the question and the documents.
- The cosine compares directions. Thresholds are calibrated.
- Good at topic, weak at negation and identifiers.
