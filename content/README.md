# Contenu LearnKit

Le contenu est indépendant de l'app. Une langue par dossier (`fr/`, plus tard `en/`).

## Arborescence

```
content/fr/
  cursus.yaml                 # plan complet : étapes → rubriques → sous-rubriques
  00-fondations/              # une étape = un chapitre
    chapitre.md               # l'anecdote d'ouverture
    01-tokens-et-contexte.md  # une rubrique = un fichier
    02-generation.md
    ...
```

## Chapitre (`chapitre.md`)

Frontmatter :

| Champ      | Rôle                                   |
|------------|----------------------------------------|
| `etape`    | numéro de l'étape                      |
| `titre`    | titre du chapitre                      |
| `accroche` | une phrase, affichée sur la carte      |
| `sources`  | liste `{titre, url}` de l'anecdote     |

Le corps raconte l'anecdote, puis annonce ce que le chapitre explique.

## Rubrique (`NN-slug.md`)

Frontmatter :

```yaml
id: "00-01"
titre: "Tokens et contexte"
sous_rubriques:
  - id: tokens
    titre: "Ce qu'est un token"
qcm:
  - id: tokens-1
    sous_rubrique: tokens      # renvoie à sous_rubriques[].id
    question: "..."
    choix: ["...", "...", "...", "..."]
    reponse: 1                 # index (base 0) du bon choix
    pourquoi: >-
      Explication, cachée derrière le bouton « Pourquoi ? ».
```

Le corps est la fiche : un titre `## …` par sous-rubrique, avec un ancre `{#id}`.

### Ordre d'affichage dans l'app

1. QCM de la rubrique (content first).
2. Après chaque réponse, bouton **Pourquoi ?** → affiche `pourquoi`.
3. Puis la fiche.

### Blocs spéciaux dans la fiche

Pourquoi repliable (même bouton que dans le QCM) :

```
:::pourquoi
Phrases courtes.
:::
```

Schéma : un bloc ` ```mermaid `.

## Règles d'écriture

- Phrases courtes. Une idée par phrase. Viser moins de 15 mots.
- Chaque affirmation clé a son `:::pourquoi`. Chaque question a son `pourquoi`.
- Dès que 3 acteurs ou plus interagissent : un schéma mermaid.
- Chaque chapitre s'ouvre sur une anecdote réelle et sourcée.
- Les chaînes YAML sont entre guillemets doubles. Pas de `"` à l'intérieur : utiliser « ».
- Bonne réponse répartie entre A, B, C et D dans chaque rubrique. L'app mélange aussi les choix à l'affichage.
