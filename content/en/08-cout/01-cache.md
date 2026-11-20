---
id: "08-01"
titre: "Caching"
sous_rubriques:
  - id: cache-prompt
    titre: "Prompt caching"
  - id: cache-semantique
    titre: "Semantic caching and its risks"
sources:
  - titre: "Anthropic — Prompt caching (documentation)"
    url: "https://platform.claude.com/docs/en/build-with-claude/prompt-caching"
  - titre: "OpenAI — Prompt caching (documentation)"
    url: "https://developers.openai.com/api/docs/guides/prompt-caching"
  - titre: "Gu et al. — Auditing Prompt Caching in Language Model APIs, ICML 2025"
    url: "https://arxiv.org/abs/2502.07776"
qcm:
  - id: cache-prompt-1
    sous_rubrique: cache-prompt
    question: "What does a prompt cache reuse?"
    choix:
      - "The model's weights"
      - "The final answer to a question already asked"
      - "The user's files"
      - "The computation already done on an identical prompt prefix"
    reponse: 3
    pourquoi: >-
      The provider keeps the computation done on the start of the prompt.
      If the next request starts the same way, it picks it up.
      The answer itself is generated again.
      You pay less and wait less.

  - id: cache-prompt-2
    sous_rubrique: cache-prompt
    question: "Prompt caching works on an exact prefix. Where do you put what changes with each request?"
    choix:
      - "At the very start of the prompt"
      - "At the end of the prompt, after everything stable"
      - "In the middle, between the tools and the instructions"
      - "The order doesn't matter"
    reponse: 1
    pourquoi: >-
      The cache stops at the first different character.
      A date at the top of the prompt breaks everything after it.
      Stable first: tools, instructions, documents.
      Variable next: the user's question.

  - id: cache-prompt-3
    sous_rubrique: cache-prompt
    question: "Your system prompt starts with “Today is November 19, 2026, 14:32:07”. Effect on the cache?"
    choix:
      - "The prefix changes every second: the cache is almost never used"
      - "No effect"
      - "The cache gets faster"
      - "The model ignores that line"
    reponse: 0
    pourquoi: >-
      Each request has a different start.
      No prefix repeats.
      You pay for writing the cache without ever reading it.
      Put the time at the end, or only to the day.

  - id: cache-prompt-4
    sous_rubrique: cache-prompt
    question: "At Anthropic, writing to the cache costs 1.25 times the normal price, and reading it 0.1 times, for most models. The same prefix is used twice. Is it worth it?"
    choix:
      - "Only from 100 uses"
      - "No, writing costs too much"
      - "Yes: 1.25 + 0.1 = 1.35, against 2 without a cache"
      - "Impossible to say"
    reponse: 2
    pourquoi: >-
      Without a cache: full price twice.
      With a cache: a slightly dearer write, then an almost free read.
      The gain starts from the second use.
      It grows with each reuse.

  - id: cache-prompt-5
    sous_rubrique: cache-prompt
    question: "Anthropic's default cache lasts 5 minutes. What happens if it is read before then?"
    choix:
      - "It still expires after 5 minutes"
      - "Its lifetime restarts, at no extra cost"
      - "It is billed twice"
      - "It becomes permanent"
    reponse: 1
    pourquoi: >-
      Each read refreshes the cache.
      Steady traffic keeps it warm.
      For spaced-out requests, a one-hour lifetime exists.
      It costs more to write.

  - id: cache-prompt-6
    sous_rubrique: cache-prompt
    question: "Why does a 2025 study warn against a prompt cache shared between users?"
    choix:
      - "The cache costs more"
      - "The cache erases prompts"
      - "The cache changes the model's answer"
      - "A shorter response time reveals that someone else already sent the same prompt prefix"
    reponse: 3
    pourquoi: >-
      A cached prefix is processed faster.
      An attacker can try a prompt prefix and measure the time.
      Gu et al. found seven providers with a cache shared between users.
      A performance gain can become a leak.

  - id: cache-semantique-1
    sous_rubrique: cache-semantique
    question: "How does a semantic cache work?"
    choix:
      - "It reuses the computation of the prompt prefix"
      - "It only keeps questions identical to the character"
      - "It returns an answer already produced if the new question is close enough in meaning to an old one"
      - "It compresses the answers"
    reponse: 2
    pourquoi: >-
      The question becomes a vector.
      You look for a close question already asked.
      Above a similarity threshold, you return its answer.
      The model isn't called at all.

  - id: cache-semantique-2
    sous_rubrique: cache-semantique
    question: "“How do I enable two-factor authentication?” is cached. Then comes “How do I disable two-factor authentication?”. Risk?"
    choix:
      - "The two sentences are very close: the cache may return the wrong answer"
      - "None, the cache sees the difference"
      - "The model fixes it on its own"
      - "The question is rejected"
    reponse: 0
    pourquoi: >-
      A single word changes. The meaning is reversed.
      The vectors stay very close.
      It is a false positive: a right answer for another question.
      A close vector doesn't mean the same meaning.

  - id: cache-semantique-3
    sous_rubrique: cache-semantique
    question: "A user asks “What is my balance?”. The answer goes into a global semantic cache. Problem?"
    choix:
      - "The balance will be rounded"
      - "None, it's the same question"
      - "Another user asking the same question gets the first user's balance"
      - "The answer will be slower"
    reponse: 2
    pourquoi: >-
      The question is the same. The right answer depends on who asks.
      A global cache mixes users up.
      It is a personal data leak.
      What depends on the user doesn't go into a shared cache.

  - id: cache-semantique-4
    sous_rubrique: cache-semantique
    question: "Which answers suit a semantic cache best?"
    choix:
      - "General, stable answers, with no personal data and no action"
      - "Answers that depend on the user"
      - "Answers that trigger a refund"
      - "Answers about data that changes every minute"
    reponse: 0
    pourquoi: >-
      “What are your opening hours?”: same answer for everyone.
      “Refund my order”: an action, never replayed from a cache.
      “Current price”: stale in a minute.
      The cache serves frequent, fixed questions.

  - id: cache-semantique-5
    sous_rubrique: cache-semantique
    question: "How do you set the similarity threshold of a semantic cache?"
    choix:
      - "Once and for all, never revisited"
      - "As low as possible to maximize reuse"
      - "At random"
      - "On a set of labeled question pairs, measuring the false positives"
    reponse: 3
    pourquoi: >-
      Low threshold: lots of reuse, lots of errors.
      High threshold: few errors, few savings.
      Only real pairs tell you where to set the bar.
      It is an evaluation, as in step 7.

  - id: cache-semantique-6
    sous_rubrique: cache-semantique
    question: "How do you limit the risks of a semantic cache?"
    choix:
      - "A single, everlasting cache for everyone"
      - "A cache per user or per customer, a short lifetime, and a way to purge a wrong entry"
      - "No rules, the model handles it"
      - "Turn off the logs"
    reponse: 1
    pourquoi: >-
      Partitioning avoids leaks between users.
      The lifetime avoids stale answers.
      Purging fixes a mistake without waiting.
      A wrong answer in the cache repeats on every visit.
---

## Prompt caching {#cache-prompt}

Many requests start the same way: same tools, same instructions, same documents.

The **prompt cache** keeps the computation done on that start. The next request picks it up.

:::pourquoi
The start of the prompt is often the longest part.
Recomputing it each time costs time and money.
The answer is still generated afresh.
:::

```mermaid
sequenceDiagram
  participant A as Application
  participant F as Provider API
  participant C as Cache
  participant M as Model
  A->>F: Tools + instructions + question 1
  F->>C: Known prefix?
  C-->>F: No
  F->>M: Full computation
  F->>C: Writes the prefix (1.25x)
  A->>F: Same tools + instructions + question 2
  F->>C: Known prefix?
  C-->>F: Yes (0.1x)
  F->>M: Computes question 2 only
```

The cache works on an **exact prefix**. Stable first, variable at the end.

:::pourquoi
The cache stops at the first different character.
At Anthropic, the order is: tools, then system, then messages.
Changing a tool invalidates everything after it.
:::

| Position | Content |
|---|---|
| Start | Tools, instructions, reference documents |
| Middle | Conversation history |
| End | The current question, date, time |

Prices at Anthropic, for most models:

| Operation | Price compared with the normal price |
|---|---|
| Write, 5-minute cache | 1.25x |
| Write, one-hour cache | 2x |
| Read | 0.1x |

:::pourquoi
Two calls without a cache: 2.
With a cache: 1.25 + 0.1 = 1.35.
The gain starts from the second use.
:::

Each read **refreshes** the 5-minute cache. Prompts that are too short aren't cached.

:::pourquoi
Steady traffic keeps the cache warm.
Below a minimum length, the gain isn't worth the bookkeeping.
The minimum length depends on the model.
:::

At OpenAI, caching is **automatic** above a minimum length. The discount depends on the model.

:::pourquoi
Nothing to turn on. But the same rule applies:
a stable prefix, or no discount.
The `cached_tokens` field in the response says what was reused.
:::

A cache shared between users can **leak** through response time.

:::pourquoi
A cached prefix answers faster.
An attacker can try a prompt prefix and time it.
In 2025, Gu et al. detected a cache shared between users at seven providers.
:::

## Semantic caching and its risks {#cache-semantique}

The **semantic cache** goes further. It keeps whole answers.

If a question is close enough to an old one, it returns the old answer. Without calling the model.

```mermaid
sequenceDiagram
  participant U as User
  participant A as Application
  participant S as Semantic cache
  participant M as Model
  U->>A: “What are your opening hours?”
  A->>S: Close question already seen? (similarity)
  alt Similarity above the threshold
    S-->>A: Cached answer
  else Otherwise
    A->>M: Question
    M-->>A: Answer
    A->>S: Stores question + answer
  end
  A-->>U: Answer
```

:::pourquoi
No model call: almost zero cost, immediate answer.
Useful for frequent questions, like an FAQ.
:::

First risk: the **false positive**. A close question isn't the same question.

| Cached question | New question | Problem |
|---|---|---|
| How do I enable 2FA? | How do I disable 2FA? | Opposite meaning |
| Status of order 512? | Status of order 513? | Another order |
| Price in 2025? | Price in 2026? | Another year |

:::pourquoi
The vectors of these sentences are very close.
A single word changes the meaning.
The cache doesn't read: it measures a distance.
:::

Second risk: the **leak**. “What is my balance?” doesn't have the same answer for everyone.

:::pourquoi
A global cache returns the first user's answer to the next.
It is a personal data leak.
:::

Third risk: the **stale** answer. A price, a stock level or a status changes.

:::pourquoi
The cached answer was right yesterday.
It is wrong today, and the cache doesn't know.
:::

The rules:

- Only cache what is **general and stable**: no personal data, no action.
- **Partition** by user or by customer.
- Set a **lifetime** and be able to **purge** an entry.
- Set the **threshold** on labeled question pairs.

:::pourquoi
A wrong answer in the cache repeats on every visit.
The threshold is set like an evaluation: by counting errors.
:::

## Key points

- Prompt caching: stable first, variable at the end. Pays off from the second use.
- No timestamp at the top of the prompt.
- Semantic caching: fast and free, but false positives, leaks and stale answers.
- Partition, limit in time, set the threshold on real data.
