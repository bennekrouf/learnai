---
id: "02-05"
titre: "GraphRAG"
sous_rubriques:
  - id: entites
    titre: "Entities and relations"
  - id: multi-sauts
    titre: "Multi-hop traversal"
  - id: communautes
    titre: "Community summaries"
  - id: cout
    titre: "When it isn't worth the cost"
sources:
  - titre: "Edge et al. (Microsoft) — From Local to Global: A Graph RAG Approach to Query-Focused Summarization, 2024"
    url: "https://arxiv.org/abs/2404.16130"
  - titre: "Microsoft Research — LazyGraphRAG: Setting a new standard for quality and cost, November 25, 2024"
    url: "https://www.microsoft.com/en-us/research/blog/lazygraphrag-setting-a-new-standard-for-quality-and-cost/"
qcm:
  - id: entites-1
    sous_rubrique: entites
    question: "In a knowledge graph, a relation is often written as…"
    choix:
      - "A vector"
      - "A triple: subject, relation, object"
      - "A chunk of text"
      - "A score"
    reponse: 1
    pourquoi: >-
      “Facebook — acquired — Instagram”.
      Two entities, linked by a named relation.
      Entities are the nodes. Relations are the edges.
      You can then follow the edges, which a vector index can't do.

  - id: entites-2
    sous_rubrique: entites
    question: "Who extracts the entities and relations in GraphRAG?"
    choix:
      - "A human"
      - "The embedding model"
      - "BM25"
      - "An LLM, chunk by chunk"
    reponse: 3
    pourquoi: >-
      An LLM reads each chunk.
      It lists the people, organizations, places, and their links.
      That is one LLM call per chunk.
      It is precise, but it is what makes indexing expensive.

  - id: entites-3
    sous_rubrique: entites
    question: "“Mr. Smith”, “John Smith” and “the CEO” refer to the same person. The challenge?"
    choix:
      - "Entity resolution: merging the mentions into a single node"
      - "Translation"
      - "Chunking"
      - "Reranking"
    reponse: 0
    pourquoi: >-
      Without merging, the graph has three nodes for one person.
      The relations scatter among them.
      The traversal misses links.
      Merging too much, on the other hand, blends two namesakes.

  - id: multi-sauts-1
    sous_rubrique: multi-sauts
    question: "“Who runs the company that acquired Instagram?” Why does vector search struggle?"
    choix:
      - "The question is too short"
      - "The answer is in two different chunks, and the second doesn't look like the question"
      - "Instagram is a proper name"
      - "It doesn't handle questions"
    reponse: 1
    pourquoi: >-
      Chunk 1: “Facebook acquires Instagram in 2012.”
      Chunk 2: “Mark Zuckerberg runs Meta, formerly Facebook.”
      The question looks like the first, not the second.
      You must first find Facebook, then look for who runs it. Two hops.

  - id: multi-sauts-2
    sous_rubrique: multi-sauts
    question: "How does a graph answer a multi-hop question?"
    choix:
      - "It looks for the closest chunk"
      - "It summarizes the whole corpus"
      - "It starts from the question's entities and follows the relations, hop by hop"
      - "It asks the user"
    reponse: 2
    pourquoi: >-
      “Instagram” is spotted in the question.
      Follow “acquired by”: Facebook.
      Follow “run by”: Mark Zuckerberg.
      Each edge points back to its source chunks, injected as evidence.

  - id: multi-sauts-3
    sous_rubrique: multi-sauts
    question: "Why limit the traversal to 1 or 2 hops?"
    choix:
      - "The number of neighbors explodes with each hop"
      - "Graphs only have 2 levels"
      - "It is a Microsoft rule"
      - "For security"
    reponse: 0
    pourquoi: >-
      An entity has 10 neighbors. Each of them has 10.
      Two hops: 100 nodes. Three hops: 1,000.
      The context fills with distant, useless links.
      Few hops, and keep the relevant edges.

  - id: communautes-1
    sous_rubrique: communautes
    question: "“What are the main themes of these 10,000 documents?” Why does vector RAG fail?"
    choix:
      - "The question is badly written"
      - "A reranker is missing"
      - "The answer is in no chunk: it takes an overall view"
      - "The corpus is too small"
    reponse: 2
    pourquoi: >-
      Vector RAG finds passages that look like the question.
      No passage lists the themes of the whole corpus.
      It is a global question.
      Microsoft designed GraphRAG for this kind of question.

  - id: communautes-2
    sous_rubrique: communautes
    question: "In GraphRAG, a “community” is…"
    choix:
      - "A group of users"
      - "A cluster of servers"
      - "A set of documents with the same date"
      - "A group of entities closely linked to one another in the graph"
    reponse: 3
    pourquoi: >-
      An algorithm such as Leiden splits the graph into dense groups.
      For example: all the entities tied to the same lawsuit.
      The split is hierarchical: large groups, subgroups.
      Each group roughly matches a theme.

  - id: communautes-3
    sous_rubrique: communautes
    question: "How does GraphRAG answer a global question?"
    choix:
      - "It reads the whole corpus for each question"
      - "It uses community summaries prepared at indexing time, then combines them"
      - "It takes the first document"
      - "It asks BM25"
    reponse: 1
    pourquoi: >-
      At indexing time, an LLM summarizes each community.
      At question time, each summary produces a partial answer.
      The partial answers are combined into a final answer.
      It is a map-reduce pattern.

  - id: cout-1
    sous_rubrique: cout
    question: "The main drawback of GraphRAG in its original version?"
    choix:
      - "It doesn't handle French"
      - "It is slow at question time"
      - "Indexing costs a lot in LLM calls"
      - "It doesn't cite its sources"
    reponse: 2
    pourquoi: >-
      One LLM call per chunk to extract the graph.
      Then one call per community to summarize it.
      On a large corpus, that is a lot of money.
      And it has to be redone when the corpus changes.

  - id: cout-2
    sous_rubrique: cout
    question: "LazyGraphRAG (Microsoft, November 2024) claims an indexing cost of…"
    choix:
      - "0.1% of GraphRAG's, the same as vector RAG"
      - "10 times GraphRAG's"
      - "The same as GraphRAG"
      - "Zero"
    reponse: 0
    pourquoi: >-
      LazyGraphRAG summarizes nothing in advance.
      It defers the LLM's work to question time.
      Indexing then costs as much as vector RAG.
      The cost moves: less at indexing, a bit more on each question.

  - id: cout-3
    sous_rubrique: cout
    question: "Your corpus is customer support: “how do I change my password?”. Do you need GraphRAG?"
    choix:
      - "Yes, always"
      - "No, you need a bigger LLM"
      - "Yes, for multilingual search"
      - "No: these are local questions, hybrid RAG with reranking is enough"
    reponse: 3
    pourquoi: >-
      The answer looks like the question and fits in one passage.
      That is the ideal case for vector and hybrid RAG.
      A graph would add cost with no gain.
      Start simple, measure, and add a graph only if multi-hop or global questions fail.
---

## Entities and relations {#entites}

Classic RAG stores isolated chunks. GraphRAG also stores **the links** between them.

An LLM reads each chunk and extracts:

- **entities**: people, organizations, places, products;
- **relations**: “Facebook — acquired — Instagram”.

:::pourquoi
A vector index can say “this chunk looks like the question”.
It can't say “this chunk is linked to that one”.
The graph keeps that link.
:::

The hardest part: recognizing that one actor goes by several names.

:::pourquoi
“Mr. Smith”, “John Smith”, “the CEO”: three nodes instead of one.
The relations scatter, the traversal misses links.
That is entity resolution.
:::

## Multi-hop traversal {#multi-sauts}

Some questions require linking two facts.

“Who runs the company that acquired Instagram?”

```mermaid
flowchart LR
  I["Instagram"] -->|"acquired by (2012)"| F["Facebook / Meta"]
  F -->|"run by"| Z["Mark Zuckerberg"]
  Q["Question"] -.->|"entity spotted"| I
```

:::pourquoi
The chunk about the acquisition looks like the question.
The chunk about the CEO doesn't.
Vector search finds the first and misses the second.
The graph follows the edge and finds both.
:::

Limit the traversal to 1 or 2 hops.

:::pourquoi
With each hop, the number of neighbors multiplies.
Beyond that, the context fills with distant links.
:::

## Community summaries {#communautes}

In 2024, Microsoft publishes GraphRAG for another family of questions: **global** questions.

“What are the main themes of this corpus?”

:::pourquoi
No passage contains the answer.
It takes an overall view.
Vector RAG has nothing similar to retrieve.
:::

The method:

```mermaid
flowchart LR
  M["Chunks"] --> X["LLM: extraction"]
  X --> G["Graph"]
  G --> C["Leiden: communities"]
  C --> S["LLM: one summary per community"]
  Q["Global question"] --> P["Partial answers, one per summary"]
  S --> P
  P --> F["Combined final answer"]
```

:::pourquoi
A community is a group of closely linked entities: often a theme.
Summarizing each theme in advance prepares the overall view.
At question time, the summaries are combined instead of rereading the corpus.
:::

## When it isn't worth the cost {#cout}

GraphRAG is expensive to build.

:::pourquoi
One LLM call per chunk to extract.
One call per community to summarize.
Everything must be redone when the corpus changes a lot.
:::

Microsoft admits it. In November 2024, it publishes **LazyGraphRAG**: indexing at the same cost as vector RAG, that is 0.1% of GraphRAG's.

:::pourquoi
LazyGraphRAG summarizes nothing in advance.
It puts the LLM to work at question time.
The cost moves from indexing to the query.
:::

Decision rule:

| Kind of question | Example | Tool |
|---|---|---|
| Local | “How do I change my password?” | Hybrid RAG + reranking |
| Multi-hop | “Who runs the company that acquired X?” | Graph, 1 to 2 hops |
| Global | “What are the main themes?” | Community summaries |

:::pourquoi
According to Microsoft, vector RAG excels when the answer looks like the question.
Start with it. Measure.
Add a graph only if multi-hop or global questions fail.
:::

## Key points

- A graph stores the links that vectors ignore.
- Multi-hop: follow the relations, 1 or 2 hops.
- Global questions: community summaries, combined.
- High indexing cost. Start simple, add a graph if measurement justifies it.
