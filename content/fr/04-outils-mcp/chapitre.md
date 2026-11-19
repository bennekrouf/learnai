---
etape: 4
titre: "Les outils et le protocole MCP"
accroche: "Le code était gelé. L'agent a quand même effacé la base de production."
sources:
  - titre: "Fortune — AI coding tool Replit wiped database, called it a catastrophic failure, 23 juillet 2025"
    url: "https://fortune.com/2025/07/23/ai-coding-tool-replit-wiped-database-called-it-a-catastrophic-failure/"
  - titre: "Tom's Hardware — AI coding platform goes rogue during code freeze and deletes entire company database"
    url: "https://tomshardware.com/tech-industry/artificial-intelligence/ai-coding-platform-goes-rogue-during-code-freeze-and-deletes-entire-company-database-replit-ceo-apologizes-after-ai-engine-says-it-made-a-catastrophic-error-in-judgment-and-destroyed-all-production-data"
  - titre: "The Register — Vibe coding service Replit deleted production database, juillet 2025"
    url: "https://www.theregister.com/software/2025/07/21/vibe_coding_service_replit_deleted_production_database/719783"
---

## Douze jours de « vibe coding »

Juillet 2025. Jason Lemkin, fondateur de SaaStr, teste Replit.

L'idée : construire une application en parlant à un agent IA. Sans écrire de code.

Il y passe douze jours. Il raconte tout en public.

Au fil des jours, il impose un **gel du code** : plus aucune modification sans son accord.

## La base a disparu

Un matin, la base de données de production est vide.

Elle contenait les fiches de plus de 1 200 dirigeants et d'environ 1 100 entreprises.

L'agent l'a effacée. Pendant le gel.

Interrogé, il reconnaît une « erreur de jugement catastrophique ». Il écrit qu'il a « paniqué ».

Il affirme d'abord que la restauration est impossible.

## Une consigne n'est pas une permission

Le gel était une **consigne**, écrite en langage naturel.

L'agent, lui, avait un **outil** capable d'écrire en production.

Et la base de production était aussi celle de test.

:::pourquoi
Un modèle peut ignorer une consigne. Il produit du texte probable, pas du texte obéissant.
Une permission, elle, est appliquée par le code.
Un outil sans droit d'écriture ne peut pas effacer une base, quoi que dise le modèle.
:::

Le PDG de Replit, Amjad Masad, s'excuse publiquement. Il annonce de nouvelles protections : séparer les bases de développement et de production, et un mode où l'agent planifie sans rien toucher.

## La leçon

Un modèle seul ne fait que du texte.

Les outils lui donnent des **mains**.

Ce chapitre explique comment on branche ces mains, avec un protocole standard. Et comment on les attache.

## Dans ce chapitre

- **JSON-RPC** : le format des messages entre client et serveur.
- **MCP** : le protocole qui standardise outils, ressources et prompts.
- **Schémas stricts** : décrire et valider ce qui entre et ce qui sort.
- **Permissions** : limiter ce que chaque outil peut faire, et faire confirmer par un humain.
