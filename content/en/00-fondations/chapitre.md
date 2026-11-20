---
etape: 0
titre: "LLM foundations"
accroche: "The model that writes code can't count the r's in “strawberry”."
sources:
  - titre: "OpenAI Community — Incorrect count of 'r' characters in the word strawberry"
    url: "https://community.openai.com/t/incorrect-count-of-r-characters-in-the-word-strawberry/829618"
  - titre: "How many Rs are in the word strawberry? (a GPT-4o test, August 2024)"
    url: "https://blog.legoktm.com/2024/08/10/how-many-rs-are-in-the-word-strawberry.html"
  - titre: "Reuters (via The Star) — OpenAI working on new reasoning technology under code name ‘Strawberry’, July 13, 2024"
    url: "https://www.thestar.com.my/tech/tech-news/2024/07/13/exclusive-openai-working-on-new-reasoning-technology-under-code-name-strawberry"
---

## Three r's

Summer 2024. One question goes around the internet.

“How many r's are in *strawberry*?”

GPT-4o answers: two. There are three.

People push back. It sticks to two.

They ask it to write a small Python program that counts. The program answers: three.

The same model summarizes contracts. It writes code. It translates ten languages.

But it can't count three letters.

## The irony

At the same time, Reuters reveals a secret OpenAI project.

Its goal: make models reason.

Its code name: *Strawberry*.

It ships in September 2024, as o1.

## What really happened

The model isn't “dumb”. It is blind to letters.

It doesn't read characters. It reads **tokens**: chunks of words, turned into numbers.

To the model, *strawberry* is one block, or a few. Not ten letters.

Counting the r's means asking it to see what was hidden from it.

:::pourquoi
The Python program gets the word letter by letter.
The model didn't count any better. It handed the job to a tool that sees letters.
That is exactly the idea behind tool calling, at the end of this chapter.
:::

## In this chapter

- **Tokens and context**: what the model sees, how much it can see, what it costs.
- **Generation**: how it picks each word, and why two answers differ.
- **Embeddings**: how a text becomes a position in a space of meaning.
- **Tool calling**: how the model asks code for help.

The rest of the course rests on these four ideas.
