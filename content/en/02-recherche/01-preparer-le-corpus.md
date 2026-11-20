---
id: "02-01"
titre: "Preparing the corpus"
sous_rubriques:
  - id: decoupage
    titre: "Chunking"
  - id: metadonnees
    titre: "Metadata"
  - id: indexation
    titre: "Indexing"
sources:
  - titre: "Anthropic — Introducing Contextual Retrieval, September 2024"
    url: "https://www.anthropic.com/engineering/contextual-retrieval"
  - titre: "Malkov & Yashunin — Efficient and robust approximate nearest neighbor search using HNSW graphs"
    url: "https://arxiv.org/abs/1603.09320"
qcm:
  - id: decoupage-1
    sous_rubrique: decoupage
    question: "Why split documents into chunks before indexing them?"
    choix:
      - "To save disk space"
      - "Because embedding models refuse long texts"
      - "One vector for 100 pages blends everything, and you only want to inject the useful passage"
      - "To translate more easily"
    reponse: 2
    pourquoi: >-
      A single vector for a long document averages all its topics.
      It no longer resembles any precise question.
      And injecting 100 pages for one useful sentence wastes the budget.
      A chunk is one idea, one vector, one passage you can inject.

  - id: decoupage-2
    sous_rubrique: decoupage
    question: "A chunk says: “Revenue grew by 3% over the previous quarter.” The problem?"
    choix:
      - "You no longer know which company or which quarter it is about"
      - "It is too long"
      - "It contains a figure"
      - "None"
    reponse: 0
    pourquoi: >-
      The company name and the date were higher up in the document.
      Chunking separated them.
      The question “ACME's revenue in Q2” won't find this chunk.
      It is Anthropic's example when presenting Contextual Retrieval.

  - id: decoupage-3
    sous_rubrique: decoupage
    question: "Contextual Retrieval (Anthropic, 2024) adds to each chunk…"
    choix:
      - "Its translation into English"
      - "A summary of the whole corpus"
      - "The whole document"
      - "A few sentences written by an LLM that place the chunk in its document"
    reponse: 3
    pourquoi: >-
      An LLM reads the document and the chunk.
      It writes 50 to 100 tokens of context: company, period, section.
      That context is prepended to the chunk before indexing.
      With keyword search as well, failures drop by 49%.

  - id: decoupage-4
    sous_rubrique: decoupage
    question: "What is overlap between two chunks for?"
    choix:
      - "Doubling the number of results"
      - "Not splitting an idea in two with no chunk holding all of it"
      - "Speeding up search"
      - "Nothing"
    reponse: 1
    pourquoi: >-
      A cut sometimes falls in the middle of an explanation.
      Each half becomes hard to understand.
      Repeating a few sentences from one chunk to the next limits that risk.
      The cost: a bit more storage and some duplicates.

  - id: decoupage-5
    sous_rubrique: decoupage
    question: "The best place to cut a document?"
    choix:
      - "Every 500 characters exactly"
      - "In the middle of sentences"
      - "At random"
      - "At natural boundaries: headings, paragraphs, functions"
    reponse: 3
    pourquoi: >-
      The author already structured the document by ideas.
      Cutting along that structure keeps chunks that make sense.
      A table cut in two loses its headers.
      A function cut in two no longer means anything.

  - id: metadonnees-1
    sous_rubrique: metadonnees
    question: "Google ended up filtering satire out of its AI summaries. Based on what?"
    choix:
      - "A piece of metadata: the kind of source"
      - "The text of the page"
      - "The length of the page"
      - "The language of the page"
    reponse: 0
    pourquoi: >-
      The text of a joke looks like the text of advice.
      Neither an embedding nor BM25 sees the difference.
      “Satire site” or “forum” is metadata.
      It has to be recorded at indexing time to filter on it.

  - id: metadonnees-2
    sous_rubrique: metadonnees
    question: "An employee has no access to the HR folder. Where do you enforce that?"
    choix:
      - "In the prompt: “don't reveal HR data”"
      - "At search time, by filtering on access rights"
      - "After the answer, by reviewing the text"
      - "Nowhere, the model is discreet"
    reponse: 1
    pourquoi: >-
      A document injected into the context is read by the model.
      An instruction can be bypassed.
      A review afterwards comes too late.
      The only safe filter: the document is never retrieved for that user.

  - id: metadonnees-3
    sous_rubrique: metadonnees
    question: "You search for the top 10 chunks, then keep those from 2024. None are left. Why?"
    choix:
      - "There is no document from 2024"
      - "The index is broken"
      - "The filter applied after the search removed all 10 results"
      - "BM25 ignores dates"
    reponse: 2
    pourquoi: >-
      The top 10 may all have been from 2023.
      Filtering afterwards recovers nothing.
      Filter before or during the search.
      Most vector databases allow it.

  - id: indexation-1
    sous_rubrique: indexation
    question: "HNSW, used by many vector databases, is a search that is…"
    choix:
      - "Approximate and fast"
      - "Exact and slow"
      - "By keyword"
      - "Random"
    reponse: 0
    pourquoi: >-
      Comparing the question with millions of vectors, one by one, is too slow.
      HNSW builds a graph of neighbors to jump to the right area.
      It sometimes misses a neighbor. That is the price of speed.
      Its parameters tune that trade-off.

  - id: indexation-2
    sous_rubrique: indexation
    question: "A document is updated. What do you do with its old chunks?"
    choix:
      - "Keep them, the more data the better"
      - "Mark them as old in the text"
      - "Remove them from every index before adding the new ones"
      - "Nothing, the index updates itself"
    reponse: 2
    pourquoi: >-
      Otherwise the old version can still be found.
      The model may quote an outdated price.
      Give each document a stable ID.
      On each update, all its chunks are replaced.

  - id: indexation-3
    sous_rubrique: indexation
    question: "What do you store with each chunk, besides its vector?"
    choix:
      - "Nothing"
      - "The question that found it"
      - "The whole document"
      - "Its text, the document's ID, its position and its metadata"
    reponse: 3
    pourquoi: >-
      The text, to inject it into the context.
      The ID, to cite the source and to update.
      The position, to fetch the neighboring chunks.
      The metadata, to filter.
---

## Chunking {#decoupage}

You don't search in documents. You search in **chunks**.

:::pourquoi
One vector for a long document averages its topics.
It no longer resembles any precise question.
A chunk carries one idea. It can be found and injected.
:::

Size is a trade-off.

| Too small | Too large |
|---|---|
| Loses its context | Mixes several ideas |
| “It grew by 3%”: who did? | Wastes the token budget |

Cut at natural boundaries: headings, paragraphs, functions. Never cut a table.

:::pourquoi
The author already structured the text by ideas.
A table without its headers no longer makes sense.
:::

Add **overlap**: a few sentences repeated from one chunk to the next.

:::pourquoi
A cut can fall in the middle of an explanation.
Overlap keeps the whole idea in at least one chunk.
:::

In 2024, Anthropic publishes **Contextual Retrieval**. An LLM writes 50 to 100 tokens of context for each chunk. They are prepended before indexing.

| Method | Failures in the top 20 |
|---|---|
| Baseline | 5.7% |
| + context, embeddings | 3.7% |
| + context, embeddings and BM25 | 2.9% |
| + reranking | 1.9% |

:::pourquoi
“Revenue grew by 3% over the previous quarter” becomes “ACME, Q2 2023: revenue grew by 3% over the previous quarter”.
The chunk can be found again.
These are Anthropic's figures, on its own data. On yours, they will differ.
:::

## Metadata {#metadonnees}

Each chunk carries a record: source, date, author, type, language, version, access rights.

:::pourquoi
The text doesn't say everything.
A joke and a piece of advice look alike. Only the source tells them apart.
That is what Google had to add after the glue on pizza.
:::

Access rights are filtered **during** the search.

:::pourquoi
A retrieved document is read by the model.
An instruction like “don't reveal it” can be bypassed.
The only safe filter: it is never retrieved.
:::

Filter before or during the search. Not after.

:::pourquoi
The filter may remove all of the top 10 results.
Then nothing is left, although good documents existed.
:::

## Indexing {#indexation}

Each chunk goes into two indexes.

```mermaid
flowchart LR
  D["Documents"] --> C["Chunker"]
  C --> X["LLM: the chunk's context"]
  X --> E["Embedding model"]
  E --> V["Vector index"]
  X --> B["BM25 index"]
  M["Metadata"] --> V
  M --> B
```

The vector index uses an **approximate** search, often HNSW.

:::pourquoi
Comparing the question with millions of vectors one by one is too slow.
HNSW jumps from neighbor to neighbor toward the right area.
It sometimes misses a neighbor. That is the price of speed.
:::

Store with each chunk: its text, the document's ID, its position, its metadata.

:::pourquoi
Text to inject it. ID to cite and update.
Position to read the neighbors. Metadata to filter.
:::

A document changes: replace **all** its chunks.

:::pourquoi
A forgotten old chunk can still be found.
The model will quote an outdated price, with full confidence.
:::

## Key points

- Chunks that carry one idea, cut along the structure.
- A chunk without context can't be found. Add it.
- Metadata filters what the text doesn't say, access rights included.
- On update: replace every chunk of the document.
