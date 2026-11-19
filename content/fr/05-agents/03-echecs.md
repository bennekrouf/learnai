---
id: "05-03"
titre: "Échecs"
sous_rubriques:
  - id: timeouts
    titre: "Timeouts"
  - id: backoff
    titre: "Backoff exponentiel et aléa"
  - id: limites
    titre: "Limiter les tentatives"
sources:
  - titre: "Marc Brooker — Exponential Backoff And Jitter, AWS Architecture Blog, mars 2015"
    url: "https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/"
  - titre: "AWS Builders' Library — Timeouts, retries, and backoff with jitter"
    url: "https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/"
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
qcm:
  - id: timeouts-1
    sous_rubrique: timeouts
    question: "Pourquoi mettre un timeout sur chaque appel d'outil ou de modèle ?"
    choix:
      - "Parce que JSON l'exige"
      - "Pour accélérer le réseau"
      - "Pour réduire le prix des tokens"
      - "Sans timeout, un appel bloqué bloque l'agent pour toujours"
    reponse: 3
    pourquoi: >-
      Un serveur peut ne jamais répondre.
      Sans limite, l'agent attend indéfiniment.
      Le timeout transforme ce silence en erreur.
      Une erreur, on sait la gérer.

  - id: timeouts-2
    sous_rubrique: timeouts
    question: "Comment choisir la valeur d'un timeout ?"
    choix:
      - "Toujours 1 seconde"
      - "À partir de la latence mesurée : un percentile élevé, avec une marge"
      - "Toujours 10 minutes"
      - "Au hasard"
    reponse: 1
    pourquoi: >-
      Trop court : on coupe des appels qui allaient réussir.
      Trop long : on attend pour rien.
      AWS recommande de partir de la latence observée, sur un percentile élevé.
      On accepte de couper une petite fraction d'appels très lents.

  - id: timeouts-3
    sous_rubrique: timeouts
    question: "Chaque appel a son timeout. Faut-il aussi un timeout pour toute la tâche ?"
    choix:
      - "Oui : des appels courts mais très nombreux peuvent durer des heures"
      - "Non, c'est redondant"
      - "Seulement le week-end"
      - "Seulement pour les petits modèles"
    reponse: 0
    pourquoi: >-
      Cent appels de 30 secondes font 50 minutes.
      Chacun respecte son timeout.
      La tâche entière, elle, dépasse tout délai raisonnable.
      Un délai global protège l'utilisateur qui attend.

  - id: timeouts-4
    sous_rubrique: timeouts
    question: "Un appel de paiement dépasse son timeout. Le paiement a-t-il échoué ?"
    choix:
      - "Non, jamais"
      - "Oui, forcément"
      - "On ne sait pas : il a pu réussir côté serveur"
      - "Oui, et il est remboursé automatiquement"
    reponse: 2
    pourquoi: >-
      Le timeout dit seulement : pas de réponse à temps.
      Le serveur a pu traiter la demande, puis répondre trop tard.
      Réessayer sans précaution peut payer deux fois.
      D'où la clé d'idempotence.

  - id: backoff-1
    sous_rubrique: backoff
    question: "Qu'est-ce que le backoff exponentiel ?"
    choix:
      - "Réessayer immédiatement, en boucle"
      - "Attendre de plus en plus longtemps entre deux tentatives : 1 s, 2 s, 4 s, 8 s…"
      - "Attendre toujours 5 secondes"
      - "Réduire la taille du prompt à chaque essai"
    reponse: 1
    pourquoi: >-
      Un service surchargé a besoin de temps pour récupérer.
      Réessayer tout de suite aggrave la surcharge.
      Doubler l'attente laisse de plus en plus d'air.
      Et limite le nombre d'appels inutiles.

  - id: backoff-2
    sous_rubrique: backoff
    question: "Pourquoi ajouter de l'aléa (jitter) au backoff ?"
    choix:
      - "Pour tromper le serveur"
      - "Pour rendre les tests imprévisibles"
      - "Pour économiser des tokens"
      - "Pour que tous les clients ne réessaient pas au même instant"
    reponse: 3
    pourquoi: >-
      Mille clients échouent ensemble.
      Sans aléa, ils réessaient tous après 1 s, puis 2 s, puis 4 s.
      Chaque vague retape le serveur en même temps.
      L'aléa étale les tentatives dans le temps.

  - id: backoff-3
    sous_rubrique: backoff
    question: "Le « full jitter » décrit par Marc Brooker (AWS, 2015), c'est…"
    choix:
      - "Une attente fixe de 0 seconde"
      - "Le délai exponentiel plus une seconde fixe"
      - "Une attente tirée au hasard entre 0 et le délai exponentiel plafonné"
      - "Un délai qui diminue à chaque essai"
    reponse: 2
    pourquoi: >-
      On calcule le délai : base × 2 puissance n, plafonné.
      On tire une attente au hasard entre 0 et ce délai.
      Dans ses simulations, la version sans aléa est clairement la pire.
      Ce principe reste un pilier chez Amazon.

  - id: backoff-4
    sous_rubrique: backoff
    question: "Pourquoi plafonner le délai exponentiel ?"
    choix:
      - "Sans plafond, la 15ᵉ tentative attendrait des heures"
      - "Pour respecter JSON-RPC"
      - "Pour que le serveur ne s'ennuie pas"
      - "Aucune raison"
    reponse: 0
    pourquoi: >-
      1 s doublée 15 fois fait plus de 9 heures.
      Le plafond garde une attente raisonnable.
      Par exemple 30 ou 60 secondes au maximum.

  - id: backoff-5
    sous_rubrique: backoff
    question: "L'API répond 429 avec un en-tête Retry-After: 20. Que faire ?"
    choix:
      - "Abandonner définitivement"
      - "Réessayer tout de suite"
      - "Attendre au moins 20 secondes avant de réessayer"
      - "Changer de clé d'API pour contourner la limite"
    reponse: 2
    pourquoi: >-
      429 veut dire : trop de requêtes.
      Retry-After dit quand revenir.
      Le serveur sait mieux que toi quand il sera prêt.
      Contourner la limite viole les conditions du service.

  - id: limites-1
    sous_rubrique: limites
    question: "Quelle erreur vaut la peine d'être réessayée ?"
    choix:
      - "Un 503 ou un timeout : souvent passagers"
      - "Un 400 : requête mal formée"
      - "Un 401 : non authentifié"
      - "Un 403 : interdit"
    reponse: 0
    pourquoi: >-
      Une erreur passagère peut disparaître au prochain essai.
      Une requête mal formée restera mal formée.
      Un accès refusé restera refusé.
      Réessayer ces erreurs-là ne fait que perdre du temps.

  - id: limites-2
    sous_rubrique: limites
    question: "Combien de fois réessayer ?"
    choix:
      - "Autant que le modèle le demande"
      - "À l'infini"
      - "Une seule fois, jamais plus"
      - "Un nombre fixé, petit. Ensuite, échec explicite et escalade"
    reponse: 3
    pourquoi: >-
      Au-delà de quelques essais, la panne n'est plus passagère.
      Continuer charge le service et retarde la réaction.
      L'état passe à « échoué ».
      Un humain ou une alerte prend le relais.

  - id: limites-3
    sous_rubrique: limites
    question: "L'agent réessaie 3 fois. L'outil réessaie 3 fois. L'API derrière réessaie 3 fois. Combien d'appels au pire ?"
    choix:
      - "9"
      - "27"
      - "3"
      - "6"
    reponse: 1
    pourquoi: >-
      Les tentatives se multiplient d'une couche à l'autre.
      3 × 3 × 3 = 27 appels pour une seule demande.
      En pleine panne, c'est ce qui achève le service.
      On réessaie à une seule couche, de préférence.

  - id: limites-4
    sous_rubrique: limites
    question: "L'outil renvoie « date invalide : format AAAA-MM-JJ attendu ». Réessayer le même appel ?"
    choix:
      - "Non, et arrêter l'agent sans rien dire"
      - "Oui, cinq fois avec backoff"
      - "Oui, immédiatement"
      - "Non : renvoyer l'erreur au modèle pour qu'il corrige l'argument"
    reponse: 3
    pourquoi: >-
      Le même argument donnera la même erreur.
      Le backoff ne corrige pas une date mal écrite.
      Le modèle, lui, peut la corriger.
      Erreur passagère : réessayer. Erreur de contenu : renvoyer au modèle.
---

## Timeouts {#timeouts}

Tout appel peut rester sans réponse. Chaque appel a un **timeout**.

:::pourquoi
Sans limite, l'agent attend pour toujours.
Le timeout transforme le silence en erreur gérable.
:::

Choisis la valeur à partir de la **latence mesurée**. Un percentile élevé, avec une marge.

:::pourquoi
Trop court, on coupe des appels qui allaient réussir.
Trop long, on attend pour rien.
:::

Ajoute un **délai global** pour toute la tâche.

:::pourquoi
Cent appels de 30 secondes respectent chacun leur timeout.
Ensemble, ils font 50 minutes.
:::

Un timeout ne dit pas que l'opération a échoué.

:::pourquoi
Le serveur a pu la traiter, puis répondre trop tard.
Avant de réessayer une action, il faut l'idempotence.
:::

## Backoff exponentiel et aléa {#backoff}

On ne réessaie pas tout de suite. On attend, de plus en plus : c'est le **backoff exponentiel**.

On ajoute de l'**aléa** : chaque client tire son attente au hasard.

```mermaid
sequenceDiagram
  participant C1 as Agent 1
  participant C2 as Agent 2
  participant C3 as Agent 3
  participant S as API surchargée
  C1->>S: Appel
  C2->>S: Appel
  C3->>S: Appel
  S-->>C1: 503
  S-->>C2: 503
  S-->>C3: 503
  Note over C1,C3: Sans aléa : les trois reviennent ensemble, à 1 s, 2 s, 4 s
  C2->>S: Réessai après 0,3 s
  C1->>S: Réessai après 0,8 s
  C3->>S: Réessai après 1,7 s
  Note over C1,S: Avec aléa : les tentatives s'étalent
```

:::pourquoi
Mille clients qui réessaient au même instant recréent la surcharge.
Marc Brooker (AWS, 2015) l'a simulé : sans aléa, c'est clairement la pire option.
:::

La formule « full jitter » :

```text
attente = aleatoire(0, min(plafond, base × 2^tentative))
```

:::pourquoi
L'exponentielle laisse de plus en plus d'air au service.
Le plafond évite d'attendre des heures.
Le hasard étale les clients.
:::

Le serveur dit quand revenir ? Écoute-le. Un `429` avec `Retry-After: 20` veut dire : pas avant 20 secondes.

:::pourquoi
Le serveur connaît sa charge mieux que toi.
:::

## Limiter les tentatives {#limites}

Toutes les erreurs ne se réessaient pas.

| Erreur | Réessayer ? | Pourquoi |
|---|---|---|
| Timeout, 503, 429 | Oui, avec backoff | Souvent passagère |
| 400, argument invalide | Non, renvoyer au modèle | Elle se reproduira |
| 401, 403 | Non, escalader | Un droit manque |

:::pourquoi
Le backoff ne corrige pas une date mal écrite.
Le modèle, lui, peut la corriger si on lui dit pourquoi.
:::

Fixe un **nombre maximum** de tentatives. Ensuite : état « échoué », alerte, humain.

:::pourquoi
Après quelques essais, la panne n'est plus passagère.
Insister charge le service et retarde la réaction.
:::

Réessaie à **une seule couche**.

```mermaid
flowchart LR
  A["Agent : 3 essais"] --> O["Outil : 3 essais chacun"]
  O --> P["API : 3 essais chacun"]
  P --> R["Au pire 27 appels pour une demande"]
```

:::pourquoi
Les tentatives se multiplient d'une couche à l'autre.
En pleine panne, ce sont elles qui achèvent le service.
:::

## À retenir

- Un timeout par appel, plus un délai global. Un timeout n'est pas un échec certain.
- Backoff exponentiel, plafonné, avec aléa. Respecter `Retry-After`.
- Réessayer seulement les erreurs passagères. Les erreurs de contenu vont au modèle.
- Peu de tentatives, à une seule couche. Puis échec explicite.
