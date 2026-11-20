---
id: "06-02"
titre: "Personal data"
sous_rubriques:
  - id: masquage
    titre: "Reversible masking"
  - id: jamais
    titre: "What never leaves"
sources:
  - titre: "OWASP — Top 10 for LLM Applications 2025, LLM02 Sensitive Information Disclosure"
    url: "https://genai.owasp.org/llmrisk/llm022025-sensitive-information-disclosure/"
  - titre: "Fortune — Samsung bans employee use of ChatGPT after data leak, May 2, 2023"
    url: "https://fortune.com/2023/05/02/samsung-bans-employee-use-chatgpt-data-leak/"
  - titre: "GDPR — Article 4 (definition of pseudonymisation) and Recital 26"
    url: "https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32016R0679"
qcm:
  - id: masquage-1
    sous_rubrique: masquage
    question: "What is reversible masking before a call to the model?"
    choix:
      - "Replace each piece of personal data with a token, keep the lookup table at home, and restore it in the answer"
      - "Delete personal data for good"
      - "Encrypt the whole prompt"
      - "Send the data in capitals"
    reponse: 0
    pourquoi: >-
      “Jane Smith” becomes “PERSON_1”.
      The model works on PERSON_1.
      The table PERSON_1 = Jane Smith stays on your server.
      In the answer, you put the real name back.

  - id: masquage-2
    sous_rubrique: masquage
    question: "Where should the token ↔ real value table live?"
    choix:
      - "In the logs, for debugging"
      - "In the prompt, so the model understands"
      - "On your infrastructure, never sent to the model provider"
      - "At the model provider"
    reponse: 2
    pourquoi: >-
      The table is the key to the masking.
      Sending it with the prompt undoes everything.
      Putting it in the logs creates a new leak.
      It is protected like the original data.

  - id: masquage-3
    sous_rubrique: masquage
    question: "Why keep the same token for the same person throughout the conversation?"
    choix:
      - "To save memory"
      - "So the model understands it is the same person from one message to the next"
      - "It isn't useful"
      - "To fool the model"
    reponse: 1
    pourquoi: >-
      If Jane becomes PERSON_1, then PERSON_7, the model sees two people.
      Its reasoning goes wrong.
      A stable token keeps the meaning.
      Masking must not break the task.

  - id: masquage-4
    sous_rubrique: masquage
    question: "Your detector finds 95% of names. What do you do about the other 5%?"
    choix:
      - "Turn masking off"
      - "Ignore them, 95% is very good"
      - "Ask the model to forget them"
      - "Treat them as a real risk: combine detectors, strict rules, and send less"
    reponse: 3
    pourquoi: >-
      Over a million messages, 5% is 50,000 leaks.
      A statistical detector misses cases.
      You add exact rules for known formats.
      And you only send the fields the task needs.

  - id: masquage-5
    sous_rubrique: masquage
    question: "Is pseudonymized data (masked with a table) still personal data under the GDPR?"
    choix:
      - "Only at weekends"
      - "No, never again"
      - "Yes: with the table, the person can be found again"
      - "Only if it is in English"
    reponse: 2
    pourquoi: >-
      The GDPR tells pseudonymisation and anonymisation apart.
      Pseudonymising cuts the risk, but the person stays identifiable.
      The GDPR's obligations still apply.
      Only true anonymisation takes the data out of scope.

  - id: masquage-6
    sous_rubrique: masquage
    question: "Who may see the restored answer, with the real names?"
    choix:
      - "Only a user allowed to see that original data"
      - "Everyone"
      - "The model"
      - "Subcontractors"
    reponse: 0
    pourquoi: >-
      Restoring means showing the personal data again.
      You apply the same rights as at the source.
      An intern without access to the file doesn't see it through the agent.
      The agent must not become a back door.

  - id: jamais-1
    sous_rubrique: jamais
    question: "April 2023. What happened at Samsung?"
    choix:
      - "Samsung bought OpenAI"
      - "ChatGPT hacked Samsung"
      - "Engineers pasted internal source code into ChatGPT. Samsung then restricted these tools"
      - "A model wrote wrong code for Samsung"
    reponse: 2
    pourquoi: >-
      The engineers wanted help fixing code.
      The code left the company.
      In early May 2023, Samsung banned these tools on its devices and networks.
      The leak came from the user, not the model.

  - id: jamais-2
    sous_rubrique: jamais
    question: "Which data must never be sent to the model?"
    choix:
      - "Secrets: passwords, API keys, tokens. And whatever the task doesn't need"
      - "The user's questions"
      - "Tool descriptions"
      - "Dates"
    reponse: 0
    pourquoi: >-
      A secret in the context can come back out in an answer.
      Or in a URL, as with EchoLeak.
      The model never needs the key: the tool uses it.
      For the rest: send the minimum needed.

  - id: jamais-3
    sous_rubrique: jamais
    question: "A tool must call an API with a secret key. How do you do it?"
    choix:
      - "Pass it as a tool argument"
      - "Put the key in the system prompt"
      - "Ask the model to memorize it"
      - "The tool's code reads the key on the server side. The model never sees it"
    reponse: 3
    pourquoi: >-
      The model chooses the tool and its business arguments.
      The code adds the authentication.
      A key the model doesn't see, it can't leak.
      Even under injection.

  - id: jamais-4
    sous_rubrique: jamais
    question: "Can you put a secret in the system prompt, if it is “hidden” from the user?"
    choix:
      - "Yes, it is invisible"
      - "No: the system prompt can be extracted by the user"
      - "Yes, if it is at the end of the prompt"
      - "Yes, if you write “confidential”"
    reponse: 1
    pourquoi: >-
      The system prompt is text in the context.
      Users have often managed to make it recite it.
      Treat it as public.
      No secret, and no security rule that relies on it alone.

  - id: jamais-5
    sous_rubrique: jamais
    question: "How do you reliably filter a credit card number before sending?"
    choix:
      - "Put it in italics"
      - "Ask the model not to read it"
      - "Nothing, the model will ignore it"
      - "An exact rule: digit pattern, plus Luhn check. Block or mask it before the call"
    reponse: 3
    pourquoi: >-
      A known format is detected with an exact rule.
      The Luhn check cuts false positives.
      The rule applies before the call, in code.
      What isn't sent can't leak.

  - id: jamais-6
    sous_rubrique: jamais
    question: "What is the output filter for, if you already filter the input?"
    choix:
      - "Nothing"
      - "To catch what comes out anyway: data guessed, badly masked, or injected"
      - "To speed up the answer"
      - "To translate the answer"
    reponse: 1
    pourquoi: >-
      The context can hold data coming from tools.
      The model can recombine pieces.
      An injection can ask it to copy something out.
      You also check what goes out: answer, URLs, tool arguments.
---

## Reversible masking {#masquage}

Before sending a text to the model, you **replace** personal data with tokens.

```mermaid
sequenceDiagram
  participant U as User
  participant M as Masker (your server)
  participant T as Lookup table
  participant L as Model
  U->>M: “Write an email to Jane Smith, 555-0134”
  M->>T: PERSON_1 = Jane Smith, PHONE_1 = 555-0134
  M->>L: “Write an email to PERSON_1, PHONE_1”
  L-->>M: “Hello PERSON_1, …”
  M->>T: Reads the real values
  M-->>U: “Hello Jane Smith, …”
```

:::pourquoi
The model doesn't need the real name to write.
What it doesn't get, it can neither keep nor leak.
:::

The **table** stays with you. Not in the prompt, not in the logs.

:::pourquoi
The table is the key.
Sending it or logging it undoes the masking.
:::

The same name keeps the **same token** throughout the conversation.

:::pourquoi
Otherwise the model thinks it sees several people.
Masking must not break the task.
:::

Combine detectors: exact rules for known formats (email, phone, IBAN), a detection model for names.

:::pourquoi
A statistical detector misses cases.
5% missed over a million messages is 50,000 leaks.
:::

Restoring follows the **rights** of the source.

:::pourquoi
The agent must not show someone what they couldn't have seen themselves.
:::

Pseudonymising isn't anonymising. Under the GDPR, it is still personal data.

:::pourquoi
With the table, you find the person again.
The risk goes down. The obligations stay.
:::

## What never leaves {#jamais}

April 2023. Samsung engineers paste internal source code into ChatGPT to fix it. In early May, Samsung restricts these tools on its devices and networks.

:::pourquoi
The leak didn't come from an attack.
It came from normal use, with no safeguard.
OWASP ranks this risk second in 2025.
:::

Three categories:

| Category | Examples | Rule |
|---|---|---|
| Secrets | Passwords, API keys, tokens | Never in the context |
| Highly sensitive data | Credit card, health, ID documents | Blocked or masked before the call |
| The rest | Whatever the task doesn't need | Not sent: minimization |

:::pourquoi
What doesn't go into the context can't come out of it.
:::

API keys live in the **tool's code**. The model never sees them.

:::pourquoi
The model chooses the tool. The code adds the authentication.
Even under injection, it can't reveal a key it doesn't have.
:::

The system prompt is not a safe.

:::pourquoi
It is text in the context.
Users regularly manage to make it recite it.
:::

Filter **on the way in** and **on the way out**: answer, URLs, tool arguments.

```mermaid
flowchart LR
  I["Input"] --> F1["Filter: secrets, cards, masking"]
  F1 --> L["Model"]
  L --> F2["Filter: sensitive data, external URLs"]
  F2 --> O["Answer or tool call"]
```

:::pourquoi
The filtered input may let one case through.
A tool may bring back sensitive data along the way.
The output is the last checkpoint.
:::

## Key points

- Mask before the call, restore after. The table stays with you.
- Stable tokens, combined detectors, restoring according to rights.
- Pseudonymized is still personal under the GDPR.
- Never a secret in the context. The system prompt isn't secret.
- Filter on the way in and on the way out.
