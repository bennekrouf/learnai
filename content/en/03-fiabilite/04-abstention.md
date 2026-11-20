---
id: "03-04"
titre: "Abstention"
sous_rubriques:
  - id: seuils
    titre: "Confidence thresholds"
  - id: je-ne-sais-pas
    titre: "Saying “I don't know”"
sources:
  - titre: "OpenAI (Kalai, Nachum, Vempala et al.) — Why language models hallucinate, September 5, 2025"
    url: "https://openai.com/index/why-language-models-hallucinate/"
  - titre: "Anthropic — Reduce hallucinations (documentation)"
    url: "https://platform.claude.com/docs/en/test-and-evaluate/strengthen-guardrails/reduce-hallucinations"
qcm:
  - id: seuils-1
    sous_rubrique: seuils
    question: "Which signals can decide an abstention?"
    choix:
      - "The best document's score, the reranker's score, the checker's verdict"
      - "The length of the question"
      - "The time of the request"
      - "The user's language"
    reponse: 0
    pourquoi: >-
      A weak best document: the corpus may not hold the answer.
      A reranker that finds everything barely relevant: same conclusion.
      A checker that finds unsupported claims: the answer is fragile.
      These signals are combined to decide.

  - id: seuils-2
    sous_rubrique: seuils
    question: "The model writes “I'm 95% sure”. Can you use it as a threshold?"
    choix:
      - "Yes, it is an exact probability"
      - "No, models don't talk about confidence"
      - "Yes, if the temperature is 0"
      - "With caution: the confidence a model expresses in words is often poorly calibrated"
    reponse: 3
    pourquoi: >-
      “95%” is generated text, like the rest.
      It doesn't come from a reliability computation.
      Prefer measurable signals: search scores, source checks.
      And check their calibration on your test set.

  - id: seuils-3
    sous_rubrique: seuils
    question: "How do you set the abstention threshold?"
    choix:
      - "At 0.5, by default"
      - "On a test set with unanswerable questions, measuring errors and abstentions"
      - "At random"
      - "As high as possible"
    reponse: 1
    pourquoi: >-
      Threshold too low: the system answers everything, and gets it wrong.
      Threshold too high: it abstains all the time, and is useless.
      The test set's unanswerable questions show where to draw the line.
      The right threshold depends on what an error costs in your field.

  - id: seuils-4
    sous_rubrique: seuils
    question: "A medical assistant and a cooking-recipe assistant. Which one should abstain more readily?"
    choix:
      - "The cooking one"
      - "Both the same"
      - "The medical one: an error there costs far more"
      - "Neither"
    reponse: 2
    pourquoi: >-
      The threshold reflects a cost.
      A bad recipe spoils a dinner.
      Bad medical information can harm someone.
      The more serious the error, the more abstention you accept.

  - id: je-ne-sais-pas-1
    sous_rubrique: je-ne-sais-pas
    question: "According to OpenAI (2025), why do models guess instead of saying “I don't know”?"
    choix:
      - "They don't know that phrase"
      - "Users forbid it"
      - "Evaluations score the right answer and give zero for abstaining: guessing pays more"
      - "It takes too long to write"
    reponse: 2
    pourquoi: >-
      It is like an exam with no negative marking.
      Guessing a birthday: a one-in-365 chance to score.
      Saying “I don't know”: zero points, always.
      A model optimized for those scores learns to guess.

  - id: je-ne-sais-pas-2
    sous_rubrique: je-ne-sais-pas
    question: "In the same article, one model abstains on 52% of questions, the other on 1%. What do you see?"
    choix:
      - "The one that abstains 1% of the time has slightly better accuracy, but far more errors"
      - "The one that abstains is useless"
      - "Both make as many errors"
      - "Abstention has no effect"
    reponse: 0
    pourquoi: >-
      The model that answers everything gains a few right answers.
      But it piles up wrong ones.
      A ranking by accuracy alone puts it first.
      A ranking that counts errors says the opposite.

  - id: je-ne-sais-pas-3
    sous_rubrique: je-ne-sais-pas
    question: "The simplest technique, according to Anthropic, to reduce made-up answers?"
    choix:
      - "Raise the temperature"
      - "Add more documents"
      - "Forbid short answers"
      - "Explicitly allow the model to say it doesn't know"
    reponse: 3
    pourquoi: >-
      Without permission, the model tends to answer no matter what.
      A clear instruction gives it an honest way out.
      For example: “If the documents don't contain the answer, say so.”
      It costs nothing and clearly reduces false information.

  - id: je-ne-sais-pas-4
    sous_rubrique: je-ne-sais-pas
    question: "A good abstention is…"
    choix:
      - "“I don't know.” and nothing else"
      - "Saying what is missing, what is certain, and offering a next step"
      - "Making up a cautious answer"
      - "Returning a technical error"
    reponse: 1
    pourquoi: >-
      “I don't know” alone leaves the user with no way forward.
      Better: “The documents cover domestic flights, not international ones.”
      Then a next step: a link, a human, a clarifying question.
      The abstention becomes a useful answer.
---

## Confidence thresholds {#seuils}

Sometimes, the right answer is not to answer.

But that must be decided on **signals**, not on a hunch.

| Signal | What it indicates |
|---|---|
| Best document's score | Does the corpus hold the answer? |
| Reranker's score | Do the passages really answer? |
| Checker's verdict | Is the answer supported? |
| Consistency across several answers | Is the model stable? (next topic) |

:::pourquoi
No signal is perfect on its own.
Together, they say whether the answer rests on solid ground.
:::

Confidence expressed in words (“95% sure”) is often poorly calibrated.

:::pourquoi
It is generated text, not a measurement.
Prefer computed scores, and check them on your test set.
:::

The threshold is tuned on **unanswerable** questions, and by the cost of an error.

:::pourquoi
Too low: you answer everything and get it wrong.
Too high: you no longer answer anything.
In medicine, you accept more abstentions than in cooking.
:::

```mermaid
flowchart LR
  Q["Question"] --> R["Search + reranking"]
  R -->|"low scores"| S["Useful abstention"]
  R -->|"high enough scores"| L["LLM: answer"]
  L --> V["Checker"]
  V -->|"unsupported claims"| S
  V -->|"supported"| F["Answer with citations"]
```

## Saying “I don't know” {#je-ne-sais-pas}

September 2025. OpenAI publishes *Why language models hallucinate*.

Its thesis: models are trained and scored like students taking an exam with no negative marking.

:::pourquoi
Guessing a birthday: a one-in-365 chance to score.
“I don't know”: zero, always.
A model optimized for those scores learns to guess.
:::

The article compares two models. One abstains on 52% of questions. The other on 1%. The second has slightly better accuracy. But far more errors.

:::pourquoi
Answering everything gains a few right answers.
And piles up wrong ones.
Never measure accuracy without counting errors.
:::

Explicitly allow abstention in the prompt.

:::pourquoi
Anthropic presents it as the basic technique against made-up answers.
Without permission, the model tries to answer at all costs.
:::

A good abstention still helps:

- what is certain, with its source;
- what is missing;
- a next step: a link, a human, a clarifying question.

:::pourquoi
Air Canada's chatbot had the official page at hand.
It could have said: “I'm not sure. Here is the official page on bereavement fares.”
A less appealing answer. But not a made-up rule.
:::

## Key points

- Abstain on measured signals, not on a hunch.
- The threshold is tuned on unanswerable questions, by the cost of an error.
- Evaluations that ignore errors push models to guess.
- Allow “I don't know”, and make it useful.
