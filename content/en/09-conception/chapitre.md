---
etape: 9
titre: "Designing and documenting"
accroche: "Two teams. Two posts a day apart. Two opposite conclusions."
sources:
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, June 12, 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "Anthropic — How we built our multi-agent research system, June 13, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
---

## June 12, 2025: “Don't build multi-agents”

Cognition makes Devin, an agent that writes code.

Walden Yan publishes a post with a blunt title: *Don't Build Multi-Agents*.

His example: cloning the game Flappy Bird.

You split the task in two. One subagent makes the background. Another makes the bird.

The first delivers a Super Mario-style background. The second, a bird that doesn't look like the one in the game.

Each did good work. Together, nothing fits.

:::pourquoi
The subagents didn't see each other's work.
Each made implicit choices: style, size, movement.
Those choices contradicted each other.
:::

His conclusion: a single agent, a single thread, a shared context.

## June 13, 2025: “Here is our multi-agent system”

The next day, Anthropic publishes the opposite.

A lead agent launches several subagents in parallel. Each explores one research lead.

On an internal evaluation, this system beats a single agent by **90.2%**.

But it uses about **15 times** more tokens than a chat.

:::pourquoi
Research splits well: each lead is independent.
More subagents means more reading in parallel.
The price is the volume of tokens.
:::

## Who is right?

Both.

Anthropic writes it itself: multi-agent is a poor fit for tasks where everyone must share the same context. Or that have many dependencies. Most coding tasks, for example.

Cognition does code. Anthropic does research.

:::pourquoi
An architecture decision isn't true or false in general.
It is right for a context, with an accepted cost.
Changing the context can reverse the answer.
:::

## The lesson

These two posts don't only say *what*. They say **why**, **at what price**, and **where it breaks**.

That is what makes them useful.

Three ideas for this chapter:

- A decision is written with its **context** and its **trade-off**.
- A diagram shows who talks to whom, and which **data** goes through.
- A number is only worth something if it is **reproducible**.

:::pourquoi
Without the context, you copy the conclusion into the wrong place.
Without the cost, you discover the bill in production.
Without the method, a number is just a claim.
:::

## In this chapter

- **Architecture decisions**: writing an ADR, justifying a trade-off.
- **Diagrams**: data flow, agent topology.
- **Benchmarks**: making a result reproducible.
- **Publishing**: analyzing your architecture in public.
