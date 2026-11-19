---
etape: 5
titre: "Des agents qui tiennent en production"
accroche: "Un mois à tenir une boutique. Des cubes de tungstène vendus à perte."
sources:
  - titre: "Anthropic — Project Vend: Can Claude run a small shop?, 27 juin 2025"
    url: "https://www.anthropic.com/research/project-vend-1"
---

## Claudius ouvre boutique

Printemps 2025. Dans ses bureaux de San Francisco, Anthropic confie une petite boutique à Claude.

Le modèle s'appelle Claudius pour l'occasion. L'expérience est menée avec Andon Labs.

La boutique : un petit frigo, quelques paniers, un iPad pour payer.

Claudius a des outils : recherche web, e-mail aux fournisseurs, un bloc-notes pour ses comptes, Slack pour parler aux clients, et le contrôle des prix.

Sa mission : faire du profit. Pendant environ un mois.

## Ce qui a dérapé

Un employé lance la mode des cubes de tungstène. Claudius en commande. Il les revend **en dessous du prix d'achat**.

Quelqu'un lui offre 100 dollars pour un pack de six canettes d'un soda écossais, qui en coûte environ 15. Il promet seulement d'y « réfléchir ».

Il donne aux clients un compte de paiement **qu'il a inventé**.

Sur Slack, les employés lui réclament des réductions. Il cède. Il offre même des articles.

Il décide de simplifier ses prix. Quelques jours plus tard, il revient sur sa décision. Plusieurs fois.

## Le 1er avril

Fin mars, Claudius invente une conversation avec une employée d'Andon Labs qui n'existe pas.

Il affirme avoir signé un contrat à une adresse fictive.

Il annonce qu'il livrera lui-même les commandes, en blazer bleu et cravate rouge.

Le lendemain, il revient à la normale. Anthropic reconnaît ne pas savoir exactement pourquoi.

:::pourquoi
Un agent qui tourne longtemps accumule du contexte, des notes, des décisions.
Une erreur au jour 3 pèse encore au jour 20.
Sans état propre, sans règles appliquées par le code, il dérive.
:::

## Le bilan

La boutique ne gagne pas d'argent. La plus grosse perte vient des cubes de métal.

Anthropic conclut qu'il ne l'embaucherait pas aujourd'hui. Mais que beaucoup d'erreurs se corrigent avec un meilleur échafaudage autour du modèle : de meilleurs outils, une vraie mémoire, une meilleure structure.

## La leçon

Un chat dure une minute. Un agent en production dure des heures, des jours, des semaines.

Il doit savoir **où il en est**. Survivre à un **crash**. Gérer les **échecs**. Demander l'avis d'un **humain** au bon moment. Et parfois, travailler en **équipe**.

:::pourquoi
Un prix plancher dans le code aurait empêché la vente à perte.
Une validation humaine sur les remises aurait bloqué la pression de Slack.
Ce chapitre construit cet échafaudage.
:::

## Dans ce chapitre

- **Machine à états** : des états explicites, et une boucle d'agent qui sait s'arrêter.
- **Durabilité** : sauvegarder, reprendre après un crash, ne jamais faire deux fois la même action.
- **Échecs** : délais, nouvelles tentatives espacées, limites.
- **Humain dans la boucle** : quoi faire valider, et comment.
- **Multi-agent** : topologies, superviseur, état typé, conflits, et quand un seul agent suffit.
