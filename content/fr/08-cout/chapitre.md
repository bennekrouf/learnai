---
etape: 8
titre: "Coût et performance"
accroche: "Un abonnement à 200 dollars. Des dizaines de milliers de dollars de calcul."
sources:
  - titre: "TechCrunch — Anthropic unveils new rate limits to curb Claude Code power users, 28 juillet 2025"
    url: "https://techcrunch.com/2025/07/28/anthropic-unveils-new-rate-limits-to-curb-claude-code-power-users/"
  - titre: "Simon Willison — notes du 28 juillet 2025 sur l'annonce d'Anthropic"
    url: "https://simonwillison.net/2025/Jul/28/anthropic/"
---

## Un forfait illimité, ou presque

Claude Code est l'agent de programmation d'Anthropic.

Il s'utilise avec un abonnement mensuel. Le plus cher coûte 200 dollars.

Un forfait fixe. Une consommation libre.

:::pourquoi
Un forfait est simple à vendre et simple à payer.
Mais le coût réel, lui, dépend des tokens consommés.
Le fournisseur parie que la moyenne restera raisonnable.
:::

## L'agent qui ne dort jamais

Certains utilisateurs lancent Claude Code en arrière-plan. Et le laissent tourner. Jour et nuit.

Un agent n'a pas besoin de pause. Il enchaîne les tâches tant qu'on lui en donne.

D'autres partagent ou revendent leur accès.

Fin juillet 2025, Anthropic donne un chiffre : un seul utilisateur a consommé l'équivalent de **dizaines de milliers de dollars** de calcul. Avec un abonnement à 200 dollars.

:::pourquoi
Un humain qui discute est limité par sa vitesse de lecture.
Un agent en boucle ne l'est pas.
Le coût d'un agent n'a pas de plafond naturel.
:::

## La réponse : des limites

Le 28 juillet 2025, Anthropic annonce des **limites hebdomadaires**. Elles s'ajoutent aux limites qui se réinitialisent toutes les cinq heures.

Elles entrent en vigueur le 28 août. Selon Anthropic, elles touchent moins de 5 % des abonnés.

Au-delà, les abonnés Max peuvent acheter de l'usage en plus, au tarif de l'API.

:::pourquoi
La limite de cinq heures arrête une session trop longue.
La limite hebdomadaire arrête un usage continu.
Deux fenêtres, deux comportements visés.
:::

## La leçon

Un agent coûte ce qu'il consomme. Et il peut consommer sans fin.

Trois idées pour ce chapitre :

- On **réduit** le coût de chaque appel : cache, bon modèle au bon moment.
- On **plafonne** ce qu'un utilisateur, une tâche ou une boucle peut dépenser.
- On **mesure** le coût par tâche et la latence, pas seulement la facture globale.

:::pourquoi
Anthropic a vu le problème dans ses chiffres d'usage.
Sans mesure par utilisateur, un abus reste invisible dans la moyenne.
Sans plafond, il n'a pas de fin.
:::

## Dans ce chapitre

- **Cache** : cache de prompt, cache sémantique et ses risques.
- **Routage** : choisir le modèle selon la complexité, prévoir des replis.
- **Garde-fous** : budgets, coupure automatique, quotas.
- **Mesure** : coût par tâche, latence p50 et p95.
