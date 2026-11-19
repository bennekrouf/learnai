---
id: "08-01"
titre: "Cache"
sous_rubriques:
  - id: cache-prompt
    titre: "Cache de prompt"
  - id: cache-semantique
    titre: "Cache sémantique et ses risques"
sources:
  - titre: "Anthropic — Prompt caching (documentation)"
    url: "https://platform.claude.com/docs/en/build-with-claude/prompt-caching"
  - titre: "OpenAI — Prompt caching (documentation)"
    url: "https://developers.openai.com/api/docs/guides/prompt-caching"
  - titre: "Gu et al. — Auditing Prompt Caching in Language Model APIs, ICML 2025"
    url: "https://arxiv.org/abs/2502.07776"
qcm:
  - id: cache-prompt-1
    sous_rubrique: cache-prompt
    question: "Que réutilise un cache de prompt ?"
    choix:
      - "Les poids du modèle"
      - "La réponse finale d'une question déjà posée"
      - "Les fichiers de l'utilisateur"
      - "Le calcul déjà fait sur un début de prompt identique"
    reponse: 3
    pourquoi: >-
      Le fournisseur garde le calcul fait sur le début du prompt.
      Si la requête suivante commence pareil, il le reprend.
      La réponse, elle, est générée à nouveau.
      On paie moins et on attend moins.

  - id: cache-prompt-2
    sous_rubrique: cache-prompt
    question: "Le cache de prompt fonctionne par préfixe exact. Où placer ce qui change à chaque requête ?"
    choix:
      - "Au tout début du prompt"
      - "À la fin du prompt, après tout ce qui est stable"
      - "Au milieu, entre les outils et les instructions"
      - "Peu importe l'ordre"
    reponse: 1
    pourquoi: >-
      Le cache s'arrête au premier caractère différent.
      Une date en tête du prompt casse tout le reste.
      Stable d'abord : outils, instructions, documents.
      Variable ensuite : la question de l'utilisateur.

  - id: cache-prompt-3
    sous_rubrique: cache-prompt
    question: "Ton prompt système commence par « Nous sommes le 19 novembre 2026, 14:32:07 ». Effet sur le cache ?"
    choix:
      - "Le préfixe change à chaque seconde : le cache ne sert presque jamais"
      - "Aucun effet"
      - "Le cache devient plus rapide"
      - "Le modèle ignore cette ligne"
    reponse: 0
    pourquoi: >-
      Chaque requête a un début différent.
      Aucun préfixe ne se répète.
      On paie l'écriture du cache sans jamais le relire.
      Mettre l'heure à la fin, ou à la journée près.

  - id: cache-prompt-4
    sous_rubrique: cache-prompt
    question: "Chez Anthropic, écrire dans le cache coûte 1,25 fois le prix normal, et le relire 0,1 fois pour la plupart des modèles. Un même préfixe est utilisé 2 fois. Ça vaut le coup ?"
    choix:
      - "Seulement à partir de 100 utilisations"
      - "Non, l'écriture coûte trop cher"
      - "Oui : 1,25 + 0,1 = 1,35, contre 2 sans cache"
      - "Impossible à dire"
    reponse: 2
    pourquoi: >-
      Sans cache : deux fois le prix plein.
      Avec cache : une écriture un peu plus chère, puis une lecture presque gratuite.
      Le gain commence dès la deuxième utilisation.
      Il grandit avec chaque réutilisation.

  - id: cache-prompt-5
    sous_rubrique: cache-prompt
    question: "Le cache par défaut d'Anthropic dure 5 minutes. Que se passe-t-il s'il est relu avant la fin ?"
    choix:
      - "Il expire quand même au bout de 5 minutes"
      - "Sa durée repart à zéro, sans coût en plus"
      - "Il est facturé deux fois"
      - "Il devient permanent"
    reponse: 1
    pourquoi: >-
      Chaque lecture rafraîchit le cache.
      Un trafic régulier le garde chaud.
      Pour des requêtes espacées, une durée d'une heure existe.
      Elle coûte plus cher à l'écriture.

  - id: cache-prompt-6
    sous_rubrique: cache-prompt
    question: "Pourquoi une étude de 2025 met-elle en garde contre un cache de prompt partagé entre utilisateurs ?"
    choix:
      - "Le cache coûte plus cher"
      - "Le cache efface les prompts"
      - "Le cache change la réponse du modèle"
      - "Un temps de réponse plus court révèle qu'un autre a déjà envoyé le même début de prompt"
    reponse: 3
    pourquoi: >-
      Un préfixe en cache se traite plus vite.
      Un attaquant peut tester un début de prompt et mesurer le temps.
      Gu et al. ont trouvé sept fournisseurs avec un cache partagé entre utilisateurs.
      Un gain de performance peut devenir une fuite.

  - id: cache-semantique-1
    sous_rubrique: cache-semantique
    question: "Comment fonctionne un cache sémantique ?"
    choix:
      - "Il réutilise le calcul du préfixe du prompt"
      - "Il ne garde que les questions identiques au caractère près"
      - "Il renvoie une réponse déjà produite si la nouvelle question est assez proche en sens d'une ancienne"
      - "Il compresse les réponses"
    reponse: 2
    pourquoi: >-
      La question devient un vecteur.
      On cherche une question proche déjà posée.
      Au-delà d'un seuil de similarité, on renvoie sa réponse.
      Le modèle n'est pas appelé du tout.

  - id: cache-semantique-2
    sous_rubrique: cache-semantique
    question: "« Comment activer la double authentification ? » est en cache. Arrive « Comment désactiver la double authentification ? ». Risque ?"
    choix:
      - "Les deux phrases sont très proches : le cache peut renvoyer la mauvaise réponse"
      - "Aucun, le cache voit la différence"
      - "Le modèle corrige tout seul"
      - "La question est rejetée"
    reponse: 0
    pourquoi: >-
      Un seul mot change. Le sens est inverse.
      Les vecteurs restent très proches.
      C'est un faux positif : une réponse juste pour une autre question.
      Un vecteur proche ne veut pas dire un sens identique.

  - id: cache-semantique-3
    sous_rubrique: cache-semantique
    question: "Un utilisateur demande « Quel est mon solde ? ». La réponse est mise en cache sémantique global. Problème ?"
    choix:
      - "Le solde sera arrondi"
      - "Aucun, c'est la même question"
      - "Un autre utilisateur qui pose la même question reçoit le solde du premier"
      - "La réponse sera plus lente"
    reponse: 2
    pourquoi: >-
      La question est la même. La bonne réponse dépend de qui demande.
      Un cache global mélange les utilisateurs.
      C'est une fuite de données personnelles.
      Ce qui dépend de l'utilisateur ne va pas dans un cache partagé.

  - id: cache-semantique-4
    sous_rubrique: cache-semantique
    question: "Quelles réponses se prêtent le mieux au cache sémantique ?"
    choix:
      - "Les réponses générales, stables, sans données personnelles, ni action"
      - "Les réponses qui dépendent de l'utilisateur"
      - "Les réponses qui déclenchent un remboursement"
      - "Les réponses sur des données qui changent chaque minute"
    reponse: 0
    pourquoi: >-
      « Quels sont vos horaires ? » : même réponse pour tous.
      « Rembourse ma commande » : une action, jamais rejouée depuis un cache.
      « Prix actuel » : périmé dans une minute.
      Le cache sert les questions fréquentes et figées.

  - id: cache-semantique-5
    sous_rubrique: cache-semantique
    question: "Comment régler le seuil de similarité d'un cache sémantique ?"
    choix:
      - "Une fois pour toutes, sans jamais le revoir"
      - "Au plus bas pour maximiser les réutilisations"
      - "Au hasard"
      - "Sur un jeu de paires de questions étiquetées, en mesurant les faux positifs"
    reponse: 3
    pourquoi: >-
      Seuil bas : beaucoup de réutilisation, beaucoup d'erreurs.
      Seuil haut : peu d'erreurs, peu d'économies.
      Seules des paires réelles disent où placer la barre.
      C'est une évaluation, comme à l'étape 7.

  - id: cache-semantique-6
    sous_rubrique: cache-semantique
    question: "Comment limiter les risques d'un cache sémantique ?"
    choix:
      - "Un cache unique, éternel, pour tout le monde"
      - "Cache par utilisateur ou par client, durée de vie courte, et un moyen de vider une entrée fausse"
      - "Aucune règle, le modèle s'en charge"
      - "Désactiver les journaux"
    reponse: 1
    pourquoi: >-
      Le cloisonnement évite les fuites entre utilisateurs.
      La durée de vie évite les réponses périmées.
      La purge corrige une erreur sans attendre.
      Une mauvaise réponse en cache se répète à chaque visite.
---

## Cache de prompt {#cache-prompt}

Beaucoup de requêtes commencent pareil : mêmes outils, mêmes instructions, mêmes documents.

Le **cache de prompt** garde le calcul fait sur ce début. La requête suivante le reprend.

:::pourquoi
Le début du prompt est souvent le plus long.
Le recalculer à chaque fois coûte du temps et de l'argent.
La réponse, elle, reste générée à neuf.
:::

```mermaid
sequenceDiagram
  participant A as Application
  participant F as API du fournisseur
  participant C as Cache
  participant M as Modèle
  A->>F: Outils + instructions + question 1
  F->>C: Préfixe connu ?
  C-->>F: Non
  F->>M: Calcul complet
  F->>C: Écrit le préfixe (1,25 x)
  A->>F: Mêmes outils + instructions + question 2
  F->>C: Préfixe connu ?
  C-->>F: Oui (0,1 x)
  F->>M: Calcul de la question 2 seulement
```

Le cache marche par **préfixe exact**. Le stable d'abord, le variable à la fin.

:::pourquoi
Le cache s'arrête au premier caractère différent.
Chez Anthropic, l'ordre est : outils, puis système, puis messages.
Changer un outil invalide tout ce qui suit.
:::

| Place | Contenu |
|---|---|
| Début | Outils, instructions, documents de référence |
| Milieu | Historique de la conversation |
| Fin | Question du moment, date, heure |

Les prix chez Anthropic, pour la plupart des modèles :

| Opération | Prix par rapport au prix normal |
|---|---|
| Écriture, cache de 5 minutes | 1,25 x |
| Écriture, cache d'une heure | 2 x |
| Lecture | 0,1 x |

:::pourquoi
Deux appels sans cache : 2.
Avec cache : 1,25 + 0,1 = 1,35.
Le gain commence dès la deuxième utilisation.
:::

Chaque lecture **rafraîchit** le cache de 5 minutes. Les prompts trop courts ne sont pas mis en cache.

:::pourquoi
Un trafic régulier garde le cache chaud.
Sous une longueur minimale, le gain ne vaut pas la gestion.
La longueur minimale dépend du modèle.
:::

Chez OpenAI, le cache est **automatique** au-delà d'une longueur minimale. La remise dépend du modèle.

:::pourquoi
Rien à activer. Mais la même règle s'applique :
un préfixe stable, ou pas de remise.
Le champ `cached_tokens` de la réponse dit ce qui a été relu.
:::

Un cache partagé entre utilisateurs peut **fuir** par le temps de réponse.

:::pourquoi
Un préfixe en cache répond plus vite.
Un attaquant peut tester un début de prompt et chronométrer.
En 2025, Gu et al. ont détecté un cache partagé entre utilisateurs chez sept fournisseurs.
:::

## Cache sémantique et ses risques {#cache-semantique}

Le **cache sémantique** va plus loin. Il garde des réponses entières.

Si une question est assez proche d'une ancienne, il renvoie l'ancienne réponse. Sans appeler le modèle.

```mermaid
sequenceDiagram
  participant U as Utilisateur
  participant A as Application
  participant S as Cache sémantique
  participant M as Modèle
  U->>A: « Quels sont vos horaires ? »
  A->>S: Question proche déjà vue ? (similarité)
  alt Similarité au-dessus du seuil
    S-->>A: Réponse en cache
  else Sinon
    A->>M: Question
    M-->>A: Réponse
    A->>S: Stocke question + réponse
  end
  A-->>U: Réponse
```

:::pourquoi
Pas d'appel au modèle : coût quasi nul, réponse immédiate.
Utile pour les questions fréquentes, comme une FAQ.
:::

Premier risque : le **faux positif**. Une question proche n'est pas la même question.

| Question en cache | Nouvelle question | Problème |
|---|---|---|
| Comment activer la 2FA ? | Comment désactiver la 2FA ? | Sens inverse |
| Statut de la commande 512 ? | Statut de la commande 513 ? | Autre commande |
| Prix en 2025 ? | Prix en 2026 ? | Autre année |

:::pourquoi
Les vecteurs de ces phrases sont très proches.
Un seul mot change le sens.
Le cache ne lit pas : il mesure une distance.
:::

Deuxième risque : la **fuite**. « Quel est mon solde ? » n'a pas la même réponse pour tout le monde.

:::pourquoi
Un cache global renvoie la réponse du premier utilisateur au suivant.
C'est une fuite de données personnelles.
:::

Troisième risque : la réponse **périmée**. Un prix, un stock ou un statut change.

:::pourquoi
La réponse en cache était juste hier.
Elle est fausse aujourd'hui, et le cache ne le sait pas.
:::

Les règles :

- Mettre en cache seulement le **général et stable** : pas de données personnelles, pas d'action.
- **Cloisonner** par utilisateur ou par client.
- Fixer une **durée de vie** et pouvoir **purger** une entrée.
- Régler le **seuil** sur des paires de questions étiquetées.

:::pourquoi
Une mauvaise réponse en cache se répète à chaque visite.
Le seuil se règle comme une évaluation : en comptant les erreurs.
:::

## À retenir

- Cache de prompt : stable d'abord, variable à la fin. Rentable dès la deuxième utilisation.
- Pas d'horodatage en tête du prompt.
- Cache sémantique : rapide et gratuit, mais faux positifs, fuites et réponses périmées.
- Cloisonner, limiter dans le temps, régler le seuil sur des données réelles.
