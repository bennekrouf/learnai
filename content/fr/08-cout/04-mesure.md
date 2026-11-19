---
id: "08-04"
titre: "Mesure"
sous_rubriques:
  - id: cout-tache
    titre: "Coût par tâche"
  - id: latence
    titre: "Latence p50 et p95"
sources:
  - titre: "Dean et Barroso — The Tail at Scale, Communications of the ACM, 2013"
    url: "https://research.google/pubs/pub40801/"
  - titre: "Anthropic — Prompt caching (documentation)"
    url: "https://platform.claude.com/docs/en/build-with-claude/prompt-caching"
qcm:
  - id: cout-tache-1
    sous_rubrique: cout-tache
    question: "Pourquoi mesurer le coût par tâche plutôt que la facture mensuelle seule ?"
    choix:
      - "La facture mensuelle est toujours fausse"
      - "La facture dit combien, le coût par tâche dit où et pourquoi"
      - "Le coût par tâche est obligatoire"
      - "Il n'y a aucune différence"
    reponse: 1
    pourquoi: >-
      La facture additionne tout.
      Une tâche qui coûte 50 fois la moyenne s'y noie.
      Par tâche, on voit quel flux, quel client, quel prompt coûte.
      On sait quoi optimiser.

  - id: cout-tache-2
    sous_rubrique: cout-tache
    question: "Que faut-il additionner pour le coût d'une tâche d'agent ?"
    choix:
      - "Tous les appels au modèle (entrée, cache, sortie, raisonnement), les outils payants et les nouveaux essais"
      - "Seulement le dernier appel au modèle"
      - "Seulement les tokens de la réponse finale"
      - "Seulement le prix de l'abonnement"
    reponse: 0
    pourquoi: >-
      Un agent fait souvent dix ou trente appels pour une tâche.
      Chaque appel a ses tokens d'entrée et de sortie.
      Les essais ratés et les outils payants comptent aussi.
      Oublier une ligne, c'est sous-estimer le coût.

  - id: cout-tache-3
    sous_rubrique: cout-tache
    question: "100 tâches coûtent 50 € au total. 80 réussissent. Quel est le coût par tâche réussie ?"
    choix:
      - "0,40 €"
      - "0,50 €"
      - "0,625 € : les échecs se paient aussi"
      - "On ne compte pas les échecs"
    reponse: 2
    pourquoi: >-
      50 € divisé par 80 réussites = 0,625 €.
      Les 20 échecs ont coûté sans rien rapporter.
      Un modèle moins cher qui échoue plus peut coûter plus cher par réussite.
      C'est le chiffre qui compte pour comparer.

  - id: cout-tache-4
    sous_rubrique: cout-tache
    question: "À chaque étape, l'agent renvoie tout l'historique au modèle. Comment évolue le coût ?"
    choix:
      - "Chaque étape coûte pareil"
      - "Chaque étape coûte plus que la précédente : le coût total grandit plus vite que le nombre d'étapes"
      - "Le coût baisse à chaque étape"
      - "Seule la première étape coûte"
    reponse: 1
    pourquoi: >-
      L'étape 20 renvoie les 19 étapes précédentes.
      Les tokens d'entrée s'accumulent.
      Le cache de prompt réduit ce coût, car le début de l'historique ne change pas.
      Résumer l'historique aussi.

  - id: cout-tache-5
    sous_rubrique: cout-tache
    question: "Comment attribuer chaque dépense à la bonne tâche, au bon client, à la bonne fonctionnalité ?"
    choix:
      - "Demander au modèle"
      - "Deviner à la fin du mois"
      - "Répartir la facture à parts égales"
      - "Étiqueter chaque appel avec l'identifiant de tâche, de client et de fonctionnalité"
    reponse: 3
    pourquoi: >-
      Sans étiquette, un coût n'a pas de propriétaire.
      Avec étiquette, on additionne par tâche, client, fonctionnalité.
      C'est aussi ce qui alimente budgets et quotas.

  - id: cout-tache-6
    sous_rubrique: cout-tache
    question: "Les tokens de sortie coûtent plusieurs fois plus cher que ceux d'entrée chez la plupart des fournisseurs. Conséquence ?"
    choix:
      - "Aucune conséquence"
      - "Il faut allonger les réponses"
      - "Demander des réponses concises et des formats structurés réduit vite le coût"
      - "Il faut supprimer le prompt système"
    reponse: 2
    pourquoi: >-
      Une réponse verbeuse coûte cher à chaque appel.
      Un JSON court au lieu d'un paragraphe : moins de sortie.
      Les tokens de raisonnement se facturent souvent comme de la sortie.
      Regarder les prix d'entrée et de sortie séparément.

  - id: latence-1
    sous_rubrique: latence
    question: "Que signifie une latence p95 de 8 secondes ?"
    choix:
      - "95 % des requêtes répondent en 8 secondes ou moins, 5 % prennent plus"
      - "La moyenne est de 8 secondes"
      - "95 requêtes ont pris 8 secondes"
      - "La requête la plus lente a pris 8 secondes"
    reponse: 0
    pourquoi: >-
      p50 : la moitié des requêtes est plus rapide.
      p95 : 19 sur 20 sont plus rapides.
      Le dernier vingtième, c'est la queue.
      Un utilisateur actif la rencontre souvent.

  - id: latence-2
    sous_rubrique: latence
    question: "Pourquoi ne pas se contenter de la latence moyenne ?"
    choix:
      - "La moyenne est toujours plus haute que p95"
      - "La moyenne est impossible à calculer"
      - "Quelques requêtes très lentes se cachent dans une moyenne correcte"
      - "Il n'y a pas de raison"
    reponse: 2
    pourquoi: >-
      99 requêtes à 1 seconde et une à 60 secondes : moyenne 1,6 seconde.
      La moyenne paraît bonne.
      Un utilisateur sur cent a attendu une minute.
      Les percentiles montrent ce que la moyenne lisse.

  - id: latence-3
    sous_rubrique: latence
    question: "Dans « The Tail at Scale » (2013), une requête interroge 100 serveurs. Chacun est lent une fois sur 100. Quelle part des requêtes est lente ?"
    choix:
      - "Environ 63 %"
      - "1 %"
      - "10 %"
      - "100 %"
    reponse: 0
    pourquoi: >-
      La requête attend le plus lent des 100.
      Probabilité qu'aucun ne soit lent : 0,99 puissance 100, environ 37 %.
      Donc environ 63 % des requêtes sont lentes.
      Plus il y a d'étapes, plus la queue domine.

  - id: latence-4
    sous_rubrique: latence
    question: "Un agent enchaîne 10 appels au modèle, l'un après l'autre. Quel effet sur la latence ?"
    choix:
      - "Aucun effet"
      - "La latence est celle d'un seul appel"
      - "La latence baisse à chaque appel"
      - "Les latences s'additionnent, et le risque de tomber sur un appel lent grandit"
    reponse: 3
    pourquoi: >-
      Dix appels en série, ce sont dix attentes.
      Chaque appel a sa chance d'être lent.
      Le p95 de la tâche est bien pire que le p95 d'un appel.
      Moins d'étapes, ou des étapes en parallèle, réduisent l'attente.

  - id: latence-5
    sous_rubrique: latence
    question: "Quelle mesure reflète le mieux l'attente ressentie dans un chat avec affichage progressif ?"
    choix:
      - "La taille du modèle"
      - "Le temps jusqu'au premier token"
      - "Le nombre de tokens du prompt système"
      - "Le prix de la requête"
    reponse: 1
    pourquoi: >-
      Avec l'affichage progressif, l'utilisateur lit dès le premier mot.
      Ce premier mot fixe l'impression de rapidité.
      Le temps total compte pour un agent qui doit finir avant d'agir.
      On mesure les deux.

  - id: latence-6
    sous_rubrique: latence
    question: "Où mesurer la latence d'un agent ?"
    choix:
      - "Nulle part"
      - "Seulement côté fournisseur"
      - "Seulement sur la machine du développeur"
      - "Par étape (modèle, chaque outil) et de bout en bout, en p50 et p95"
    reponse: 3
    pourquoi: >-
      De bout en bout : ce que vit l'utilisateur.
      Par étape : où passe le temps.
      Un outil lent se cache derrière un modèle accusé à tort.
      La trace de l'agent porte ces mesures.
---

## Coût par tâche {#cout-tache}

La facture dit **combien**. Le coût par tâche dit **où** et **pourquoi**.

:::pourquoi
Une tâche qui coûte 50 fois la moyenne se noie dans la facture.
Par tâche, on voit quel flux, quel client, quel prompt coûte.
:::

Une tâche d'agent, c'est plusieurs appels. On additionne tout :

| Ligne | Exemple |
|---|---|
| Tokens d'entrée | Prompt, historique, résultats d'outils |
| Tokens lus en cache | Préfixe stable, à prix réduit |
| Tokens de sortie | Réponses, appels d'outils |
| Tokens de raisonnement | Souvent facturés comme de la sortie |
| Outils payants | Recherche web, API externe |
| Nouveaux essais | Erreurs, replis, cascade |

:::pourquoi
Oublier une ligne, c'est sous-estimer le coût.
Les essais ratés coûtent autant que les réussis.
:::

Le chiffre qui compte : le **coût par tâche réussie**.

:::pourquoi
100 tâches, 50 €, 80 réussites : 0,625 € par réussite.
Un modèle moins cher qui échoue plus peut coûter plus par réussite.
:::

Le coût d'une étape **grandit** avec l'historique.

:::pourquoi
L'étape 20 renvoie les 19 précédentes.
Le cache de prompt et le résumé de l'historique freinent cette croissance.
:::

On **étiquette** chaque appel : tâche, client, fonctionnalité.

```mermaid
sequenceDiagram
  participant A as Agent
  participant M as Modèle
  participant T as Outil payant
  participant C as Compteur de coûts
  participant B as Tableau de bord
  A->>M: Appel 1 (tâche 812, client 42)
  M-->>A: 3 200 tokens entrée, 400 sortie
  A->>C: Ajoute le coût à la tâche 812
  A->>T: Recherche web
  A->>C: Ajoute le prix de l'outil
  A->>M: Appel 2 (préfixe en cache)
  A->>C: Ajoute le coût à la tâche 812
  C->>B: Coût par tâche, par client, par fonctionnalité
```

:::pourquoi
Sans étiquette, un coût n'a pas de propriétaire.
Ces mêmes compteurs alimentent les budgets et les quotas.
:::

## Latence p50 et p95 {#latence}

La **moyenne** cache la queue. On mesure des **percentiles**.

| Mesure | Sens |
|---|---|
| p50 | La moitié des requêtes est plus rapide |
| p95 | 19 requêtes sur 20 sont plus rapides |
| p99 | 99 sur 100 sont plus rapides |

:::pourquoi
99 requêtes à 1 seconde, une à 60 secondes : moyenne 1,6 seconde.
Un utilisateur sur cent a attendu une minute.
:::

La queue **domine** quand les étapes s'enchaînent.

:::pourquoi
Dean et Barroso (2013) : une requête interroge 100 serveurs, chacun lent une fois sur 100.
Environ 63 % des requêtes tombent sur au moins un serveur lent.
Un agent qui enchaîne les appels vit le même effet.
:::

Deux mesures pour deux usages :

- **Temps jusqu'au premier token** : l'attente ressentie dans un chat.
- **Temps total** : ce qui compte pour un agent qui doit finir avant d'agir.

:::pourquoi
Avec l'affichage progressif, l'utilisateur lit dès le premier mot.
Un agent, lui, n'agit qu'une fois la réponse complète.
:::

On mesure **par étape** et **de bout en bout**.

:::pourquoi
De bout en bout : ce que vit l'utilisateur.
Par étape : où passe le temps.
Un outil lent se cache souvent derrière un modèle accusé à tort.
:::

Pour réduire la queue :

- moins d'étapes en série ;
- des étapes **en parallèle** quand elles sont indépendantes ;
- le **cache** de prompt ;
- un **petit modèle** pour les étapes simples ;
- un **délai maximal** par appel, avec repli.

:::pourquoi
Chaque étape en série ajoute son attente et son risque d'être lente.
Le cache et le petit modèle raccourcissent chaque étape.
Le délai maximal coupe la queue la plus longue.
:::

## À retenir

- Mesurer le coût par tâche réussie, toutes lignes comprises.
- Étiqueter chaque appel : tâche, client, fonctionnalité.
- Latence en p50 et p95, pas en moyenne. Par étape et de bout en bout.
- Les étapes en série font grandir la queue.
