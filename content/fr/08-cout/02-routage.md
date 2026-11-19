---
id: "08-02"
titre: "Routage"
sous_rubriques:
  - id: selon-complexite
    titre: "Selon la complexité"
  - id: repli
    titre: "Chaînes de repli"
sources:
  - titre: "Ong et al. — RouteLLM: Learning to Route LLMs with Preference Data, 2024"
    url: "https://arxiv.org/abs/2406.18665"
  - titre: "LMSYS — RouteLLM: An Open-Source Framework for Cost-Effective LLM Routing, juillet 2024"
    url: "https://lmsys.org/blog/2024-07-01-routellm/"
  - titre: "Chen, Zaharia, Zou — FrugalGPT, 2023"
    url: "https://arxiv.org/abs/2305.05176"
qcm:
  - id: selon-complexite-1
    sous_rubrique: selon-complexite
    question: "Qu'est-ce que le routage de modèles ?"
    choix:
      - "Envoyer chaque requête à tous les modèles"
      - "Toujours utiliser le plus gros modèle"
      - "Répartir les requêtes au hasard"
      - "Envoyer chaque requête au modèle le moins cher capable de bien la traiter"
    reponse: 3
    pourquoi: >-
      Beaucoup de requêtes sont simples.
      Un petit modèle les traite bien, pour bien moins cher.
      Le gros modèle est gardé pour ce qui le demande.
      On paie la difficulté, pas le volume.

  - id: selon-complexite-2
    sous_rubrique: selon-complexite
    question: "Quelle requête peut aller à un petit modèle ?"
    choix:
      - "Planifier une migration de base de données en dix étapes"
      - "Classer un e-mail dans une de cinq catégories"
      - "Analyser un contrat de 80 pages"
      - "Corriger un bug dans plusieurs fichiers"
    reponse: 1
    pourquoi: >-
      Classer, extraire, reformuler : tâches courtes et bien cadrées.
      Planifier, raisonner longtemps, coder sur plusieurs fichiers : tâches dures.
      La complexité dépend de la tâche, pas de la longueur de la question.

  - id: selon-complexite-3
    sous_rubrique: selon-complexite
    question: "Que montre RouteLLM (2024) sur MT Bench ?"
    choix:
      - "Un routeur garde 95 % de la performance de GPT-4 en n'envoyant que 26 % des requêtes à GPT-4"
      - "Les petits modèles battent toujours GPT-4"
      - "Le routage dégrade toujours la qualité de moitié"
      - "Le routage ne fait rien gagner"
    reponse: 0
    pourquoi: >-
      Le routeur apprend sur des préférences humaines.
      Il prédit quand le petit modèle suffit.
      Les trois quarts des requêtes partent vers le modèle bon marché.
      La qualité reste proche du gros modèle.

  - id: selon-complexite-4
    sous_rubrique: selon-complexite
    question: "Qu'est-ce qu'une cascade, comme dans FrugalGPT ?"
    choix:
      - "Demander au gros modèle de choisir le petit"
      - "Appeler tous les modèles en même temps"
      - "Essayer d'abord un petit modèle, vérifier la réponse, et monter vers un plus gros si elle ne convient pas"
      - "Couper la réponse en morceaux"
    reponse: 2
    pourquoi: >-
      Le routeur décide avant. La cascade décide après.
      Un vérificateur note la réponse du petit modèle.
      Si elle est faible, on paie le gros.
      FrugalGPT annonce jusqu'à 98 % d'économies au niveau de GPT-4 sur ses tests.

  - id: selon-complexite-5
    sous_rubrique: selon-complexite
    question: "Quel est le coût caché d'une cascade ?"
    choix:
      - "Aucun coût caché"
      - "Quand on monte d'un niveau, on paie le petit modèle en plus du gros, et on attend plus longtemps"
      - "Elle désactive le cache"
      - "Elle oblige à changer de fournisseur"
    reponse: 1
    pourquoi: >-
      Une requête difficile passe par deux modèles.
      Le coût et la latence s'additionnent.
      Si la plupart des requêtes montent, la cascade coûte plus cher que le gros modèle seul.
      On mesure le taux de montée.

  - id: selon-complexite-6
    sous_rubrique: selon-complexite
    question: "Comment savoir si ton routage est bon ?"
    choix:
      - "Demander au petit modèle s'il a bien répondu"
      - "Regarder seulement la facture"
      - "Faire confiance au routeur"
      - "Évaluer chaque branche sur le jeu de test, et suivre la qualité et le coût par branche"
    reponse: 3
    pourquoi: >-
      Une économie qui casse des réponses n'en est pas une.
      Le jeu de test de l'étape 7 dit si le petit modèle tient.
      Par branche : on voit où ça casse.
      Le routeur lui-même se teste.

  - id: repli-1
    sous_rubrique: repli
    question: "À quoi sert une chaîne de repli ?"
    choix:
      - "Envoyer la même requête à tous les modèles"
      - "Réduire la qualité volontairement"
      - "Basculer vers un autre modèle ou fournisseur quand le principal échoue ou ne répond pas"
      - "Remplacer les tests"
    reponse: 2
    pourquoi: >-
      Les fournisseurs ont des pannes et des limites de débit.
      Sans repli, ta panne suit la leur.
      Avec repli, le service continue, peut-être un peu moins bien.

  - id: repli-2
    sous_rubrique: repli
    question: "Le modèle principal renvoie une erreur 429 (trop de requêtes). Que faire ?"
    choix:
      - "Réessayer avec attente croissante, puis basculer sur le repli si ça persiste"
      - "Réessayer en boucle sans attendre"
      - "Abandonner et afficher une erreur brute"
      - "Envoyer la requête à tous les modèles de la chaîne"
    reponse: 0
    pourquoi: >-
      Une limite de débit est souvent passagère.
      Réessayer sans attendre aggrave la saturation.
      Après quelques essais, le repli prend le relais.
      C'est le backoff de l'étape 5.

  - id: repli-3
    sous_rubrique: repli
    question: "Le modèle principal refuse une demande pour raison de sécurité. Faut-il passer au modèle de repli ?"
    choix:
      - "Oui, tous les refus doivent être contournés"
      - "Oui, un autre modèle acceptera peut-être"
      - "Non : un refus n'est pas une panne. Le rejouer ailleurs contourne la règle"
      - "Oui, mais sans rien journaliser"
    reponse: 2
    pourquoi: >-
      Le repli sert aux pannes techniques.
      Un refus est une décision.
      Le contourner par un autre modèle, c'est chercher le plus permissif.
      On traite le refus, on ne le recycle pas.

  - id: repli-4
    sous_rubrique: repli
    question: "Ton modèle de repli n'a jamais été évalué. Risque ?"
    choix:
      - "Le jour de la panne, il répond, mais peut-être mal : format cassé, outils mal appelés"
      - "Aucun, un modèle en vaut un autre"
      - "Il sera forcément meilleur"
      - "Il refusera de démarrer"
    reponse: 0
    pourquoi: >-
      Chaque modèle réagit différemment au même prompt.
      Le repli ne sert qu'en cas de crise : c'est là qu'on découvre ses défauts.
      Il passe le même jeu de test que le principal.
      Parfois avec un prompt adapté.

  - id: repli-5
    sous_rubrique: repli
    question: "Que doit faire le dernier maillon d'une chaîne de repli ?"
    choix:
      - "Rien, la chaîne ne s'arrête jamais"
      - "Relancer toute la chaîne depuis le début, sans fin"
      - "Inventer une réponse"
      - "Échouer proprement : message clair, tâche mise en attente ou transmise à un humain"
    reponse: 3
    pourquoi: >-
      Tous les modèles peuvent être indisponibles en même temps.
      Une chaîne sans fin devient une boucle coûteuse.
      Un échec propre garde la confiance.
      La tâche peut reprendre plus tard.

  - id: repli-6
    sous_rubrique: repli
    question: "Pourquoi journaliser quel modèle a répondu à chaque requête ?"
    choix:
      - "Ce n'est pas utile"
      - "Pour relier la qualité, le coût et les incidents au bon modèle"
      - "Pour la décoration"
      - "Pour ralentir l'agent"
    reponse: 1
    pourquoi: >-
      Une baisse de qualité un mardi ? Le repli a peut-être tourné toute la journée.
      Sans cette trace, on cherche le bug au mauvais endroit.
      Le modèle utilisé fait partie de la trajectoire.
---

## Selon la complexité {#selon-complexite}

Toutes les requêtes ne se valent pas. Le **routage** envoie chacune au modèle le moins cher qui la traite bien.

:::pourquoi
Classer un e-mail ne demande pas le plus gros modèle.
Payer le gros modèle pour tout, c'est payer la difficulté sur chaque requête facile.
:::

| Tâche | Modèle |
|---|---|
| Classer, extraire, reformuler | Petit |
| Répondre à partir de documents fournis | Moyen |
| Planifier, raisonner longtemps, coder sur plusieurs fichiers | Gros |

Deux façons de router :

```mermaid
flowchart LR
  Q[Requête] --> R{Routeur}
  R -->|simple| P[Petit modèle]
  R -->|difficile| G[Gros modèle]
  Q2[Requête] --> P2[Petit modèle]
  P2 --> V{Vérificateur}
  V -->|réponse correcte| OK[Réponse]
  V -->|réponse faible| G2[Gros modèle]
  G2 --> OK
```

- Le **routeur** décide **avant**, à partir de la requête.
- La **cascade** décide **après**, à partir de la réponse du petit modèle.

:::pourquoi
RouteLLM (2024) apprend un routeur sur des préférences humaines.
Sur MT Bench, il garde 95 % de la performance de GPT-4 avec 26 % des requêtes envoyées à GPT-4.
FrugalGPT (2023) décrit la cascade et annonce jusqu'à 98 % d'économies au niveau de GPT-4 sur ses tests.
:::

Une cascade a un **coût caché**.

:::pourquoi
Une requête qui monte paie deux modèles et attend deux fois.
Si beaucoup montent, la cascade coûte plus que le gros modèle seul.
On suit le taux de montée.
:::

Le routage s'**évalue** branche par branche.

:::pourquoi
Une économie qui casse des réponses n'en est pas une.
Le jeu de test de l'étape 7 dit si le petit modèle tient sa branche.
:::

## Chaînes de repli {#repli}

Un fournisseur tombe en panne. Une limite de débit est atteinte. Le modèle ne répond pas.

La **chaîne de repli** prend le relais.

```mermaid
sequenceDiagram
  participant A as Application
  participant P as Modèle principal
  participant S as Modèle de repli
  participant H as Humain
  participant J as Journal
  A->>P: Requête
  P-->>A: Erreur 429
  A->>P: Nouvel essai après attente
  P-->>A: Erreur 429
  A->>S: Même requête (prompt adapté)
  S-->>A: Réponse
  A->>J: Réponse servie par le modèle de repli
  Note over A,H: Si le repli échoue aussi
  A->>H: Tâche mise en attente ou transmise
```

:::pourquoi
Sans repli, ta panne suit celle du fournisseur.
Avec repli, le service continue, peut-être un peu moins bien.
:::

Les règles :

1. **Réessayer** d'abord, avec attente croissante.
2. **Basculer** si l'erreur persiste.
3. **Échouer proprement** au bout de la chaîne.

:::pourquoi
Une erreur passagère se règle souvent en attendant.
Une chaîne sans fin devient une boucle coûteuse.
Un échec propre vaut mieux qu'une réponse inventée.
:::

Le repli sert aux **pannes**, pas aux **refus**.

:::pourquoi
Un refus de sécurité est une décision.
Le rejouer ailleurs, c'est chercher le modèle le plus permissif.
:::

Le modèle de repli passe le **même jeu de test**.

:::pourquoi
Le même prompt ne marche pas pareil sur tous les modèles.
Le repli ne sert qu'en crise : mauvais moment pour découvrir ses défauts.
:::

On **journalise** quel modèle a répondu.

:::pourquoi
Une baisse de qualité peut venir d'un repli resté actif toute la journée.
Sans trace, on cherche au mauvais endroit.
:::

## À retenir

- Router : petit modèle pour le simple, gros pour le difficile.
- Routeur avant, cascade après. Surveiller le taux de montée.
- Repli : réessayer, basculer, échouer proprement.
- Jamais de repli pour contourner un refus. Repli testé et journalisé.
