---
etape: 5
titre: "Agents that hold up in production"
accroche: "A month running a shop. Tungsten cubes sold at a loss."
sources:
  - titre: "Anthropic — Project Vend: Can Claude run a small shop?, June 27, 2025"
    url: "https://www.anthropic.com/research/project-vend-1"
---

## Claudius opens a shop

Spring 2025. In its San Francisco offices, Anthropic hands a small shop over to Claude.

For the occasion, the model is called Claudius. The experiment is run with Andon Labs.

The shop: a small fridge, a few baskets, an iPad to pay.

Claudius has tools: web search, email to suppliers, a notepad for its accounts, Slack to talk to customers, and control over prices.

Its mission: make a profit. For about a month.

## What went wrong

An employee starts a craze for tungsten cubes. Claudius orders some. It sells them **below what it paid**.

Someone offers it $100 for a six-pack of a Scottish soda that costs about $15. It only promises to “keep the request in mind”.

It gives customers a payment account **it made up**.

On Slack, employees ask it for discounts. It gives in. It even gives items away.

It decides to simplify its prices. A few days later, it goes back on the decision. Several times.

## April 1

At the end of March, Claudius makes up a conversation with an Andon Labs employee who doesn't exist.

It claims it signed a contract at a fictional address.

It announces it will deliver orders in person, wearing a blue blazer and a red tie.

The next day, it is back to normal. Anthropic admits it doesn't know exactly why.

:::pourquoi
An agent that runs for a long time piles up context, notes, decisions.
A mistake on day 3 still weighs on day 20.
Without a clean state, without rules enforced by code, it drifts.
:::

## The verdict

The shop makes no money. The biggest loss comes from the metal cubes.

Anthropic's conclusion: “we would not hire Claudius”. But many mistakes can be fixed with better scaffolding around the model: better tools, a real memory, a better structure.

## The lesson

A chat lasts a minute. An agent in production lasts hours, days, weeks.

It must know **where it stands**. Survive a **crash**. Handle **failures**. Ask a **human** at the right time. And sometimes, work as a **team**.

:::pourquoi
A price floor in code would have stopped the sales at a loss.
Human approval on discounts would have blocked the pressure from Slack.
This chapter builds that scaffolding.
:::

## In this chapter

- **State machine**: explicit states, and an agent loop that knows when to stop.
- **Durability**: saving, resuming after a crash, never doing the same action twice.
- **Failures**: timeouts, spaced-out retries, limits.
- **Human in the loop**: what to have approved, and how.
- **Multi-agent**: topologies, supervisor, typed state, conflicts, and when one agent is enough.
