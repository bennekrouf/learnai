---
id: "08-03"
titre: "Garde-fous"
sous_rubriques:
  - id: budgets
    titre: "Budgets"
  - id: coupure
    titre: "Coupure automatique"
  - id: quotas
    titre: "Quotas"
sources:
  - titre: "TechCrunch — Anthropic unveils new rate limits to curb Claude Code power users, 28 juillet 2025"
    url: "https://techcrunch.com/2025/07/28/anthropic-unveils-new-rate-limits-to-curb-claude-code-power-users/"
  - titre: "OWASP — LLM10:2025 Unbounded Consumption"
    url: "https://genai.owasp.org/llmrisk/llm102025-unbounded-consumption/"
qcm:
  - id: budgets-1
    sous_rubrique: budgets
    question: "Pourquoi un agent a-t-il besoin d'un budget, alors qu'un simple appel à un modèle s'en passe souvent ?"
    choix:
      - "C'est une obligation légale"
      - "Un agent coûte toujours moins cher"
      - "Les agents n'utilisent pas de tokens"
      - "Un agent enchaîne les appels en boucle : rien ne borne naturellement son coût"
    reponse: 3
    pourquoi: >-
      Un appel unique a un coût prévisible.
      Un agent décide lui-même de rappeler le modèle.
      Une boucle mal finie tourne jusqu'à ce qu'on l'arrête.
      Le budget est cet arrêt.

  - id: budgets-2
    sous_rubrique: budgets
    question: "À quels niveaux fixer un budget ?"
    choix:
      - "Seulement pour toute l'organisation, une fois par an"
      - "Par tâche, par utilisateur ou client, et pour toute l'organisation"
      - "Seulement par requête"
      - "Aucun niveau, on regarde la facture"
    reponse: 1
    pourquoi: >-
      Par tâche : une boucle ne ruine pas tout.
      Par utilisateur : un abus reste local.
      Pour l'organisation : la facture du mois reste sous contrôle.
      Chaque niveau arrête un problème différent.

  - id: budgets-3
    sous_rubrique: budgets
    question: "En quoi exprimer le budget d'une tâche d'agent ?"
    choix:
      - "En plusieurs limites : tokens, nombre d'étapes, appels d'outils, durée, montant"
      - "Seulement en nombre de mots de la réponse"
      - "En nombre d'utilisateurs"
      - "En pourcentage de CPU"
    reponse: 0
    pourquoi: >-
      Les tokens mesurent le coût du modèle.
      Les étapes et appels d'outils mesurent la boucle.
      La durée mesure l'attente.
      Un outil payant compte en argent, pas en tokens.

  - id: budgets-4
    sous_rubrique: budgets
    question: "Une tâche atteint 80 % de son budget. Que faire ?"
    choix:
      - "Ne rien faire jusqu'à 100 %"
      - "Doubler le budget automatiquement"
      - "Alerter, et laisser l'agent conclure avec ce qu'il a"
      - "Effacer le travail fait"
    reponse: 2
    pourquoi: >-
      Un seuil d'alerte avant la limite donne le temps de réagir.
      L'agent peut résumer et rendre un résultat partiel.
      À 100 %, on coupe net.
      Un budget qui s'agrandit seul n'est pas un budget.

  - id: coupure-1
    sous_rubrique: coupure
    question: "Quel signal doit couper automatiquement un agent ?"
    choix:
      - "Une réponse polie"
      - "Le même appel d'outil répété plusieurs fois avec les mêmes arguments"
      - "Une question courte"
      - "Un utilisateur connecté"
    reponse: 1
    pourquoi: >-
      Un appel identique répété signale une boucle.
      L'agent ne progresse plus. Il consomme.
      Le détecter tôt évite de payer cent fois le même échec.

  - id: coupure-2
    sous_rubrique: coupure
    question: "Où placer le code qui coupe un agent ?"
    choix:
      - "Nulle part, l'agent s'arrête seul"
      - "Dans le prompt : « Arrête-toi si tu boucles »"
      - "Dans la réponse du modèle"
      - "Dans l'orchestrateur, hors du modèle"
    reponse: 3
    pourquoi: >-
      Le modèle qui boucle ne sait pas qu'il boucle.
      Une consigne dans le prompt ne garantit rien.
      L'orchestrateur compte les étapes et les tokens.
      Il coupe, que le modèle soit d'accord ou non.

  - id: coupure-3
    sous_rubrique: coupure
    question: "Après une coupure automatique, que doit-il rester ?"
    choix:
      - "Une relance automatique immédiate de la même tâche"
      - "Rien, on efface tout"
      - "Un état sauvegardé, la raison de l'arrêt, et un message clair pour l'utilisateur"
      - "Une facture envoyée au client"
    reponse: 2
    pourquoi: >-
      L'état permet de reprendre après correction.
      La raison aide à corriger la cause.
      Le message évite une attente sans fin.
      Relancer à l'identique refait la même boucle.

  - id: coupure-4
    sous_rubrique: coupure
    question: "La dépense de toute l'organisation triple en une heure. Que doit faire un disjoncteur ?"
    choix:
      - "Suspendre les nouvelles tâches non critiques et alerter une personne"
      - "Attendre la facture de fin de mois"
      - "Supprimer les comptes des utilisateurs"
      - "Passer tout le trafic au plus gros modèle"
    reponse: 0
    pourquoi: >-
      Un pic soudain vient d'un bug, d'une boucle ou d'un abus.
      Attendre la facture, c'est payer avant de comprendre.
      Le disjoncteur limite les dégâts.
      Une personne décide ensuite.

  - id: quotas-1
    sous_rubrique: quotas
    question: "Quelle différence entre un budget et un quota ?"
    choix:
      - "Le quota est toujours en euros"
      - "Aucune différence"
      - "Le budget borne une dépense, le quota borne un usage par période pour un utilisateur ou un client"
      - "Le budget concerne seulement le matériel"
    reponse: 2
    pourquoi: >-
      Budget : combien une tâche ou l'organisation peut coûter.
      Quota : combien un utilisateur peut consommer, par heure, par jour, par semaine.
      Le quota protège le service et les autres utilisateurs.

  - id: quotas-2
    sous_rubrique: quotas
    question: "Pourquoi Anthropic a-t-il ajouté une limite hebdomadaire à la limite de cinq heures pour Claude Code ?"
    choix:
      - "La limite de cinq heures n'arrêtait pas un usage continu, jour et nuit"
      - "Pour ralentir tous les utilisateurs"
      - "Parce que la limite de cinq heures était trop stricte"
      - "Pour supprimer les abonnements"
    reponse: 0
    pourquoi: >-
      Une fenêtre de cinq heures se réinitialise sans cesse.
      Un agent en arrière-plan la consomme, attend, recommence.
      Une fenêtre longue borne la consommation totale.
      Deux fenêtres, deux comportements visés.

  - id: quotas-3
    sous_rubrique: quotas
    question: "Un utilisateur atteint son quota. Quelle est la meilleure réponse du produit ?"
    choix:
      - "Couper le compte sans prévenir"
      - "Renvoyer une erreur 500 sans explication"
      - "Faire semblant de répondre"
      - "Le dire clairement : ce qui est atteint, quand ça se réinitialise, et l'option payante s'il y en a une"
    reponse: 3
    pourquoi: >-
      Un quota surprise passe pour une panne.
      Un quota expliqué est une règle.
      Anthropic propose aux abonnés Max d'acheter de l'usage en plus au tarif de l'API.
      L'utilisateur choisit en connaissance de cause.

  - id: quotas-4
    sous_rubrique: quotas
    question: "Quel risque OWASP vise la consommation sans limite d'un service LLM ?"
    choix:
      - "Une simple faute d'orthographe"
      - "LLM10 « Unbounded Consumption » : coûts qui explosent, déni de service, vol de modèle par requêtes massives"
      - "Le choix de la police d'écriture"
      - "Aucun, ce n'est pas un risque de sécurité"
    reponse: 1
    pourquoi: >-
      OWASP classe la consommation sans limite parmi les dix risques des applications LLM.
      Un attaquant peut faire payer la victime en saturant son service.
      Les quotas et les limites de débit sont la défense de base.

  - id: quotas-5
    sous_rubrique: quotas
    question: "Un client partage son accès avec dix personnes. Quel garde-fou le détecte ?"
    choix:
      - "Le cache sémantique"
      - "Aucun, c'est indétectable"
      - "Un prompt système plus long"
      - "Un quota par compte, et une alerte sur un usage anormal pour un seul utilisateur"
    reponse: 3
    pourquoi: >-
      Dix personnes consomment comme dix.
      Le quota plafonne le compte.
      L'alerte signale un profil d'usage anormal.
      Anthropic citait le partage et la revente d'accès parmi les abus visés.
---

## Budgets {#budgets}

Un appel à un modèle a un coût prévisible. Un **agent**, non.

:::pourquoi
L'agent décide lui-même de rappeler le modèle.
Une boucle mal finie tourne tant qu'on ne l'arrête pas.
Chez Anthropic, un abonné à 200 dollars a consommé des dizaines de milliers de dollars de calcul.
:::

Un budget à **trois niveaux** :

| Niveau | Exemple | Ce qu'il arrête |
|---|---|---|
| Tâche | 200 000 tokens, 30 étapes, 10 minutes | Une boucle |
| Utilisateur ou client | 20 € par jour | Un abus |
| Organisation | 5 000 € par mois | Une facture hors de contrôle |

:::pourquoi
Chaque niveau arrête un problème différent.
Un seul niveau laisse passer les deux autres.
:::

Le budget d'une tâche combine plusieurs **unités** : tokens, étapes, appels d'outils, durée, argent.

:::pourquoi
Un outil payant, comme une recherche web, se compte en argent.
Une boucle d'appels courts se voit en étapes, pas en tokens.
:::

Une **alerte** avant la limite. Une **coupure** à la limite.

:::pourquoi
À 80 %, l'agent peut conclure avec ce qu'il a.
À 100 %, on coupe.
Un budget qui s'agrandit seul n'est pas un budget.
:::

## Coupure automatique {#coupure}

Certains signaux doivent **arrêter** l'agent sans attendre le budget.

| Signal | Ce qu'il révèle |
|---|---|
| Même appel d'outil, mêmes arguments, plusieurs fois | Boucle |
| Même erreur d'outil répétée | Schéma mal compris |
| Aucune progression vers le but après N étapes | Agent perdu |
| Dépense globale qui triple en une heure | Bug, boucle ou abus |

:::pourquoi
Un agent qui boucle ne progresse plus. Il consomme.
Le détecter tôt évite de payer cent fois le même échec.
:::

La coupure vit dans l'**orchestrateur**, pas dans le prompt.

```mermaid
sequenceDiagram
  participant O as Orchestrateur
  participant M as Modèle
  participant T as Outil
  participant C as Compteurs
  participant U as Utilisateur
  O->>M: Étape 12
  M-->>O: rechercher("facture 512")
  O->>C: Même appel déjà vu 3 fois ?
  C-->>O: Oui
  O->>O: Sauvegarde l'état, motif « boucle »
  O-->>U: « Tâche arrêtée : l'agent tournait en rond. Reprise possible. »
  Note over O,T: L'outil n'est pas rappelé
```

:::pourquoi
Le modèle qui boucle ne sait pas qu'il boucle.
Une consigne « arrête-toi si tu boucles » ne garantit rien.
Le code compte, et coupe.
:::

Après la coupure : un **état sauvegardé**, le **motif**, un **message clair**.

:::pourquoi
L'état permet de reprendre après correction.
Le motif aide à corriger la cause.
Relancer à l'identique refait la même boucle.
:::

Au niveau de l'organisation, un **disjoncteur** suspend le non-critique et alerte une personne.

:::pourquoi
Attendre la facture, c'est payer avant de comprendre.
:::

## Quotas {#quotas}

Le **quota** borne l'usage d'un utilisateur ou d'un client sur une période.

:::pourquoi
Le budget protège ta facture.
Le quota protège aussi le service et les autres utilisateurs.
:::

Deux **fenêtres** se complètent :

| Fenêtre | Ce qu'elle arrête |
|---|---|
| Courte (minute, quelques heures) | Les rafales |
| Longue (jour, semaine) | L'usage continu |

:::pourquoi
Anthropic avait déjà une limite qui se réinitialise toutes les cinq heures.
Des agents tournaient jour et nuit, d'une fenêtre à l'autre.
La limite hebdomadaire, annoncée en juillet 2025, borne le total.
:::

Le quota vise aussi le **partage** et la **revente** d'accès.

:::pourquoi
Dix personnes sur un compte consomment comme dix.
Anthropic citait ces pratiques parmi les abus visés.
:::

Un quota atteint s'**explique** : quoi, jusqu'à quand, et l'option payante s'il y en a une.

:::pourquoi
Un quota surprise passe pour une panne.
Un quota expliqué est une règle.
:::

OWASP en fait un risque à part entière : **LLM10, Unbounded Consumption**.

:::pourquoi
Un attaquant peut saturer ton service et te faire payer la note.
Quotas et limites de débit sont la défense de base.
:::

## À retenir

- Un agent n'a pas de plafond naturel : il faut lui en donner un.
- Budgets par tâche, par utilisateur, pour l'organisation. Alerte avant, coupure à la limite.
- La coupure vit dans le code de l'orchestrateur, pas dans le prompt.
- Quotas sur deux fenêtres, expliqués à l'utilisateur.
