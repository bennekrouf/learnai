---
id: "00-02"
titre: "Generation"
sous_rubriques:
  - id: suivant
    titre: "Predicting the next token"
  - id: temperature
    titre: "Temperature and top-p"
  - id: determinisme
    titre: "Determinism"
  - id: structure
    titre: "Structured outputs"
sources:
  - titre: "Holtzman et al. — The Curious Case of Neural Text Degeneration (top-p), 2019"
    url: "https://arxiv.org/abs/1904.09751"
  - titre: "Horace He, Thinking Machines Lab — Defeating Nondeterminism in LLM Inference, September 2025 (summary by Simon Willison)"
    url: "https://simonwillison.net/2025/Sep/11/defeating-nondeterminism"
qcm:
  - id: suivant-1
    sous_rubrique: suivant
    question: "At each step, what does the model compute?"
    choix:
      - "The whole sentence at once"
      - "The most accurate word according to its sources"
      - "A probability for every token in the vocabulary"
      - "A summary of the question"
    reponse: 2
    pourquoi: >-
      The model outputs a list of probabilities.
      One per possible token, often more than 100,000.
      One is picked. It is added to the text.
      Then it starts again for the next one.

  - id: suivant-2
    sous_rubrique: suivant
    question: "Why can an LLM state something false with confidence?"
    choix:
      - "It lies on purpose"
      - "It produces the likely text, not the true text"
      - "Its temperature is too low"
      - "It runs out of tokens"
    reponse: 1
    pourquoi: >-
      The model learned what plausible text looks like.
      It has no mechanism to check a fact.
      A fake reference can be very likely.
      The confident tone comes from the style it learned, not from a check.
      That is the topic of step 3.

  - id: temperature-1
    sous_rubrique: temperature
    question: "A temperature close to 0 gives…"
    choix:
      - "More creative answers"
      - "Shorter answers"
      - "More predictable answers"
      - "Faster answers"
    reponse: 2
    pourquoi: >-
      Temperature stretches or squashes the probabilities.
      Low, it boosts the most likely token.
      That token is picked almost every time.
      High, it gives rare tokens more of a chance.

  - id: temperature-2
    sous_rubrique: temperature
    question: "Top-p at 0.9 means…"
    choix:
      - "Sampling among the most likely tokens, up to 90% cumulative probability"
      - "Keeping the 90% most frequent tokens of the vocabulary"
      - "The answer is 90% right"
      - "Cutting the answer at 90% of max_tokens"
    reponse: 0
    pourquoi: >-
      The tokens are sorted from most to least likely.
      Their probabilities are added up until they reach 0.9.
      The draw happens only within that group.
      The absurd tokens in the long tail are left out.

  - id: temperature-3
    sous_rubrique: temperature
    question: "You extract the amount and date from invoices. Which setting?"
    choix:
      - "It doesn't matter"
      - "High temperature"
      - "Top-p at 1 and high temperature"
      - "Low temperature"
    reponse: 3
    pourquoi: >-
      An extraction has a single right answer.
      Variety only brings errors.
      High temperature is for creative tasks.

  - id: temperature-4
    sous_rubrique: temperature
    question: "Should you tune temperature and top-p at the same time?"
    choix:
      - "Yes, always both"
      - "Usually you tune only one"
      - "No, top-p is obsolete"
      - "Yes, or the API refuses"
    reponse: 1
    pourquoi: >-
      Both act on the same draw.
      Combining them makes the effect hard to predict.
      Providers advise changing only one.

  - id: determinisme-1
    sous_rubrique: determinisme
    question: "Temperature 0, same prompt, two calls. Are the answers identical?"
    choix:
      - "No, not guaranteed"
      - "Yes, guaranteed"
      - "Yes, if the prompt is short"
      - "No, the model changes every day"
    reponse: 0
    pourquoi: >-
      The server groups your request with others, in a batch.
      The batch size depends on the load at that moment.
      It changes the order of some computations.
      A tiny gap can flip a token.
      Everything after it then changes.

  - id: determinisme-2
    sous_rubrique: determinisme
    question: "What does this mean in practice for your tests?"
    choix:
      - "Compare the exact text of the answer"
      - "Turn off tests on LLMs"
      - "Check properties of the answer, not the exact text"
      - "Always set the temperature to 2"
    reponse: 2
    pourquoi: >-
      A test on the exact text will break for no reason.
      Check the substance instead: a field present, a value, a citation.
      That is the topic of step 7.

  - id: structure-1
    sous_rubrique: structure
    question: "You write “Answer in JSON” in the prompt. Is the JSON guaranteed?"
    choix:
      - "Yes"
      - "No, you need a structured output constrained by a schema"
      - "Yes, if the temperature is 0"
      - "No, LLMs can't write JSON"
    reponse: 1
    pourquoi: >-
      An instruction is a request, not a constraint.
      The model may add a sentence or forget a brace.
      With a constrained output, the API only allows valid tokens.
      The schema is then followed by construction.

  - id: structure-2
    sous_rubrique: structure
    question: "Is JSON that is valid against the schema necessarily right?"
    choix:
      - "Yes, the schema checked it"
      - "No, the schema is ignored"
      - "Yes, if every field is filled in"
      - "No, the shape is right, the content can be wrong"
    reponse: 3
    pourquoi: >-
      The schema checks types and fields.
      It doesn't check that the date is the right one.
      A made-up amount is still a valid number.
      You also have to validate the values.

  - id: structure-3
    sous_rubrique: structure
    question: "The answer stops in the middle of a JSON object. Most likely cause?"
    choix:
      - "The model crashed"
      - "The JSON was too complex"
      - "The temperature was too low"
      - "The max_tokens limit was reached"
    reponse: 3
    pourquoi: >-
      max_tokens cuts the output dead, even mid-word.
      The API gives the reason it stopped.
      Check it before reading the answer.
      If it is max_tokens, the answer is incomplete.
---

## Predicting the next token {#suivant}

An LLM does one thing: predict the next token.

At each step, it gives a probability to every token in the vocabulary.

One is picked. It is added. Start again.

:::pourquoi
That is how it was trained: guessing the next word over billions of texts.
Everything else follows from it: writing, summarizing, coding.
:::

The model produces the **likely** text, not the **true** text.

:::pourquoi
Nothing in this mechanism checks a fact.
A well-formed fake quote can be very likely.
Hence hallucinations, in a perfectly confident tone.
:::

## Temperature and top-p {#temperature}

These two settings decide **how** a token is picked from the probabilities.

**Temperature**: low, the most likely token almost always wins. High, rare tokens get a chance.

:::pourquoi
Temperature divides the scores before the probabilities are computed.
Small, it widens the gaps. Large, it flattens them.
:::

**Top-p**: keep the most likely tokens up to a given total, say 90%. Draw among them.

:::pourquoi
It cuts off the long tail of absurd tokens.
The group kept adapts: small when the model is sure, wide when it hesitates.
:::

Simple rule: a task with one right answer → low temperature. A creative task → higher. Tune only one of the two.

## Determinism {#determinisme}

Temperature 0 doesn't guarantee the same answer twice.

:::pourquoi
Your request is computed in a batch with other customers' requests.
The batch size changes with the server's load.
It changes the order of some computations, and so the rounding.
A token can flip. Everything after it diverges.
Thinking Machines showed this in 2025, and proposed computations that no longer depend on the batch.
:::

In practice: test properties, not an exact text.

## Structured outputs {#structure}

Asking for JSON in the prompt doesn't guarantee it.

A **structured output** gives the API a schema. It only allows tokens that respect that schema.

:::pourquoi
At each step, tokens that would break the schema are forbidden.
The model can no longer step outside the frame.
:::

The schema guarantees the shape. Not the content.

:::pourquoi
A made-up amount is still a valid number.
Always validate the values, not just the structure.
:::

Always check the stop reason. `max_tokens` cuts the answer dead.

:::pourquoi
A truncated answer looks like a normal one.
Only the stop reason tells you the end is missing.
:::

## Key points

- An LLM predicts the next token. Likely isn't true.
- Low temperature for tasks with one right answer.
- Temperature 0 isn't deterministic in practice.
- The schema guarantees the shape, never the substance.
