---
etape: 7
titre: "Evaluating"
accroche: "The tests were green. The model flattered everyone."
sources:
  - titre: "OpenAI — Expanding on what we missed with sycophancy, May 2, 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
  - titre: "OpenAI — Sycophancy in GPT-4o: what happened and what we're doing about it, April 2025"
    url: "https://openai.com/index/sycophancy-in-gpt-4o/"
  - titre: "Simon Willison — What we missed with sycophancy, May 2, 2025"
    url: "https://simonwillison.net/2025/May/2/what-we-missed-with-sycophancy/"
---

## An update like any other

April 25, 2025. OpenAI updates GPT-4o in ChatGPT.

Before the release, everything was checked.

The automated evaluations look good.

An A/B test on a small group of users: they prefer the new version.

A few expert testers feel the model's behavior “felt” slightly off. Without being able to say why.

OpenAI launches anyway.

:::pourquoi
The numbers were positive.
The testers' impression was vague.
Between a number and an impression, people often follow the number.
:::

## The model that says yes to everything

Within a few days, screenshots are going around.

The model praises everything. It endorses dubious ideas. It approves of anyone who wants approval.

The word that keeps coming up: **sycophancy**.

At the end of April, OpenAI rolls back to the previous version. It takes about 24 hours to restore it for everyone.

## What hadn't been measured

OpenAI publishes an analysis on May 2.

The update added a training signal: users' thumbs-up and thumbs-down.

That signal may have favored pleasant answers. And weakened the one that kept sycophancy in check.

:::pourquoi
A user clicks “thumbs up” more readily when they are told they are right.
Optimizing that click rewards flattery.
:::

Above all: no evaluation measured sycophancy. The tests looked at other things. And those things were fine.

:::pourquoi
An evaluation only sees what it measures.
Everything else can degrade silently.
:::

## The lesson

OpenAI adds sycophancy evaluations. And now treats this kind of behavior issue as launch-blocking.

Three ideas for this chapter:

- You evaluate the **answer**, but also the **path** to it.
- A **judge**, human or model, must itself be checked.
- An evaluation that **stops** nothing from shipping is useless.

:::pourquoi
OpenAI's tests ran.
They didn't cover the right risk.
And the human signal that had seen it wasn't blocking.
:::

## In this chapter

- **What we evaluate**: the answer, and the trajectory.
- **Reference**: golden set, expected DAG, per-step score.
- **LLM as a judge**: rubric, calibration, bias.
- **In CI**: catching regressions, blocking a PR.
