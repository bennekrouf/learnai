---
id: "03-03"
titre: "Grounding"
sous_rubriques:
  - id: citer
    titre: "Citing sources"
  - id: support
    titre: "Checking support"
sources:
  - titre: "Mata v. Avianca, Inc., S.D.N.Y. 2023 — sanctions order (Justia)"
    url: "https://law.justia.com/cases/federal/district-courts/new-york/nysdce/1:2022cv01461/575368/54"
  - titre: "Magesh et al. (Stanford) — Hallucination-Free? Assessing the Reliability of Leading AI Legal Research Tools, 2024"
    url: "https://arxiv.org/abs/2405.20362"
  - titre: "Stanford HAI — AI on Trial: Legal Models Hallucinate in 1 out of 6 (or More) Benchmarking Queries"
    url: "https://hai.stanford.edu/news/ai-trial-legal-models-hallucinate-1-out-6-queries"
  - titre: "Anthropic — Reduce hallucinations (documentation)"
    url: "https://platform.claude.com/docs/en/test-and-evaluate/strengthen-guardrails/reduce-hallucinations"
  - titre: "Es et al. — RAGAS: Automated Evaluation of Retrieval Augmented Generation, 2023"
    url: "https://arxiv.org/abs/2309.15217"
qcm:
  - id: citer-1
    sous_rubrique: citer
    question: "2023, Mata v. Avianca. A lawyer doubts the case law ChatGPT gave him. What does he do?"
    choix:
      - "He looks them up in a legal database"
      - "He calls the court clerk"
      - "He asks ChatGPT whether they are real"
      - "He withdraws them"
    reponse: 2
    pourquoi: >-
      He asks ChatGPT whether the Varghese case exists.
      ChatGPT says yes, and that it can be found in the major legal databases.
      It didn't exist. Nor did the others.
      Asking the model to check itself isn't a check.

  - id: citer-2
    sous_rubrique: citer
    question: "How do you make a citation checkable by your application?"
    choix:
      - "Give each chunk provided an ID, and require the answer to cite those IDs"
      - "Ask the model to make up a precise reference"
      - "Ask for a bibliography at the end"
      - "Trust the model"
    reponse: 0
    pourquoi: >-
      You provide chunks [1], [2], [3].
      The model must cite [2] after each claim.
      Your code checks that [2] is among the chunks provided.
      A reference outside that list is rejected outright.

  - id: citer-3
    sous_rubrique: citer
    question: "The model quotes a passage in quotation marks. How do you check it without an LLM?"
    choix:
      - "Impossible"
      - "Check the language"
      - "Count the words"
      - "Check that the quoted text appears word for word in the source chunk"
    reponse: 3
    pourquoi: >-
      An exact quote is checked with a plain text search.
      If it doesn't appear in the source, it is made up or distorted.
      It is free, fast and unambiguous.
      Hence the value of asking for word-for-word quotes.

  - id: citer-4
    sous_rubrique: citer
    question: "For a long document, Anthropic advises first asking for…"
    choix:
      - "A summary"
      - "Relevant word-for-word quotes, then the analysis based on them"
      - "A translation"
      - "A confidence score"
    reponse: 1
    pourquoi: >-
      Extracting the passages first forces the model to read.
      The analysis then rests on those exact passages.
      If it finds no passage, it must say so.
      The answer stays tied to the real text.

  - id: support-1
    sous_rubrique: support
    question: "Air Canada's chatbot cites the right page. Is its answer grounded?"
    choix:
      - "Yes, since it cites the page"
      - "Impossible to say"
      - "Yes, if the page is official"
      - "No: the page says the opposite of its answer"
    reponse: 3
    pourquoi: >-
      A citation says where a claim comes from.
      It doesn't prove the source supports it.
      You must check, claim by claim, that the source really says that.

  - id: support-2
    sous_rubrique: support
    question: "Stanford, 2024: legal tools with RAG, marketed as “hallucination-free”. Result?"
    choix:
      - "No errors"
      - "Wrong or badly sourced answers in more than one query in six"
      - "More errors than without RAG"
      - "The study couldn't test them"
    reponse: 1
    pourquoi: >-
      The researchers tested tools from LexisNexis and Thomson Reuters.
      They hallucinate less than GPT-4 without search.
      But they are still wrong in more than one query in six.
      RAG reduces hallucinations. It doesn't eliminate them.

  - id: support-3
    sous_rubrique: support
    question: "How do you check that an answer is supported by its sources?"
    choix:
      - "Split the answer into claims, and judge each against its source passage"
      - "Count the citations"
      - "Measure the answer's length"
      - "Ask the user"
    reponse: 0
    pourquoi: >-
      An answer mixes true and false claims.
      A single overall judgment drowns them.
      Each claim gets a verdict: supported, contradicted, absent.
      Those not supported are removed or flagged.

  - id: support-4
    sous_rubrique: support
    question: "An answer's faithfulness, in RAGAS, measures…"
    choix:
      - "Whether the answer is true in the real world"
      - "Its length"
      - "The share of its claims supported by the context provided"
      - "Its politeness"
    reponse: 2
    pourquoi: >-
      Faithfulness doesn't judge absolute truth.
      It judges the link between the answer and the documents provided.
      3 claims supported out of 4: a faithfulness of 0.75.
      A true answer that comes out of nowhere isn't faithful.
---

## Citing sources {#citer}

An answer without a source can't be checked.

Spring 2023. In *Mata v. Avianca*, a New York lawyer files a brief. It cites case law found with ChatGPT.

None of it exists.

He had even asked ChatGPT whether the cases were real. ChatGPT said yes.

:::pourquoi
The model produces likely text.
“Yes, this case exists” is a likely answer.
The judge sanctions the lawyers. The fault was not checking.
:::

The citation must be checkable **by your code**, not by the model.

```mermaid
sequenceDiagram
  participant A as Application
  participant R as Search
  participant L as LLM
  participant V as Checker
  A->>R: Question
  R-->>A: Chunks [1] [2] [3]
  A->>L: Question + numbered chunks
  L-->>A: Answer with [2] and a word-for-word quote
  A->>V: Answer + chunks
  V-->>A: Does [2] exist? Is the quote in [2]?
```

:::pourquoi
The IDs come from your application. The model can't make up valid ones.
A word-for-word quote is checked with a plain text search.
:::

For a long document, ask for the exact passages first, then the analysis.

:::pourquoi
It is a recommendation from Anthropic.
Extracting first forces the model to rely on the real text.
If it finds nothing, it says so instead of making things up.
:::

## Checking support {#support}

Citing isn't enough. Air Canada's chatbot cited the right page. The page said the opposite.

:::pourquoi
The citation says where the claim comes from.
It doesn't say whether the source supports it.
:::

In 2024, Stanford tests RAG-based legal tools. Some are marketed as “hallucination-free”. They are wrong in more than one query in six.

:::pourquoi
RAG reduces hallucinations, it doesn't eliminate them.
The model can misread, miscombine, or fill in from memory.
You must check the output, not just the input.
:::

The method: split the answer into claims, then judge each one.

A made-up example, an online shop:

| Claim | Source | Verdict |
|---|---|---|
| “Free returns within 60 days” | Terms of sale: “free returns within 30 days” | Contradicted |
| “Refunds go to the original payment method” | Terms of sale, section 7 | Supported |
| “Sale items can't be returned” | None | Absent |

:::pourquoi
An overall answer mixes true and false.
A verdict per claim shows exactly what is wrong.
What isn't supported is removed or flagged.
:::

**Faithfulness** is the share of claims supported by the context.

:::pourquoi
It doesn't judge the truth of the world. It judges the link to the sources.
A true answer with no source isn't faithful.
:::

## Key points

- The model doesn't check its own citations.
- IDs provided by the application, word-for-word quotes checked by code.
- Citing isn't supporting. Judge each claim.
- RAG reduces hallucinations. It doesn't eliminate them.
