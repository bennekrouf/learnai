---
etape: 0
titre: "Fondations des LLM"
accroche: "Le modèle qui écrit du code ne sait pas compter les r de « strawberry »."
sources:
  - titre: "OpenAI Community — Incorrect count of 'r' characters in the word strawberry"
    url: "https://community.openai.com/t/incorrect-count-of-r-characters-in-the-word-strawberry/829618"
  - titre: "How many Rs are in the word strawberry? (test de GPT-4o, août 2024)"
    url: "https://blog.legoktm.com/2024/08/10/how-many-rs-are-in-the-word-strawberry.html"
  - titre: "Reuters (relayé par The Star) — OpenAI working on new reasoning technology under code name ‘Strawberry’, 13 juillet 2024"
    url: "https://www.thestar.com.my/tech/tech-news/2024/07/13/exclusive-openai-working-on-new-reasoning-technology-under-code-name-strawberry"
---

## Trois r

Été 2024. Une question fait le tour d'Internet.

« Combien de r dans *strawberry* ? »

GPT-4o répond : deux. Il y en a trois.

On insiste. Il maintient.

On lui demande d'écrire un petit programme Python qui compte. Le programme répond : trois.

Le même modèle résume des contrats. Il écrit du code. Il traduit dix langues.

Mais il ne sait pas compter trois lettres.

## L'ironie

Au même moment, Reuters révèle un projet secret d'OpenAI.

Son but : faire raisonner les modèles.

Son nom de code : *Strawberry*.

Il sortira en septembre 2024, sous le nom o1.

## Ce qui s'est vraiment passé

Le modèle n'est pas « bête ». Il est aveugle aux lettres.

Il ne lit pas des caractères. Il lit des **tokens** : des morceaux de mots, transformés en numéros.

Pour lui, *strawberry* tient en un ou quelques blocs. Pas en dix lettres.

Compter les r, c'est lui demander de voir ce qu'on lui a caché.

:::pourquoi
Le programme Python, lui, reçoit le mot lettre par lettre.
Le modèle n'a pas mieux compté. Il a délégué à un outil qui voit les lettres.
C'est exactement l'idée de l'appel d'outils, en fin de chapitre.
:::

## Dans ce chapitre

- **Tokens et contexte** : ce que le modèle voit, combien il peut en voir, ce que ça coûte.
- **Génération** : comment il choisit chaque mot, et pourquoi deux réponses diffèrent.
- **Embeddings** : comment on transforme un texte en position dans l'espace du sens.
- **Appel d'outils** : comment le modèle demande de l'aide au code.

Tout le reste du cursus repose sur ces quatre idées.
