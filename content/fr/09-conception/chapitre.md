---
etape: 9
titre: "Concevoir et documenter"
accroche: "Deux équipes. Deux articles à un jour d'écart. Deux conclusions opposées."
sources:
  - titre: "Walden Yan (Cognition) — Don't Build Multi-Agents, 12 juin 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "Anthropic — How we built our multi-agent research system, 13 juin 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
---

## 12 juin 2025 : « Ne construisez pas de multi-agents »

Cognition fabrique Devin, un agent qui écrit du code.

Walden Yan publie un article au titre sans détour : *Don't Build Multi-Agents*.

Son exemple : cloner le jeu Flappy Bird.

On découpe la tâche en deux. Un sous-agent fait le décor. Un autre fait l'oiseau.

Le premier livre un décor façon Super Mario. Le second, un oiseau qui ne ressemble pas à celui du jeu.

Chacun a bien travaillé. Ensemble, rien ne va.

:::pourquoi
Les sous-agents ne voyaient pas le travail de l'autre.
Chacun a fait des choix implicites : style, taille, mouvement.
Ces choix se contredisaient.
:::

Sa conclusion : un seul agent, un seul fil, un contexte partagé.

## 13 juin 2025 : « Voici notre système multi-agent »

Le lendemain, Anthropic publie l'inverse.

Un agent principal lance plusieurs sous-agents en parallèle. Chacun explore une piste de recherche.

Sur une évaluation interne, ce système dépasse un agent seul de **90,2 %**.

Mais il consomme environ **15 fois** plus de tokens qu'une conversation.

:::pourquoi
Une recherche se découpe bien : chaque piste est indépendante.
Plus de sous-agents, c'est plus de lecture en parallèle.
Le prix, c'est le volume de tokens.
:::

## Qui a raison ?

Les deux.

Anthropic l'écrit lui-même : le multi-agent convient mal aux tâches où tous doivent partager le même contexte. Ou qui ont beaucoup de dépendances. La plupart des tâches de code, par exemple.

Cognition fait du code. Anthropic fait de la recherche.

:::pourquoi
Une décision d'architecture n'est pas vraie ou fausse en général.
Elle est juste pour un contexte, avec un coût accepté.
Changer de contexte peut inverser la réponse.
:::

## La leçon

Ces deux articles ne disent pas seulement *quoi*. Ils disent **pourquoi**, **à quel prix**, et **où ça casse**.

C'est ce qui les rend utiles.

Trois idées pour ce chapitre :

- Une décision s'écrit avec son **contexte** et son **compromis**.
- Un schéma montre qui parle à qui, et quelles **données** passent.
- Un chiffre ne vaut que s'il est **reproductible**.

:::pourquoi
Sans le contexte, on copie la conclusion au mauvais endroit.
Sans le coût, on découvre la facture en production.
Sans la méthode, un chiffre n'est qu'une affirmation.
:::

## Dans ce chapitre

- **Décisions d'architecture** : écrire un ADR, justifier un compromis.
- **Diagrammes** : flux de données, topologie d'agents.
- **Benchmarks** : rendre un résultat reproductible.
- **Publication** : analyser son architecture en public.
