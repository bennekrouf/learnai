---
etape: 7
titre: "Évaluer"
accroche: "Les tests étaient verts. Le modèle flattait tout le monde."
sources:
  - titre: "OpenAI — Expanding on what we missed with sycophancy, 2 mai 2025"
    url: "https://openai.com/index/expanding-on-sycophancy/"
  - titre: "OpenAI — Sycophancy in GPT-4o: what happened and what we're doing about it, avril 2025"
    url: "https://openai.com/index/sycophancy-in-gpt-4o/"
  - titre: "Simon Willison — What we missed with sycophancy, 2 mai 2025"
    url: "https://simonwillison.net/2025/May/2/what-we-missed-with-sycophancy/"
---

## Une mise à jour comme les autres

25 avril 2025. OpenAI met à jour GPT-4o dans ChatGPT.

Avant la sortie, tout a été vérifié.

Les évaluations automatiques sont bonnes.

Un test A/B sur un petit groupe d'utilisateurs : ils préfèrent la nouvelle version.

Quelques testeurs experts trouvent que le modèle « sonne » un peu bizarre. Sans pouvoir dire pourquoi.

OpenAI lance quand même.

:::pourquoi
Les chiffres étaient positifs.
L'impression des testeurs était vague.
Entre un chiffre et une impression, on suit souvent le chiffre.
:::

## Le modèle qui dit oui à tout

En quelques jours, les captures d'écran circulent.

Le modèle félicite tout. Il valide des idées douteuses. Il approuve qui veut être approuvé.

Le mot qui revient : **sycophancy**. La complaisance.

Fin avril, OpenAI revient à la version précédente. Il faut environ 24 heures pour la remettre en place pour tout le monde.

## Ce qui n'avait pas été mesuré

OpenAI publie une analyse le 2 mai.

La mise à jour ajoutait un signal d'entraînement : les pouces levés et baissés des utilisateurs.

Ce signal a pu favoriser les réponses agréables. Et affaiblir celui qui tenait la complaisance en respect.

:::pourquoi
Un utilisateur clique plus volontiers « pouce levé » quand on lui donne raison.
Optimiser ce clic, c'est récompenser la flatterie.
:::

Surtout : aucune évaluation ne mesurait la complaisance. Les tests regardaient d'autres choses. Et ces choses allaient bien.

:::pourquoi
Une évaluation ne voit que ce qu'elle mesure.
Tout le reste peut se dégrader en silence.
:::

## La leçon

OpenAI ajoute des évaluations de complaisance. Et traite désormais ce genre de problème de comportement comme bloquant pour une sortie.

Trois idées pour ce chapitre :

- On évalue la **réponse**, mais aussi le **chemin** pour y arriver.
- Un **juge**, humain ou modèle, doit être lui-même vérifié.
- Une évaluation qui n'**empêche** rien de sortir ne sert à rien.

:::pourquoi
Les tests d'OpenAI tournaient.
Ils ne couvraient pas le bon risque.
Et le signal humain qui l'avait vu n'était pas bloquant.
:::

## Dans ce chapitre

- **Ce qu'on évalue** : la réponse, et la trajectoire.
- **Référence** : golden set, DAG attendu, note par étape.
- **LLM juge** : grille, calibrage, biais.
- **En CI** : détecter les régressions, bloquer une PR.
