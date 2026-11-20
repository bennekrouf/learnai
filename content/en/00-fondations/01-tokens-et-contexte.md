---
id: "00-01"
titre: "Tokens and context"
sous_rubriques:
  - id: tokens
    titre: "What a token is"
  - id: fenetre
    titre: "The context window"
  - id: cout
    titre: "Input, output and cost"
sources:
  - titre: "Rumbelow & Watkins — SolidGoldMagikarp (plus, prompt generation), SERI-MATS, 2023"
    url: "https://www.greaterwrong.com/posts/aPeJE8bSo6rAFoLqg/"
qcm:
  - id: tokens-1
    sous_rubrique: tokens
    question: "You type “strawberry”. What does the model actually receive?"
    choix:
      - "Ten letters, one at a time"
      - "An image of the word"
      - "A sequence of token numbers"
      - "One vector per letter"
    reponse: 2
    pourquoi: >-
      The text first goes through a tokenizer.
      It cuts the text into frequent chunks.
      Each chunk becomes a number.
      The model only sees those numbers.
      The letters are gone before it even starts.

  - id: tokens-2
    sous_rubrique: tokens
    question: "The same sentence, in English and in French. Which one usually produces more tokens?"
    choix:
      - "French"
      - "English"
      - "No difference"
      - "It depends only on the number of characters"
    reponse: 0
    pourquoi: >-
      Tokenizers are built mostly on English text.
      A common English word often fits in one token.
      A French word is more often split into several.
      Same meaning, more tokens.
      So it costs more and takes more room in the context.

  - id: tokens-3
    sous_rubrique: tokens
    question: "In English, 1,000 tokens are roughly…"
    choix:
      - "250 words"
      - "4,000 words"
      - "1,000 words"
      - "750 words"
    reponse: 3
    pourquoi: >-
      A token is about 4 characters of English on average.
      That is about three quarters of a word.
      It is an order of magnitude, not a rule.
      Most other languages fit fewer words into 1,000 tokens.

  - id: tokens-4
    sous_rubrique: tokens
    question: "In 2023, the word “SolidGoldMagikarp” makes GPT-3 go off the rails. Why?"
    choix:
      - "OpenAI banned the word"
      - "It is a token in the vocabulary that was almost never seen in training"
      - "It is too long for the model"
      - "It hides a prompt injection"
    reponse: 1
    pourquoi: >-
      The token vocabulary was built on one corpus.
      The model was trained on another.
      This username was frequent in the first and nearly absent from the second.
      So the model has a token it never learned to use.
      Faced with it, it produces nonsense.

  - id: fenetre-1
    sous_rubrique: fenetre
    question: "The context window limits…"
    choix:
      - "The prompt and the answer together"
      - "Only the prompt"
      - "Only the answer"
      - "The number of messages"
    reponse: 0
    pourquoi: >-
      The model writes token by token.
      Each token written is added to the context to write the next one.
      So the answer sits in the same window as the prompt.
      A prompt that fills almost everything leaves little room to answer.

  - id: fenetre-2
    sous_rubrique: fenetre
    question: "Does the model remember yesterday's conversation?"
    choix:
      - "Yes, it keeps a memory per user"
      - "Yes, if it was trained since"
      - "No, the application sends the history again on every call"
      - "No, unless the temperature is 0"
    reponse: 2
    pourquoi: >-
      The model keeps no state between two calls.
      Each call starts from scratch.
      The feeling of memory comes from the application.
      It sends the whole history with every message.
      Without it, the model knows nothing of what came before.

  - id: fenetre-3
    sous_rubrique: fenetre
    question: "Your prompt is larger than the context window. What does the API do?"
    choix:
      - "It reads the start and ignores the rest"
      - "It rejects the request with an error"
      - "It summarizes the overflow on its own"
      - "It switches to a larger model"
    reponse: 1
    pourquoi: >-
      The API doesn't guess what to cut.
      It returns an error.
      Choosing what to keep is your application's job.
      Some chat interfaces cut silently. The API doesn't.
      That is the topic of step 1.

  - id: cout-1
    sous_rubrique: cout
    question: "Which token costs the most?"
    choix:
      - "An input token"
      - "The first token of the prompt"
      - "They cost the same"
      - "An output token"
    reponse: 3
    pourquoi: >-
      The input is processed in one block, in parallel.
      The output is produced one token at a time.
      Each output token takes a full pass through the model.
      More compute, so a price several times higher.

  - id: cout-2
    sous_rubrique: cout
    question: "Your answer takes 20 seconds. Which lever does the most for latency?"
    choix:
      - "Writing the prompt in English"
      - "Shortening the prompt"
      - "Lowering the temperature"
      - "Asking for a shorter answer"
    reponse: 3
    pourquoi: >-
      The output is written token by token.
      So the total time follows the length of the answer.
      A shorter prompt helps, but much less.
      Temperature doesn't change the speed.

  - id: cout-3
    sous_rubrique: cout
    question: "A conversation costs a little more with each new message. Why?"
    choix:
      - "The whole history is sent, and billed, on every call"
      - "The provider raises its prices with usage"
      - "The model thinks for longer"
      - "Older tokens cost more"
    reponse: 0
    pourquoi: >-
      The model has no memory.
      The application sends the whole conversation on every turn.
      Each message adds input tokens to every call that follows.
      The cost of a turn grows with the length of the conversation.
---

## What a token is {#tokens}

A model doesn't read letters. It reads tokens.

A token is a frequent chunk of text. A word, part of a word, a space followed by a word.

:::pourquoi
Working letter by letter would make very long sequences.
Working word by word would need an endless vocabulary.
The token is the trade-off: a fixed vocabulary, short sequences.
:::

The **tokenizer** does the cutting. It turns each chunk into a number.

```mermaid
flowchart LR
  T["Text: strawberry"] --> K["Tokenizer"]
  K --> N["Token numbers"]
  N --> M["Model"]
  M --> S["Output numbers"]
  S --> K2["Tokenizer (reverse)"]
  K2 --> R["Answer text"]
```

The model never sees the text. Only the numbers.

:::pourquoi
That is why it misses “how many r's in strawberry”.
The letter information was erased before it got there.
To count letters, you need a tool that sees letters.
:::

Order of magnitude: in English, 1 token ≈ 4 characters ≈ ¾ of a word.

Most other languages, French among them, take more tokens than English for the same meaning.

:::pourquoi
The vocabulary is learned on a mostly English corpus.
Frequent English words get a token of their own.
Words in other languages are split more often.
:::

The vocabulary can hold tokens that were almost never seen in training. In 2023, researchers find some in GPT-3: “SolidGoldMagikarp” is the best known. These tokens make it go off the rails.

:::pourquoi
The vocabulary and the model didn't learn from the same data.
A token present in one and missing from the other has no learned meaning.
:::

## The context window {#fenetre}

The context window is everything the model sees at once.

It is counted in tokens.

It holds **the prompt and the answer**.

:::pourquoi
The model writes a token, adds it to the context, then writes the next one.
So the answer grows in the same window as the question.
:::

The model has no memory between two calls.

```mermaid
sequenceDiagram
  participant U as User
  participant A as Application
  participant L as LLM
  U->>A: Message 1
  A->>L: Message 1
  L-->>A: Answer 1
  U->>A: Message 2
  A->>L: Message 1 + Answer 1 + Message 2
  L-->>A: Answer 2
```

:::pourquoi
Each call stands alone.
The application is what sends the history again.
A chat's “memory” is an application choice, not a property of the model.
:::

If the content is larger than the window, the API returns an error.

:::pourquoi
The API doesn't know what matters to you.
It refuses rather than cut at random.
Deciding what to keep is your job. That is step 1.
:::

## Input, output and cost {#cout}

APIs bill per million tokens. Input and output have two different prices.

An output token costs several times more than an input token.

:::pourquoi
The input is read in a single block, in parallel.
The output is written one token at a time.
Each token written takes a full pass through the model.
:::

The length of the answer drives the latency.

:::pourquoi
Same reasons: the output is sequential.
An answer twice as long takes about twice as long.
:::

A long conversation costs more and more with each turn.

:::pourquoi
The whole history is sent on every call.
It is billed every time.
:::

## Key points

- The model sees token numbers, not letters.
- The window holds the question **and** the answer.
- The model has no memory. The application fakes it.
- Output costs more and drives the latency.
