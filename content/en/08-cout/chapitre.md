---
etape: 8
titre: "Cost and performance"
accroche: "A 200-dollar subscription. Tens of thousands of dollars of compute."
sources:
  - titre: "TechCrunch — Anthropic unveils new rate limits to curb Claude Code power users, July 28, 2025"
    url: "https://techcrunch.com/2025/07/28/anthropic-unveils-new-rate-limits-to-curb-claude-code-power-users/"
  - titre: "Simon Willison — notes of July 28, 2025 on Anthropic's announcement"
    url: "https://simonwillison.net/2025/Jul/28/anthropic/"
---

## An unlimited plan, or almost

Claude Code is Anthropic's coding agent.

It comes with a monthly subscription. The most expensive one costs 200 dollars.

A flat fee. Free consumption.

:::pourquoi
A flat plan is easy to sell and easy to pay.
But the real cost depends on the tokens used.
The provider bets that the average will stay reasonable.
:::

## The agent that never sleeps

Some users run Claude Code in the background. And leave it running. Day and night.

An agent doesn't need a break. It keeps chaining tasks as long as it is given some.

Others share or resell their access.

At the end of July 2025, Anthropic gives a figure: a single user consumed **tens of thousands of dollars** worth of compute. On a 200-dollar subscription.

:::pourquoi
A human chatting is limited by their reading speed.
An agent in a loop isn't.
An agent's cost has no natural ceiling.
:::

## The answer: limits

On July 28, 2025, Anthropic announces **weekly limits**. They come on top of the limits that reset every five hours.

They take effect on August 28. According to Anthropic, they affect fewer than 5% of subscribers.

Beyond them, Max subscribers can buy extra usage at API rates.

:::pourquoi
The five-hour limit stops a session that runs too long.
The weekly limit stops continuous use.
Two windows, two behaviors targeted.
:::

## The lesson

An agent costs what it consumes. And it can consume without end.

Three ideas for this chapter:

- You **cut** the cost of each call: caching, the right model at the right time.
- You **cap** what a user, a task or a loop can spend.
- You **measure** the cost per task and the latency, not only the overall bill.

:::pourquoi
Anthropic saw the problem in its usage numbers.
Without per-user measurement, abuse stays invisible in the average.
Without a cap, it has no end.
:::

## In this chapter

- **Caching**: prompt caching, semantic caching and its risks.
- **Routing**: choosing the model by complexity, planning fallbacks.
- **Guardrails**: budgets, automatic cutoff, quotas.
- **Measurement**: cost per task, p50 and p95 latency.
