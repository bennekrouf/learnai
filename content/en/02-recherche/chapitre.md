---
etape: 2
titre: "Finding the information"
accroche: "Google advises putting glue on pizza. The model made nothing up."
sources:
  - titre: "Gizmodo — Google explains why it suggested adding glue to pizza, May 2024"
    url: "https://gizmodo.com/google-explains-why-it-suggested-adding-glue-to-pizza-1851511786"
  - titre: "MIT Technology Review — Why are Google's AI Overviews results so bad?, May 31, 2024"
    url: "https://www.technologyreview.com/2024/05/31/1093019/why-are-googles-ai-overviews-results-so-bad/"
  - titre: "Engadget — Google is putting more restrictions on AI Overviews after it told people to put glue on pizza"
    url: "https://www.engadget.com/google-is-putting-more-restrictions-on-ai-overviews-after-it-told-people-to-put-glue-on-pizza-011316780.html"
---

## Glue on pizza

May 2024. Google launches AI Overviews in the United States.

At the top of the results, an AI sums up the web for you.

A few days later, a user asks: how do you keep cheese from sliding off pizza?

The answer: add non-toxic glue to the sauce.

Another asks: how many rocks should I eat?

The answer: at least one small rock per day.

The screenshots go around the world.

## The model made nothing up

The glue comes from an old Reddit comment. A joke, posted under a crude username.

The rocks come from *The Onion*, a satire site. Headline: “Geologists Recommend Eating At Least One Small Rock Per Day”.

The model faithfully summarized what it was given.

The problem was **upstream**: in what it was given.

:::pourquoi
A RAG system has two stages: retrieve, then write.
The second can't be better than the first.
A joke retrieved well becomes an answer written well.
:::

## Google's explanation

Liz Reid, head of Google Search, publishes an explanation.

For rare questions, few good sources exist. She calls it a “data void”.

In that void, satire and forum jokes rise to the top.

Google adds filters: less satire, less forum content for those questions.

:::pourquoi
Filtering on the kind of source is filtering on metadata.
The text of a joke looks like the text of advice.
Only the fact “comes from a satire site” tells them apart.
:::

## The lesson

The “R” in RAG does half the work. Often more.

Retrieving isn't finding a similar text. It is finding the **right** text.

## In this chapter

- **Preparing the corpus**: chunking, describing, indexing.
- **Search**: by keyword, by meaning, and both together.
- **Reranking**: a second sort, slower and finer.
- **Measuring search**: knowing whether it really finds.
- **GraphRAG**: linking information, and knowing when it isn't worth the cost.
