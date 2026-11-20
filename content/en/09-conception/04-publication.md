---
id: "09-04"
titre: "Publishing"
sous_rubriques:
  - id: analyse-architecture
    titre: "Analyzing your own architecture"
sources:
  - titre: "Anthropic — How we built our multi-agent research system, June 13, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, June 12, 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, May 2, 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
qcm:
  - id: analyse-architecture-1
    sous_rubrique: analyse-architecture
    question: "Why publish an analysis of your architecture?"
    choix:
      - "To show your reasoning: the choices, the trade-offs, the measurements and the failures"
      - "To show you use trendy tools"
      - "To hide the problems"
      - "To replace the code"
    reponse: 0
    pourquoi: >-
      The code shows what was done.
      The analysis shows why, and at what price.
      That is what a recruiter or a peer wants to assess.
      Judgment shows in the choices, not in the list of tools.

  - id: analyse-architecture-2
    sous_rubrique: analyse-architecture
    question: "Which outline does a good architecture analysis follow?"
    choix:
      - "The full code copied into the post"
      - "A list of the technologies used"
      - "Screenshots of the interface only"
      - "Problem and constraints, architecture, decisions and rejected options, measurements, failures, what I would change"
    reponse: 3
    pourquoi: >-
      The constraints explain the choices.
      The rejected options show there was a choice.
      The measurements prove.
      The failures and the “I would change” show perspective.

  - id: analyse-architecture-3
    sous_rubrique: analyse-architecture
    question: "Anthropic reports +90.2% for its multi-agent system. What makes the post credible?"
    choix:
      - "The number is very high"
      - "It also gives the cost (about 15 times more tokens) and the tasks the approach suits poorly"
      - "It only talks about successes"
      - "It is long"
    reponse: 1
    pourquoi: >-
      A gain without a price looks like advertising.
      A gain with its price and its limits looks like engineering.
      The reader can decide whether the trade-off holds for them.

  - id: analyse-architecture-4
    sous_rubrique: analyse-architecture
    question: "Should you tell your failures in a published analysis?"
    choix:
      - "Only in a footnote"
      - "No, never"
      - "Only if they are funny"
      - "Yes: an explained failure teaches more than a success and shows you tested"
    reponse: 3
    pourquoi: >-
      Cognition builds its post on a failure: Flappy Bird.
      OpenAI publishes what it missed with GPT-4o.
      A failure shows you measured, and what you learned.
      A post with no failure casts doubt on the tests.

  - id: analyse-architecture-5
    sous_rubrique: analyse-architecture
    question: "What must you remove before publishing?"
    choix:
      - "The diagrams"
      - "API keys, customer data, sensitive internal names, and prompts that reveal secrets"
      - "The numbers"
      - "The rejected options"
    reponse: 1
    pourquoi: >-
      A key in a code snippet gets stolen in minutes.
      An agent trace can hold personal data.
      You publish the reasoning, not the secrets.
      Step 6 applies to a post too.

  - id: analyse-architecture-6
    sous_rubrique: analyse-architecture
    question: "How do you present a number in the post?"
    choix:
      - "With its method: test set, number of cases and runs, error bar, model version"
      - "In bold, with no context"
      - "Rounded to the most flattering figure"
      - "With no unit"
    reponse: 0
    pourquoi: >-
      “92% success” says nothing on its own.
      Over 20 cases? Over 2,000? One run or five?
      The method lets the reader judge.
      It is the card from the Benchmarks topic.

  - id: analyse-architecture-7
    sous_rubrique: analyse-architecture
    question: "Which section best shows the author's perspective?"
    choix:
      - "The title"
      - "The acknowledgments"
      - "“What I would change”, with the reasons"
      - "The publication date"
    reponse: 2
    pourquoi: >-
      It proves you looked at your work critically.
      It shows what you learned.
      And it answers readers' criticism in advance.

  - id: analyse-architecture-8
    sous_rubrique: analyse-architecture
    question: "How do you make the analysis verifiable by the reader?"
    choix:
      - "Ask the reader to trust you"
      - "Link the repository, the ADRs, the diagrams and the evaluation results"
      - "Give no links"
      - "Publish only a video"
    reponse: 1
    pourquoi: >-
      The post summarizes. The repository proves.
      The ADRs give the detail of the decisions.
      The results let others redo the measurement.
      The whole chapter comes together here.

  - id: analyse-architecture-9
    sous_rubrique: analyse-architecture
    question: "Which title best suits an architecture analysis?"
    choix:
      - "“Using LangChain, Pinecone and GPT”"
      - "“My awesome revolutionary AI project”"
      - "“Final project”"
      - "“One agent instead of three: 2.8 times cheaper, and what it cost us”"
    reponse: 3
    pourquoi: >-
      The title announces a decision and its trade-off.
      It tells the reader what they will learn.
      A list of tools says nothing about the reasoning.
      A superlative drives technical readers away.

  - id: analyse-architecture-10
    sous_rubrique: analyse-architecture
    question: "Before publishing, who should review the post?"
    choix:
      - "Only the model that helped write it"
      - "Nobody"
      - "A peer for the technical side, and a security review for leaks"
      - "Only the author, one more time"
    reponse: 2
    pourquoi: >-
      A peer sees the holes in the reasoning.
      A security review sees the forgotten key or the customer data.
      What is published can never be fully taken back.
---

## Analyzing your own architecture {#analyse-architecture}

The code shows **what** was done. The analysis shows **why**, and **at what price**.

:::pourquoi
A recruiter or a peer wants to see your judgment.
Judgment shows in the choices and trade-offs, not in the list of tools.
:::

The outline:

| Section | Content |
|---|---|
| Problem and constraints | What had to be solved, with what budget, what latency, what data |
| Architecture | Data flow and agent topology |
| Decisions | The key choices, the rejected options, links to the ADRs |
| Measurements | Quality, cost, latency, with the method |
| Failures | What didn't work, and why |
| What I would change | With the reasons |

:::pourquoi
The constraints explain the choices.
The rejected options prove there was a choice.
The failures and the “I would change” show perspective.
:::

Give the **price** with the **gain**.

:::pourquoi
Anthropic reports +90.2% for its multi-agent system.
It also writes: about 15 times more tokens, and tasks it suits poorly.
That is what makes the post credible.
:::

Tell the **failures**.

:::pourquoi
Cognition builds its post on a failed Flappy Bird.
OpenAI publishes what it missed with GPT-4o.
A post with no failure casts doubt on the tests.
:::

Every number comes with its **method**: test set, number of cases, runs, error bar, model version.

:::pourquoi
“92%” over 20 cases and one run isn't worth “92%” over 2,000 cases and five runs.
The reader must be able to judge.
:::

Before publishing, two reviews:

```mermaid
sequenceDiagram
  participant A as Author
  participant P as Technical peer
  participant S as Security review
  participant L as Readers
  A->>P: Draft + repository + results
  P-->>A: Holes in the reasoning, numbers without a method
  A->>S: Corrected draft
  S-->>A: API key in a snippet, customer name in a trace
  A->>A: Removes the secrets, anonymizes
  A->>L: Publishes with links to repository, ADRs, diagrams, results
```

:::pourquoi
A peer sees what the author no longer sees.
A published key gets stolen in minutes.
What is published can never be fully taken back.
:::

You remove: **API keys**, **customer data**, **sensitive internal names**, **prompts** that reveal secrets.

:::pourquoi
An agent trace can hold personal data.
You publish the reasoning, not the secrets.
:::

A **title** that announces a decision and its trade-off.

| Weaker | Better |
|---|---|
| “My awesome AI project” | “One agent instead of three: 2.8 times cheaper” |
| “Using LangChain and Pinecone” | “Why our semantic cache returned the wrong answer” |

:::pourquoi
The title tells the reader what they will learn.
A list of tools says nothing about the reasoning.
:::

The whole chapter comes together here: the post **summarizes**, the repository **proves**.

:::pourquoi
The ADRs give the decisions.
The diagrams give the structure.
The evaluation results let others redo the measurement.
:::

## Key points

- Problem, architecture, decisions, measurements, failures, what I would change.
- The price with the gain. The failures with the successes.
- Every number with its method.
- Technical and security review. Links to the repository, the ADRs, the results.
