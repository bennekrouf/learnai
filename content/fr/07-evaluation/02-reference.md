---
id: "07-02"
titre: "Référence"
sous_rubriques:
  - id: golden-set
    titre: "Golden set"
  - id: dag
    titre: "DAG attendu"
  - id: note-etape
    titre: "Note par étape"
sources:
  - titre: "Anthropic — Adding Error Bars to Evals: A Statistical Approach to Language Model Evaluations, 2024"
    url: "https://arxiv.org/abs/2411.00640"
  - titre: "Yao et al. — τ-bench, 2024"
    url: "https://arxiv.org/abs/2406.12045"
  - titre: "Shankar et al. — Who Validates the Validators?, 2024"
    url: "https://arxiv.org/abs/2404.12272"
qcm:
  - id: golden-set-1
    sous_rubrique: golden-set
    question: "Qu'est-ce qu'un golden set ?"
    choix:
      - "Les meilleures réponses du modèle"
      - "Un jeu de cas de test fixes, avec pour chacun le résultat attendu, validé par un humain"
      - "Les données d'entraînement"
      - "Un jeu de prompts système"
    reponse: 1
    pourquoi: >-
      Chaque cas : une entrée, et ce qu'on attend.
      Un humain a vérifié l'attendu.
      On rejoue ces cas à chaque changement.
      C'est la règle graduée qui permet de comparer.

  - id: golden-set-2
    sous_rubrique: golden-set
    question: "D'où viennent les meilleurs cas d'un golden set ?"
    choix:
      - "Générés au hasard"
      - "Uniquement d'exemples inventés par le développeur"
      - "D'un benchmark public, tel quel"
      - "De vraies demandes et de vrais échecs observés, plus des cas limites écrits exprès"
    reponse: 3
    pourquoi: >-
      Les vraies demandes reflètent l'usage réel.
      Chaque bug corrigé devient un cas, pour qu'il ne revienne pas.
      Les cas limites testent ce qui arrive rarement mais coûte cher.
      Un benchmark public ne connaît pas ton métier.

  - id: golden-set-3
    sous_rubrique: golden-set
    question: "Un golden set de 20 cas. Le nouveau prompt passe de 15 à 16 réussites. Est-ce une amélioration ?"
    choix:
      - "Oui, clairement"
      - "On ne peut pas le conclure : l'écart est dans le bruit pour si peu de cas"
      - "Oui, de 5 %"
      - "Non, c'est forcément pire"
    reponse: 1
    pourquoi: >-
      Avec 20 cas, un seul cas qui bascule fait 5 points.
      Le hasard de génération suffit à l'expliquer.
      Anthropic recommande d'afficher une marge d'erreur avec chaque score.
      Plus de cas, ou plusieurs tirages par cas, réduisent ce bruit.

  - id: golden-set-4
    sous_rubrique: golden-set
    question: "Pourquoi comparer deux versions sur exactement les mêmes cas ?"
    choix:
      - "Les différences par cas (appariées) sont beaucoup moins bruitées que deux scores séparés"
      - "Pour gagner du temps uniquement"
      - "Parce que c'est obligatoire légalement"
      - "Ce n'est pas utile"
    reponse: 0
    pourquoi: >-
      Certains cas sont durs pour toutes les versions.
      Comparer cas par cas retire cette difficulté commune.
      Anthropic recommande ces tests appariés.
      On voit aussi exactement quels cas ont changé.

  - id: golden-set-5
    sous_rubrique: golden-set
    question: "Faut-il un jeu de test séparé de celui qui sert à mettre au point le prompt ?"
    choix:
      - "Non, c'est du gaspillage"
      - "Non, le même suffit"
      - "Oui : sinon on optimise pour ces cas précis et le score devient trompeur"
      - "Seulement pour les gros modèles"
    reponse: 2
    pourquoi: >-
      À force d'ajuster le prompt sur 50 cas, il les apprend.
      Le score monte, le comportement réel non.
      Un jeu mis de côté dit la vérité.
      On ne le regarde pas pendant la mise au point.

  - id: dag-1
    sous_rubrique: dag
    question: "Qu'est-ce qu'un DAG attendu pour évaluer un agent ?"
    choix:
      - "La liste exacte des appels, dans un ordre unique"
      - "Le graphe des étapes requises et de leurs dépendances, sans imposer un ordre total"
      - "Le prompt système"
      - "Un schéma de base de données"
    reponse: 1
    pourquoi: >-
      DAG : graphe orienté sans cycle.
      Il dit « vérifier l'identité avant de rembourser ».
      Il ne dit pas dans quel ordre lire deux documents indépendants.
      Plusieurs trajectoires valides passent le test.

  - id: dag-2
    sous_rubrique: dag
    question: "Pourquoi un DAG plutôt qu'une liste ordonnée d'appels attendus ?"
    choix:
      - "Aucune différence"
      - "Un DAG est plus court à écrire"
      - "Les listes sont interdites en YAML"
      - "Une liste rigide fait échouer des trajectoires correctes qui changent l'ordre d'étapes indépendantes"
    reponse: 3
    pourquoi: >-
      L'agent peut lire la facture puis le contrat, ou l'inverse.
      Les deux ordres sont bons.
      Une liste ordonnée en rejette un.
      Le DAG n'impose que les vraies dépendances.

  - id: dag-3
    sous_rubrique: dag
    question: "Que vérifier en comparant la trajectoire réelle au DAG attendu ?"
    choix:
      - "Que les arguments sont écrits avec les mêmes espaces"
      - "Que la trajectoire a exactement la même longueur"
      - "Les étapes obligatoires sont présentes, leurs dépendances respectées, et aucune action interdite n'apparaît"
      - "Seulement la dernière étape"
    reponse: 2
    pourquoi: >-
      Présence : rien d'essentiel n'est sauté.
      Ordre : chaque étape vient après ce dont elle dépend.
      Interdits : pas de suppression, pas d'envoi externe non prévu.
      Les étapes en plus sont permises, mais comptées.

  - id: dag-4
    sous_rubrique: dag
    question: "Comment comparer les arguments d'un appel attendu avec ceux de l'appel réel ?"
    choix:
      - "Sur les champs qui comptent, avec une règle par champ : égalité, ensemble, ou motif"
      - "Comparaison exacte du texte JSON complet"
      - "Ne jamais comparer les arguments"
      - "Demander au modèle s'ils se ressemblent"
    reponse: 0
    pourquoi: >-
      L'id de commande doit être exact.
      Une requête de recherche peut être formulée de dix façons.
      L'ordre des clés JSON n'a aucune importance.
      Une règle par champ évite les faux échecs.

  - id: note-etape-1
    sous_rubrique: note-etape
    question: "Pourquoi noter chaque étape et pas seulement le résultat final ?"
    choix:
      - "Parce que le résultat final ne compte pas"
      - "Pour avoir plus de chiffres dans le tableau"
      - "Pour savoir où l'agent échoue : choix d'outil, arguments, interprétation du résultat"
      - "Pour ralentir l'évaluation"
    reponse: 2
    pourquoi: >-
      « Échec » ne dit pas quoi corriger.
      « Bon outil, mauvais argument à l'étape 3 » le dit.
      On sait s'il faut revoir le schéma, le prompt ou l'outil.

  - id: note-etape-2
    sous_rubrique: note-etape
    question: "Quelle échelle pour noter une étape ?"
    choix:
      - "Une échelle simple et définie : réussi / raté, ou quelques niveaux décrits précisément"
      - "Une note de 0 à 100 sans définition"
      - "Un emoji"
      - "Un texte libre"
    reponse: 0
    pourquoi: >-
      Entre 73 et 76, personne ne sait la différence.
      Deux correcteurs ne donneront pas le même chiffre.
      Réussi ou raté, avec une définition, se reproduit.
      On peut ensuite compter et comparer.

  - id: note-etape-3
    sous_rubrique: note-etape
    question: "L'agent rate l'étape 2, puis se corrige à l'étape 3 et réussit. Comment le noter ?"
    choix:
      - "On retire ce cas du jeu de test"
      - "Échec total"
      - "Réussite totale, on oublie l'étape 2"
      - "Étape 2 ratée, rattrapage réussi, résultat réussi : chaque fait est noté séparément"
    reponse: 3
    pourquoi: >-
      Se corriger est une qualité.
      Mais l'erreur initiale coûte du temps et des tokens.
      Noter séparément garde les deux informations.
      Une moyenne les écraserait.

  - id: note-etape-4
    sous_rubrique: note-etape
    question: "Selon Shankar et al. (2024), que se passe-t-il quand on définit des critères d'évaluation ?"
    choix:
      - "Ils sont parfaits du premier coup"
      - "Les critères évoluent en notant des sorties réelles : c'est la dérive des critères"
      - "Ils deviennent inutiles"
      - "Le modèle les choisit seul"
    reponse: 1
    pourquoi: >-
      On a besoin de critères pour noter.
      Mais noter des sorties fait découvrir de nouveaux critères.
      Certains n'apparaissent qu'en voyant les réponses.
      La grille se révise après avoir lu de vrais résultats.
---

## Golden set {#golden-set}

Un **golden set** : des cas de test fixes, chacun avec son résultat attendu, vérifié par un humain.

```yaml
- id: remb-017
  entree: "Je veux être remboursé de la commande 512, l'article est arrivé cassé."
  attendu:
    etat_final: { commande_512: rembourse, montant: 49.90 }
    etapes_obligatoires: [verifier_identite, lire_commande, rembourser]
    interdits: [envoyer_mail_externe]
  origine: "ticket réel #8812, anonymisé"
```

:::pourquoi
Sans référence fixe, on ne peut pas comparer deux versions.
On compare des impressions.
:::

D'où viennent les cas :

- de **vraies demandes**, anonymisées ;
- de chaque **bug** corrigé ;
- de **cas limites** écrits exprès : sans réponse, ambigus, malveillants.

:::pourquoi
Un bug devenu test ne revient pas sans qu'on le voie.
Les cas limites coûtent cher quand ils ratent.
:::

Affiche une **marge d'erreur**. Compare les versions **cas par cas**.

:::pourquoi
Avec 20 cas, un cas qui bascule fait 5 points.
Anthropic recommande de donner l'erreur standard avec chaque score.
Et d'analyser les différences appariées : même cas, deux versions.
:::

Garde un jeu **mis de côté**, jamais utilisé pour régler le prompt.

:::pourquoi
À force d'ajuster sur les mêmes cas, le prompt les apprend.
Le score monte. Le comportement réel, non.
:::

## DAG attendu {#dag}

Pour un agent, l'attendu est souvent un **chemin**. Mais pas un chemin unique.

Un **DAG** (graphe orienté sans cycle) décrit les étapes requises et leurs dépendances.

```mermaid
flowchart LR
  I["verifier_identite"] --> R["rembourser"]
  C["lire_commande"] --> R
  P["lire_politique_retour"] --> R
  R --> N["notifier_client"]
```

`lire_commande` et `lire_politique_retour` peuvent venir dans n'importe quel ordre. `rembourser` vient après les trois.

:::pourquoi
Une liste ordonnée rejette des trajectoires correctes.
Le DAG n'impose que les vraies dépendances.
:::

On vérifie :

| Contrôle | Échoue si |
|---|---|
| Présence | Une étape obligatoire manque |
| Dépendances | Une étape arrive avant ce dont elle dépend |
| Interdits | Une action interdite apparaît |
| Arguments | Un champ clé diffère : id, montant |
| Extras | Permises, mais comptées dans le coût |

:::pourquoi
Chaque contrôle attrape une famille d'erreurs.
Ensemble, ils jugent le chemin sans le figer.
:::

Compare les arguments **champ par champ**, avec une règle par champ.

:::pourquoi
L'id doit être exact.
Une requête de recherche peut se formuler de dix façons.
:::

## Note par étape {#note-etape}

On note **chaque étape**, pas seulement la fin.

| Étape | Critère | Note |
|---|---|---|
| 1. verifier_identite | Bon outil, bons arguments | réussi |
| 2. lire_commande | Bon id | raté (id 215 au lieu de 512) |
| 3. lire_commande | Correction après erreur | réussi |
| 4. rembourser | Bon montant, après vérification | réussi |

:::pourquoi
« Échec » ne dit pas quoi corriger.
« Mauvais argument à l'étape 2 » le dit.
:::

Une échelle **simple et définie** : réussi ou raté, ou quelques niveaux décrits.

:::pourquoi
Une note de 0 à 100 sans définition ne se reproduit pas.
Deux correcteurs donneront deux chiffres.
:::

Les critères **évoluent**. Shankar et ses coauteurs (2024) appellent ça la dérive des critères.

:::pourquoi
Noter de vraies sorties fait découvrir de nouveaux critères.
On révise la grille après avoir lu des résultats, pas seulement avant.
:::

## À retenir

- Golden set : cas fixes, attendu vérifié, issus du réel, des bugs et des cas limites.
- Marge d'erreur, comparaison cas par cas, jeu mis de côté.
- DAG attendu : étapes obligatoires, dépendances, interdits. Pas d'ordre imposé inutilement.
- Note par étape, échelle simple, grille révisée en lisant les résultats.
