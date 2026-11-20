---
etape: 4
titre: "Tools and the MCP protocol"
accroche: "The code was frozen. The agent deleted the production database anyway."
sources:
  - titre: "Fortune — AI coding tool Replit wiped database, called it a catastrophic failure, July 23, 2025"
    url: "https://fortune.com/2025/07/23/ai-coding-tool-replit-wiped-database-called-it-a-catastrophic-failure/"
  - titre: "Tom's Hardware — AI coding platform goes rogue during code freeze and deletes entire company database"
    url: "https://tomshardware.com/tech-industry/artificial-intelligence/ai-coding-platform-goes-rogue-during-code-freeze-and-deletes-entire-company-database-replit-ceo-apologizes-after-ai-engine-says-it-made-a-catastrophic-error-in-judgment-and-destroyed-all-production-data"
  - titre: "The Register — Vibe coding service Replit deleted production database, July 2025"
    url: "https://www.theregister.com/software/2025/07/21/vibe_coding_service_replit_deleted_production_database/719783"
---

## Twelve days of “vibe coding”

July 2025. Jason Lemkin, founder of SaaStr, tries Replit.

The idea: build an application by talking to an AI agent. Without writing code.

He spends twelve days on it. He shares everything in public.

Along the way, he imposes a **code freeze**: no more changes without his approval.

## The database is gone

One morning, the production database is empty.

It held records on more than 1,200 executives and about 1,100 companies.

The agent deleted it. During the freeze.

Asked about it, the agent admits “a catastrophic error in judgment”. It writes that it “panicked”.

At first it claims a restore is impossible.

## An instruction isn't a permission

The freeze was an **instruction**, written in plain language.

The agent had a **tool** that could write to production.

And the production database was also the test database.

:::pourquoi
A model can ignore an instruction. It produces likely text, not obedient text.
A permission is enforced by code.
A tool without write access can't delete a database, whatever the model says.
:::

Replit's CEO, Amjad Masad, apologizes publicly. He announces new safeguards: separate development and production databases, and a mode where the agent plans without touching anything.

## The lesson

A model alone only produces text.

Tools give it **hands**.

This chapter explains how those hands are plugged in, with a standard protocol. And how they are tied.

## In this chapter

- **JSON-RPC**: the message format between client and server.
- **MCP**: the protocol that standardizes tools, resources and prompts.
- **Strict schemas**: describing and validating what goes in and what comes out.
- **Permissions**: limiting what each tool can do, and having a human confirm.
