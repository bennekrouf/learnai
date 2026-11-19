# LearnAI

Learn AI systems engineering, the quiz first. The course goes from how an LLM
works to designing and documenting agents in production, in ten étapes:
context, retrieval, reliability, tools and MCP, agents, security, evaluation,
cost, and design.

Each étape opens on a true story from AI news or literature. Each rubrique is
taken in two tabs:

| Tab | What it does |
| --- | --- |
| QCM | One question at a time. Once answered, the right choice shows, and **Pourquoi ?** unfolds the reason. A link leads to the part of the lesson it comes from. |
| Cours | The lesson, in short sentences, with a **Pourquoi ?** to unfold after each claim and a diagram wherever more than two actors talk to each other. |

The course is in French for now: 10 étapes, 467 questions.

## Run it

`desktop-kit` must sit beside this folder (`../desktop-kit`).

```bash
cargo run
```

Answers are kept in `progress.json`, in the app's data folder
(`~/Library/Application Support/learnai` on macOS, `%LOCALAPPDATA%\learnai` on
Windows). Settings → *Afficher le fichier de progression* shows it.

⌘+ / ⌘− / ⌘0 (Ctrl on Windows and Linux) zoom the page; the − 100 % + buttons
at the right of the tabs do the same.

## Writing the course

The course is Markdown under `content/fr/`, built into the app: edit a file,
then `cargo run` again.

- `cursus.yaml` is the plan: étapes, rubriques, sous-rubriques.
- Each étape folder has `chapitre.md` (the opening story) and one file per
  rubrique. A rubrique's YAML header carries its questions (`qcm`): four
  `choix`, the `reponse` index from 0, and the `pourquoi`.
- In the lesson, `:::pourquoi … :::` makes a **Pourquoi ?** button, and a
  `mermaid` code block makes a diagram. Headings end with `{#id}`, the
  sous-rubrique's id, so a question can link to its part of the lesson.

Diagrams are drawn by Mermaid 11.4.1, carried in the app
(`assets/vendor/mermaid.min.js`), so they work offline.

## Release

`./scripts/release.sh` bumps the version, stamps `CHANGELOG.md`, tags and
pushes; the Release workflow builds and signs the macOS, Windows and Linux
apps. `./scripts/setup-github.sh` sets the secrets it needs, once.

## Licence

PolyForm Noncommercial 1.0.0; see [LICENSE](LICENSE). The name and logo are
covered by [TRADEMARK.md](TRADEMARK.md).
