---
id: "07-01"
titre: "What we evaluate"
sous_rubriques:
  - id: reponse
    titre: "The answer"
  - id: trajectoire
    titre: "The trajectory"
sources:
  - titre: "Yao et al. — τ-bench: A Benchmark for Tool-Agent-User Interaction in Real-World Domains, 2024"
    url: "https://arxiv.org/abs/2406.12045"
  - titre: "OpenAI — Expanding on what we missed with sycophancy, May 2, 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
qcm:
  - id: reponse-1
    sous_rubrique: reponse
    question: "Which aspects of a final answer should you evaluate?"
    choix:
      - "Only the spelling"
      - "Only the length"
      - "Only the speed"
      - "Accuracy, grounding in the sources, completeness, format, tone"
    reponse: 3
    pourquoi: >-
      An answer can be right but incomplete.
      Complete but made up.
      Accurate but in the wrong format.
      Each aspect is measured separately.

  - id: reponse-2
    sous_rubrique: reponse
    question: "The expected answer is an amount: $1,240. How do you check it?"
    choix:
      - "With an LLM judge"
      - "With an exact comparison in code, no LLM"
      - "By asking the user"
      - "You can't"
    reponse: 1
    pourquoi: >-
      A number can be compared exactly.
      Code is fast, free and deterministic.
      An LLM judge would add noise and cost.
      Keep the judge for what code can't judge.

  - id: reponse-3
    sous_rubrique: reponse
    question: "The evaluations of the April 2025 GPT-4o update were good. Why didn't they see the sycophancy?"
    choix:
      - "None of them measured sycophancy"
      - "They were broken"
      - "The model cheated on the tests"
      - "They had been deleted"
    reponse: 0
    pourquoi: >-
      OpenAI wrote it: no evaluation dedicated to this behavior.
      The tests looked at other things, which were fine.
      An evaluation only sees what it measures.
      Each known risk deserves its own test.

  - id: reponse-4
    sous_rubrique: reponse
    question: "Your agent answers correctly, but cites a source that doesn't say that. A good answer?"
    choix:
      - "Yes, nobody checks sources"
      - "Yes, the conclusion is right"
      - "No: the grounding is wrong, even if the conclusion is right"
      - "It depends on the model"
    reponse: 2
    pourquoi: >-
      The user trusts the citation.
      A false citation misleads, even when right by chance.
      Tomorrow, the same mechanism will give a wrong conclusion.
      Grounding is evaluated apart from accuracy.

  - id: reponse-5
    sous_rubrique: reponse
    question: "Should you also evaluate the cases where the agent should refuse or say “I don't know”?"
    choix:
      - "No, only normal cases"
      - "Yes: a good test set holds questions with no answer and requests to refuse"
      - "No, it never happens"
      - "Only in production"
    reponse: 1
    pourquoi: >-
      An agent that always answers looks good on easy cases.
      It makes things up on cases with no answer.
      Without those cases in the test, abstention is never measured.
      That is the topic of step 3.

  - id: trajectoire-1
    sous_rubrique: trajectoire
    question: "What is an agent's trajectory?"
    choix:
      - "The price of the request"
      - "Its final answer"
      - "The response time"
      - "The sequence of its steps: tools called, arguments, results, decisions"
    reponse: 3
    pourquoi: >-
      Two agents can give the same answer.
      One in 3 calls, the other in 40.
      One following the rules, the other getting around them.
      The trajectory shows how it got there.

  - id: trajectoire-2
    sous_rubrique: trajectoire
    question: "An agent refunds the right amount, but without checking the customer's identity as the rule requires. Should the test pass?"
    choix:
      - "Yes, if the customer is happy"
      - "Yes, the result is right"
      - "No: the trajectory breaks a rule, even if the result is right"
      - "It depends on the amount"
    reponse: 2
    pourquoi: >-
      This time, it was the right customer.
      Next time, it will be a fraudster.
      The result alone hides the risk.
      The trajectory reveals it.

  - id: trajectoire-3
    sous_rubrique: trajectoire
    question: "How does τ-bench (2024) judge whether a customer service agent succeeded?"
    choix:
      - "By comparing the final state of the database with the expected state"
      - "By reading the final answer by eye"
      - "By measuring the length of the conversation"
      - "By asking the model whether it succeeded"
    reponse: 0
    pourquoi: >-
      What matters is what changed in the system.
      Is the order cancelled? The right item exchanged?
      The final state can be compared objectively.
      The agent's own account may embellish.

  - id: trajectoire-4
    sous_rubrique: trajectoire
    question: "What does pass^k measure in τ-bench?"
    choix:
      - "The number of tools used"
      - "The best result over k tries"
      - "The probability of succeeding at the same task all k times out of k tries"
      - "The cost of k requests"
    reponse: 2
    pourquoi: >-
      In production, a task repeats thousands of times.
      Succeeding once isn't enough.
      pass^k asks for success every time.
      In the retail domain, pass^8 fell below 25% for the best agents tested.

  - id: trajectoire-5
    sous_rubrique: trajectoire
    question: "Which trajectory metrics should you track on top of success?"
    choix:
      - "Number of steps, useless or repeated calls, tool errors, cost, duration"
      - "Only the color of the interface"
      - "Only the number of words"
      - "None"
    reponse: 0
    pourquoi: >-
      An agent that succeeds in 40 steps costs 10 times more than in 4.
      Repeated calls signal a loop.
      Tool errors signal a misunderstood schema.
      These numbers guide the fixes.

  - id: trajectoire-6
    sous_rubrique: trajectoire
    question: "For a research agent, does Anthropic consider it must follow a single path set in advance?"
    choix:
      - "Anthropic doesn't evaluate its agents"
      - "Yes, always the same exact path"
      - "Yes, otherwise the test fails"
      - "No: different paths can be valid. You judge the result and whether the process was followed"
    reponse: 3
    pourquoi: >-
      Two agents can find the same information through different sources.
      Requiring an exact path punishes valid solutions.
      You check the mandatory steps and the forbidden ones instead.
      The rest is free.
---

## The answer {#reponse}

You evaluate first what the user receives.

| Aspect | Question | How |
|---|---|---|
| Accuracy | Is it right? | Code if possible, otherwise a judge |
| Grounding | Do the sources say it? | Judge, citation check |
| Completeness | Is everything covered? | List of expected points |
| Format | Valid JSON, fields present? | Code |
| Abstention | Does it refuse when it should? | Cases with no answer in the test set |
| Behavior | Tone, sycophancy, safety | Judge, dedicated tests |

:::pourquoi
An answer can pass one aspect and fail the others.
An overall score hides what breaks.
:::

What can be checked by **code** is checked by code.

:::pourquoi
An amount, a format, a field: exact comparison.
Fast, free, deterministic.
The LLM judge is for the rest.
:::

Each known risk has **its own** test.

:::pourquoi
GPT-4o's tests were good.
None measured sycophancy.
An evaluation only sees what it measures.
:::

## The trajectory {#trajectoire}

The **trajectory** is the path: tools called, arguments, results, decisions.

```mermaid
sequenceDiagram
  participant U as Simulated user
  participant A as Agent
  participant T as Tools
  participant B as Database
  participant E as Evaluator
  U->>A: “Cancel my order 512”
  A->>T: verify_identity(customer)
  A->>T: cancel_order(512)
  T->>B: Order 512: cancelled
  A-->>U: “Done”
  E->>B: Final state = expected state?
  E->>E: Mandatory steps present? Forbidden ones avoided?
```

A good result isn't enough.

:::pourquoi
A correct refund without an identity check passes today.
Tomorrow, a fraudster takes advantage of it.
:::

τ-bench (2024) simulates a customer with a model. It judges the agent on the **final state** of the database, compared with the expected state.

:::pourquoi
What matters is what changed in the system.
The agent can say “done” without having done it.
:::

Measure **consistency**: pass^k, succeeding at the same task k times out of k.

:::pourquoi
In τ-bench, the best agents succeeded at less than half the tasks.
In the retail domain, pass^8 fell below 25%.
In production, the same task comes back thousands of times.
:::

Also track: number of steps, repeated calls, tool errors, cost, duration.

:::pourquoi
They explain why an agent fails, or succeeds too expensively.
:::

Don't freeze a single path when several are valid.

:::pourquoi
Anthropic notes it for its research agents.
Judging the result and the process rules avoids punishing a good, different solution.
:::

## Key points

- Evaluate the answer by aspect: accuracy, grounding, completeness, format, abstention, behavior.
- Code whenever possible. One test per known risk.
- Evaluate the trajectory: final state, mandatory steps, forbidden ones.
- Measure consistency (pass^k), not only the best try.
