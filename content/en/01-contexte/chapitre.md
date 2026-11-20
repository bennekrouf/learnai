---
etape: 1
titre: "Mastering the context"
accroche: "One sentence added to the prompt takes a model from 27% to 98%."
sources:
  - titre: "Anthropic — Long context prompting for Claude 2.1, December 6, 2023"
    url: "https://claude.com/blog/claude-2-1-prompting"
  - titre: "Greg Kamradt — LLMTest_NeedleInAHaystack"
    url: "https://github.com/gkamradt/LLMTest_NeedleInAHaystack"
  - titre: "Arize — The Needle In a Haystack Test (a reproduction attempt)"
    url: "https://arize.com/blog/the-needle-in-a-haystack-test-evaluating-the-performance-of-llm-rag-systems/"
---

## A needle in a haystack

November 2023. Anthropic releases Claude 2.1.

Its context window: 200,000 tokens. A whole book.

A developer, Greg Kamradt, wants to know if the model really reads all of it.

He takes dozens of Paul Graham's essays on startups.

Somewhere in the middle, he hides an unrelated sentence:

“The best thing to do in San Francisco is eat a sandwich and sit in Dolores Park on a sunny day.”

Then he asks the model: what is the best thing to do in San Francisco?

Often, the model doesn't find it.

## One sentence changes everything

December 2023. Anthropic responds.

The model isn't blind. It is wary.

A sentence about a sandwich, in the middle of essays on startups, looks off-topic.

It learned not to answer on the strength of a doubtful passage.

The fix is one sentence. It is written at the start of the model's answer:

“Here is the most relevant sentence in the context:”

Score before: 27%. Score after: 98%.

:::pourquoi
The sentence forces the model to search first, then answer.
It no longer decides on its own whether the passage is reliable.
A team at Arize didn't get 98%. But they saw failures drop from 165 to 74.
The effect is real, even if the exact figure varies.
:::

## The lesson

A large window doesn't mean everything in it is read the same way.

**What you put** in the context matters.

**Where you put it** matters.

**How you introduce it** matters.

Assembling the context is engineering work. Not copy-paste.

## In this chapter

- **Token budget**: splitting the window, and keeping room to answer.
- **Assembly by priority**: what to keep, what to cut, where to place each piece.
- **Three-tier memory**: the task at hand, what happened, what is known.
- **Memory lifecycle**: what deserves remembering, and when to forget.
