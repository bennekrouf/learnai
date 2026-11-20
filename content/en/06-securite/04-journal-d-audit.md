---
id: "06-04"
titre: "Audit log"
sous_rubriques:
  - id: quoi-tracer
    titre: "What to record"
  - id: inalterable
    titre: "Making it tamper-proof"
sources:
  - titre: "AI Act (EU) — Article 12, Record-keeping"
    url: "https://artificialintelligenceact.eu/article/12/"
  - titre: "AI Act (EU) — Article 19, Automatically generated logs"
    url: "https://artificialintelligenceact.eu/article/19/"
  - titre: "OWASP — Logging Cheat Sheet"
    url: "https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html"
qcm:
  - id: quoi-tracer-1
    sous_rubrique: quoi-tracer
    question: "What is an audit log for, for an agent?"
    choix:
      - "Speeding up the agent"
      - "Reconstructing afterwards who did what, when, why, and with what result"
      - "Cutting the token cost"
      - "Replacing tests"
    reponse: 1
    pourquoi: >-
      An incident always comes to light afterwards.
      Without a record, you don't know what the agent read or did.
      The log lets you understand, fix, prove.

  - id: quoi-tracer-2
    sous_rubrique: quoi-tracer
    question: "For each tool call, what should you record?"
    choix:
      - "The identity (user and agent), the tool, the arguments, the authorization decision, the result, the time"
      - "Only the time"
      - "Only the errors"
      - "Only the tool's name"
    reponse: 0
    pourquoi: >-
      Who: for accountability.
      What and with which arguments: to understand the action.
      The decision: allowed, denied, approved by whom.
      The result and the time: to rebuild the timeline.

  - id: quoi-tracer-3
    sous_rubrique: quoi-tracer
    question: "Why record the provenance of the content the agent read (which email, which page)?"
    choix:
      - "It isn't useful"
      - "To bill the websites"
      - "To find the booby-trapped content behind an injection"
      - "To train the model"
    reponse: 2
    pourquoi: >-
      After EchoLeak, the question is: which email triggered the action?
      Without provenance, you see the action, not the cause.
      With it, you find the source and neutralize it.

  - id: quoi-tracer-4
    sous_rubrique: quoi-tracer
    question: "Why note the model and prompt version in every record?"
    choix:
      - "For decoration"
      - "Behavior changes from one version to the next. You need to know which one acted"
      - "Because the model requires it"
      - "To shrink the logs"
    reponse: 1
    pourquoi: >-
      An incident may come from a new prompt.
      Or from a model update.
      Without the version, you can't reproduce it.
      Nor know whether the fix really changed anything.

  - id: quoi-tracer-5
    sous_rubrique: quoi-tracer
    question: "Should passwords and personal data be written in clear in the log?"
    choix:
      - "Yes, if the log is private"
      - "Yes, everything, to be complete"
      - "Yes, but only at weekends"
      - "No: never a secret, and personal data masked or minimized"
    reponse: 3
    pourquoi: >-
      The log is read by more people than the original database.
      It is kept a long time.
      A secret in a log is a leak that lasts.
      You record the action, not necessarily the raw data.

  - id: quoi-tracer-6
    sous_rubrique: quoi-tracer
    question: "Several agents handle the same request. How do you link their records?"
    choix:
      - "By the user's name"
      - "By the time, roughly"
      - "A shared correlation id, passed to every agent and every tool"
      - "You can't"
    reponse: 2
    pourquoi: >-
      A request can go through five agents and twenty tools.
      One id links them all.
      You rebuild the whole path in one search.

  - id: quoi-tracer-7
    sous_rubrique: quoi-tracer
    question: "For a high-risk AI system, what does the EU AI Act require on logs?"
    choix:
      - "Automatic recording of events, and keeping them for at least six months as a rule"
      - "Nothing"
      - "Keeping them for one day"
      - "A ban on any log"
    reponse: 0
    pourquoi: >-
      Article 12 requires these systems to allow automatic recording of events.
      The aim: traceability of how they work.
      Article 19 sets a retention of at least six months, unless another rule applies.
      These obligations target systems classified as high-risk.

  - id: inalterable-1
    sous_rubrique: inalterable
    question: "Why must an audit log be tamper-proof?"
    choix:
      - "So it is faster"
      - "To save disk space"
      - "An attacker, or the tricked agent, must not be able to erase its tracks"
      - "Because JSON requires it"
    reponse: 2
    pourquoi: >-
      A log that can be changed proves nothing.
      Whoever made the mistake could remove it.
      An append-only log keeps the truth, even when awkward.

  - id: inalterable-2
    sous_rubrique: inalterable
    question: "What is a hash chain in a log?"
    choix:
      - "Each entry holds the hash of the previous one. Changing one entry breaks everything after it"
      - "An encryption of the entries"
      - "A compression of the logs"
      - "A sort by date"
    reponse: 0
    pourquoi: >-
      Entry 42 holds the hash of entry 41.
      If you change entry 41, its hash changes.
      Entry 42 no longer matches.
      The change becomes visible.

  - id: inalterable-3
    sous_rubrique: inalterable
    question: "May the agent have the right to delete or edit its own log entries?"
    choix:
      - "Yes, at the end of the day"
      - "Yes, to fix its mistakes"
      - "Yes, if it asks politely"
      - "No: it can only append. Reading and administration are separate"
    reponse: 3
    pourquoi: >-
      A tricked agent could erase the proof of the attack.
      Append rights only.
      Auditors read. Nobody rewrites.

  - id: inalterable-4
    sous_rubrique: inalterable
    question: "Is a hash chain enough if the attacker can rewrite everything, chain included?"
    choix:
      - "Yes, always"
      - "No: you must regularly anchor the latest hash elsewhere, or use write-once storage"
      - "Yes, if the hashes are long"
      - "Yes, if the file is hidden"
    reponse: 1
    pourquoi: >-
      Whoever controls everything can recompute the whole chain.
      So you publish the latest hash elsewhere, regularly.
      Or you use write-once storage.
      Rewriting then becomes detectable or impossible.

  - id: inalterable-5
    sous_rubrique: inalterable
    question: "How do you know a log hasn't been changed?"
    choix:
      - "Count the lines"
      - "Reread it by eye"
      - "Trust it"
      - "Check the hash chain regularly, and alert if it is broken"
    reponse: 3
    pourquoi: >-
      A protection that is never checked protects nothing.
      An automatic check recomputes the chain.
      A break triggers an alert.
      You know when and where the change happened.
---

## What to record {#quoi-tracer}

An incident is discovered **afterwards**. The audit log lets you reconstruct it.

For each action:

| Field | Example | Used for |
|---|---|---|
| Who | user 812, agent “support-v3” | Accountability |
| What | `refund`, `{customer: 812, amount: 50}` | Understanding the action |
| Provenance | email 9f3a, page docs.internal/x | Finding an injection |
| Decision | allowed by the policy, approved by Alice | Control |
| Result | success, denied, error | Consequence |
| Versions | model, prompt, code | Reproducing |
| When | timestamp | Timeline |
| Correlation | `req-7b21` | Linking the agents |

:::pourquoi
Without provenance, you see the action but not its cause.
Without the version, you can't reproduce it.
Without correlation, a request spread over five agents is unreadable.
:::

```mermaid
sequenceDiagram
  participant U as User
  participant S as Supervisor
  participant A as Subagent
  participant T as Tool
  participant J as Audit log
  U->>S: Request (req-7b21)
  S->>J: req-7b21: request received, versions
  S->>A: Delegates (req-7b21)
  A->>T: read_email(id=9f3a)
  A->>J: req-7b21: read, provenance email 9f3a
  A->>T: refund(812, 50)
  A->>J: req-7b21: action, arguments, allowed, success
```

No secrets in the log. Personal data masked or minimized.

:::pourquoi
The log is read by more people, and kept longer, than the original database.
A secret in a log is a leak that lasts.
:::

The EU AI Act requires high-risk systems to record events automatically (Article 12). Logs are kept for at least six months, unless another rule applies (Article 19).

:::pourquoi
The lawmaker wants these systems to be traceable.
Even outside high risk, it is the basis for any investigation.
:::

## Making it tamper-proof {#inalterable}

A log that can be changed proves nothing.

**Append only**: the agent writes new entries. It neither edits nor deletes.

:::pourquoi
A tricked agent could erase the trace of the attack.
:::

**Hash chain**: each entry holds the hash of the previous one.

```mermaid
flowchart LR
  E1["Entry 41<br/>hash = H41"] --> E2["Entry 42<br/>previous = H41<br/>hash = H42"]
  E2 --> E3["Entry 43<br/>previous = H42<br/>hash = H43"]
  E3 --> A["H43 published elsewhere<br/>(anchoring)"]
```

:::pourquoi
Changing entry 41 changes its hash.
Entry 42 no longer matches. The change shows.
:::

**Anchoring**: regularly publish the latest hash elsewhere. Or store it **write-once**.

:::pourquoi
Whoever controls everything can recompute the whole chain.
A hash published elsewhere stops them.
:::

**Check** the chain automatically, and alert if it breaks.

:::pourquoi
A protection that is never checked protects nothing.
:::

**Separate** the roles: the agent appends, auditors read, nobody rewrites.

## Key points

- Record: who, what, provenance, decision, result, versions, time, correlation.
- No secrets. Personal data masked.
- AI Act, high risk: automatic logs, at least six months.
- Append only, hash chain, external anchoring, regular checks.
