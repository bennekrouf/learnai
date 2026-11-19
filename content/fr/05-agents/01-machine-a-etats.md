---
id: "05-01"
titre: "Machine à états"
sous_rubriques:
  - id: etats
    titre: "États et transitions"
  - id: boucle
    titre: "La boucle d'agent"
sources:
  - titre: "Anthropic — Building effective agents, 19 décembre 2024"
    url: "https://www.anthropic.com/engineering/building-effective-agents"
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Anthropic — Project Vend, 27 juin 2025"
    url: "https://www.anthropic.com/research/project-vend-1"
qcm:
  - id: etats-1
    sous_rubrique: etats
    question: "Pourquoi décrire un agent comme une machine à états ?"
    choix:
      - "Parce que le modèle l'exige"
      - "Pour qu'il réponde plus vite"
      - "Pour économiser des tokens"
      - "On sait toujours où il en est, et quelles actions sont permises à cet instant"
    reponse: 3
    pourquoi: >-
      Un état explicite se lit, se sauvegarde, se reprend.
      Chaque état liste ses transitions autorisées.
      Le reste est refusé par le code.
      Sans état, il faut deviner où en est l'agent en relisant tout son historique.

  - id: etats-2
    sous_rubrique: etats
    question: "Un agent de remboursement est dans l'état « en attente de validation ». Le modèle demande « rembourser ». Que fait la machine ?"
    choix:
      - "Elle rembourse, le modèle a sûrement raison"
      - "Elle refuse : cette transition n'existe pas depuis cet état"
      - "Elle redémarre l'agent"
      - "Elle demande au modèle s'il est sûr"
    reponse: 1
    pourquoi: >-
      Depuis « en attente », seules « validé » et « rejeté » existent.
      Le modèle propose. La machine dispose.
      Une transition absente du graphe ne peut pas avoir lieu.
      C'est le code qui garde la porte, pas la bonne volonté du modèle.

  - id: etats-3
    sous_rubrique: etats
    question: "Qui fixe la liste des transitions autorisées ?"
    choix:
      - "Le code, écrit à l'avance. Le modèle choisit parmi les transitions permises"
      - "Le modèle, à chaque tour"
      - "L'utilisateur, en langage naturel"
      - "Personne, tout est permis"
    reponse: 0
    pourquoi: >-
      Le graphe est la règle du jeu.
      Le modèle joue dans ce cadre.
      On peut tester le graphe sans modèle.
      On ne peut pas tester un cadre que le modèle invente.

  - id: etats-4
    sous_rubrique: etats
    question: "Selon Anthropic (2024), qu'est-ce qui distingue un workflow d'un agent ?"
    choix:
      - "L'agent n'a pas d'outils"
      - "Le workflow utilise un plus petit modèle"
      - "Le workflow suit des chemins de code prédéfinis. L'agent dirige lui-même son processus"
      - "Aucune différence"
    reponse: 2
    pourquoi: >-
      Un workflow : le code décide de l'ordre des étapes.
      Un agent : le modèle décide de la prochaine étape et des outils à utiliser.
      Une machine à états se place entre les deux.
      Le modèle choisit, mais seulement parmi des chemins permis.

  - id: etats-5
    sous_rubrique: etats
    question: "Quels états faut-il toujours prévoir ?"
    choix:
      - "Uniquement « en cours »"
      - "Des états finaux : terminé, échoué, abandonné"
      - "Un état par outil"
      - "Aucun, l'agent s'arrête tout seul"
    reponse: 1
    pourquoi: >-
      Sans état final, rien ne dit que la tâche est finie.
      « Échoué » permet d'alerter et de reprendre.
      « Abandonné » trace une annulation.
      Chaque tâche doit finir dans l'un d'eux.

  - id: etats-6
    sous_rubrique: etats
    question: "Claudius vendait des articles sous leur prix d'achat. Où placer la règle « prix de vente ≥ coût » ?"
    choix:
      - "Nulle part, le modèle apprendra"
      - "Dans le prompt système, en majuscules"
      - "Dans un message Slack"
      - "Dans une garde du code, vérifiée avant la transition « changer le prix »"
    reponse: 3
    pourquoi: >-
      Une consigne dans le prompt peut être oubliée.
      Une garde dans le code s'applique à chaque fois.
      La transition est refusée, et le modèle reçoit la raison.
      Il peut alors proposer un autre prix.

  - id: boucle-1
    sous_rubrique: boucle
    question: "Dans une boucle d'agent, quand la boucle s'arrête-t-elle ?"
    choix:
      - "Après exactement un appel d'outil"
      - "Jamais"
      - "Quand le modèle donne une réponse finale, ou qu'une limite est atteinte"
      - "Quand le modèle dit « je suis fatigué »"
    reponse: 2
    pourquoi: >-
      La boucle : le modèle appelle un outil, lit le résultat, recommence.
      Elle s'arrête quand il répond sans appeler d'outil.
      Ou quand le code coupe : trop de tours, trop de coût, trop de temps.
      Il faut les deux conditions.

  - id: boucle-2
    sous_rubrique: boucle
    question: "Pourquoi fixer un nombre maximum de tours ?"
    choix:
      - "Pour éviter une boucle sans fin, et la facture qui va avec"
      - "Pour rendre le modèle plus intelligent"
      - "Parce que l'API l'impose"
      - "Pour réduire la latence de chaque appel"
    reponse: 0
    pourquoi: >-
      Un modèle peut tourner en rond.
      Chaque tour coûte des tokens et du temps.
      Anthropic recommande des conditions d'arrêt, comme un nombre maximum d'itérations.
      Atteindre la limite donne l'état « échoué », pas un silence.

  - id: boucle-3
    sous_rubrique: boucle
    question: "L'agent appelle cinq fois le même outil avec les mêmes arguments. Que faire ?"
    choix:
      - "Augmenter la température"
      - "Laisser continuer, il finira par réussir"
      - "Détecter la répétition dans le code, puis arrêter ou escalader"
      - "Effacer son historique sans rien dire"
    reponse: 2
    pourquoi: >-
      Le même appel donnera le même résultat.
      L'agent est coincé.
      Le code compare les derniers appels et repère la boucle.
      Il peut alors prévenir le modèle, ou passer la main à un humain.

  - id: boucle-4
    sous_rubrique: boucle
    question: "Un outil échoue. Que renvoyer au modèle ?"
    choix:
      - "Un message clair sur l'échec, pour qu'il puisse s'adapter"
      - "Rien, on cache l'erreur"
      - "Un résultat inventé pour ne pas le perturber"
      - "La pile d'appels complète"
    reponse: 0
    pourquoi: >-
      Anthropic l'a constaté dans son système de recherche.
      Dire à l'agent qu'un outil échoue lui permet de changer de stratégie.
      Un résultat inventé le pousse à construire sur du faux.
      La pile d'appels peut fuiter des détails internes.

  - id: boucle-5
    sous_rubrique: boucle
    question: "En plus du nombre de tours, quelles limites mettre à une boucle d'agent ?"
    choix:
      - "Une limite sur la taille du prompt système"
      - "Une limite de caractères par réponse uniquement"
      - "Aucune autre"
      - "Un budget de tokens ou de coût, et une durée maximale"
    reponse: 3
    pourquoi: >-
      Dix tours avec d'énormes résultats peuvent coûter très cher.
      Un outil lent peut faire durer un tour des heures.
      Trois compteurs : tours, coût, temps.
      Le premier atteint arrête la boucle.
---

## États et transitions {#etats}

Un agent en production a un **état**. Un nom précis, sauvegardé, lisible.

```mermaid
stateDiagram-v2
  [*] --> Recu
  Recu --> Analyse
  Analyse --> EnAttenteValidation : montant > seuil
  Analyse --> Rembourse : montant ≤ seuil
  EnAttenteValidation --> Rembourse : validé
  EnAttenteValidation --> Refuse : rejeté
  Analyse --> Echoue : erreur
  Rembourse --> [*]
  Refuse --> [*]
  Echoue --> [*]
```

Chaque état liste ses **transitions** permises. Le reste est refusé.

:::pourquoi
On sait toujours où en est l'agent.
On sait ce qu'il a le droit de faire maintenant.
Une action hors du graphe ne peut pas arriver.
:::

Le **code** fixe le graphe. Le **modèle** choisit parmi les transitions permises.

:::pourquoi
Le modèle propose. La machine dispose.
Le graphe se teste sans modèle.
:::

Anthropic distingue deux formes. Le **workflow** suit des chemins de code prédéfinis. L'**agent** dirige lui-même son processus et ses outils.

:::pourquoi
La machine à états prend le meilleur des deux.
Le modèle garde l'initiative.
Le code garde les limites.
:::

Les règles métier vont dans des **gardes** : des conditions vérifiées avant une transition.

:::pourquoi
Claudius vendait sous le prix d'achat.
Une garde « prix ≥ coût » aurait refusé ce changement.
Une consigne dans le prompt, elle, peut être oubliée.
:::

Prévois toujours des **états finaux** : terminé, échoué, abandonné.

:::pourquoi
Sans état final, impossible de savoir si une tâche est finie.
« Échoué » déclenche une alerte ou une reprise.
:::

## La boucle d'agent {#boucle}

Le cœur d'un agent est une boucle.

```mermaid
sequenceDiagram
  participant O as Orchestrateur (code)
  participant M as Modèle
  participant T as Outil
  loop Tant qu'aucune limite n'est atteinte
    O->>M: Contexte + résultats précédents
    M-->>O: Appel d'outil demandé
    O->>O: Vérifie l'état et les gardes
    O->>T: Exécute l'appel
    T-->>O: Résultat ou erreur
  end
  M-->>O: Réponse finale
  O->>O: État final : terminé
```

Elle s'arrête dans deux cas : le modèle répond sans appeler d'outil, ou une **limite** est atteinte.

:::pourquoi
Le modèle peut tourner en rond.
Anthropic recommande des conditions d'arrêt, comme un nombre maximum d'itérations.
:::

Trois limites :

| Limite | Évite |
|---|---|
| Nombre de tours | La boucle sans fin |
| Budget de tokens ou de coût | La facture surprise |
| Durée maximale | La tâche bloquée pendant des heures |

:::pourquoi
Chaque limite couvre un risque différent.
La première atteinte arrête la boucle, avec l'état « échoué ».
:::

Détecte les répétitions : même outil, mêmes arguments, plusieurs fois.

:::pourquoi
Le même appel donne le même résultat.
L'agent est coincé. Mieux vaut le dire que payer dix tours de plus.
:::

Quand un outil échoue, **dis-le** au modèle. Clairement.

:::pourquoi
Anthropic l'a observé : l'agent qui sait qu'un outil échoue change de stratégie.
Celui à qui on cache l'erreur construit sur du vide.
:::

## À retenir

- Un état explicite, des transitions listées, des états finaux.
- Le code fixe le graphe et les gardes. Le modèle choisit dans ce cadre.
- La boucle s'arrête sur une réponse finale ou sur une limite : tours, coût, temps.
- Les échecs d'outil sont dits au modèle, pas cachés.
