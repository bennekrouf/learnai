---
id: "05-05"
titre: "Multi-agent"
sous_rubriques:
  - id: topologies
    titre: "Topologies"
  - id: superviseur
    titre: "Superviseur"
  - id: etat-type
    titre: "État typé"
  - id: conflits
    titre: "Conflits"
  - id: un-seul
    titre: "Quand un seul agent suffit"
sources:
  - titre: "Anthropic — How we built our multi-agent research system, 2025"
    url: "https://www.anthropic.com/engineering/multi-agent-research-system"
  - titre: "Cognition (Walden Yan) — Don't Build Multi-Agents, 12 juin 2025"
    url: "https://cognition.com/blog/dont-build-multi-agents"
  - titre: "Anthropic — Building effective agents, 19 décembre 2024"
    url: "https://www.anthropic.com/engineering/building-effective-agents"
  - titre: "Cemri et al. — Why Do Multi-Agent LLM Systems Fail? (MAST), 2025"
    url: "https://arxiv.org/abs/2503.13657"
qcm:
  - id: topologies-1
    sous_rubrique: topologies
    question: "Dans la topologie « orchestrateur et travailleurs », que fait l'orchestrateur ?"
    choix:
      - "Il ne fait que relayer les messages"
      - "Il exécute tout lui-même"
      - "Il vote avec les autres agents"
      - "Il découpe la tâche, délègue les morceaux à des sous-agents, puis assemble leurs résultats"
    reponse: 3
    pourquoi: >-
      Le découpage n'est pas connu à l'avance.
      L'orchestrateur le décide selon la tâche.
      Les sous-agents travaillent chacun sur un morceau.
      L'orchestrateur rassemble et conclut.

  - id: topologies-2
    sous_rubrique: topologies
    question: "Anthropic décrit deux formes de parallélisation. Lesquelles ?"
    choix:
      - "Plusieurs GPU, ou plusieurs datacenters"
      - "Découper en sous-tâches indépendantes, ou lancer la même tâche plusieurs fois et voter"
      - "Plusieurs langues, ou plusieurs prompts système"
      - "Synchrone, ou asynchrone"
    reponse: 1
    pourquoi: >-
      Découper : chaque agent traite une partie différente.
      Voter : plusieurs agents traitent la même chose.
      Le premier va plus vite.
      Le second donne plus de confiance dans la réponse.

  - id: topologies-3
    sous_rubrique: topologies
    question: "Le système de recherche multi-agent d'Anthropic contre un agent seul (Opus 4), sur leur évaluation interne ?"
    choix:
      - "Environ 90 % de mieux"
      - "Pareil"
      - "Moins bien"
      - "10 fois mieux"
    reponse: 0
    pourquoi: >-
      Un Opus 4 orchestrateur, avec des sous-agents Sonnet 4.
      Il dépasse l'Opus 4 seul de 90,2 % sur leur évaluation de recherche.
      Surtout sur les questions larges, explorables en parallèle.

  - id: topologies-4
    sous_rubrique: topologies
    question: "Selon Anthropic, combien de tokens consomme un système multi-agent par rapport à un simple chat ?"
    choix:
      - "Moitié moins"
      - "Autant"
      - "Environ 15 fois plus"
      - "Environ 2 fois plus"
    reponse: 2
    pourquoi: >-
      Un agent seul consomme environ 4 fois plus qu'un chat.
      Un système multi-agent, environ 15 fois plus.
      La tâche doit valoir ce prix.

  - id: superviseur-1
    sous_rubrique: superviseur
    question: "Que doit recevoir un sous-agent de la part du superviseur ?"
    choix:
      - "Une phrase de trois mots"
      - "Un objectif, un format de sortie, les outils à utiliser, et les limites de sa tâche"
      - "Tout l'historique de tous les agents"
      - "Rien, il devine"
    reponse: 1
    pourquoi: >-
      Une consigne vague donne du travail en double ou à côté.
      L'objectif dit quoi chercher.
      Le format dit quoi rendre.
      Les limites disent où s'arrêter, et ce que font les autres.

  - id: superviseur-2
    sous_rubrique: superviseur
    question: "Qui décide que la tâche globale est terminée ?"
    choix:
      - "Personne"
      - "Le premier sous-agent qui finit"
      - "Le dernier sous-agent qui finit"
      - "Le superviseur, avec ses conditions d'arrêt"
    reponse: 3
    pourquoi: >-
      Chaque sous-agent ne voit qu'un morceau.
      Seul le superviseur voit l'ensemble.
      Il juge si la réponse est complète, ou s'il faut relancer.
      Il a ses limites de tours et de coût, comme tout agent.

  - id: superviseur-3
    sous_rubrique: superviseur
    question: "Dans le système d'Anthropic, quel agent ajoute les citations à la fin ?"
    choix:
      - "L'utilisateur"
      - "Chaque sous-agent, au hasard"
      - "Un agent spécialisé dans les citations, qui relie chaque affirmation à sa source"
      - "Aucun, il n'y a pas de citations"
    reponse: 2
    pourquoi: >-
      Le rapport est écrit d'abord.
      Un agent dédié relit les documents et le rapport.
      Il attache chaque affirmation à sa source.
      Une tâche précise, confiée à un agent précis.

  - id: etat-type-1
    sous_rubrique: etat-type
    question: "Pourquoi partager un état typé entre agents, plutôt que du texte libre ?"
    choix:
      - "Chaque champ a un type et un propriétaire. On valide ce qui entre"
      - "Le texte libre coûte plus cher"
      - "Les modèles ne lisent pas le texte"
      - "C'est plus joli"
    reponse: 0
    pourquoi: >-
      Un résumé en texte libre se lit de dix façons.
      Un champ « sources: liste d'URL » se vérifie.
      On sait qui écrit quoi.
      On rejette une sortie mal formée avant qu'elle contamine les autres.

  - id: etat-type-2
    sous_rubrique: etat-type
    question: "Deux sous-agents écrivent en même temps dans le même champ de l'état. Que prévoir ?"
    choix:
      - "Interdire le parallélisme"
      - "Le dernier qui écrit gagne, sans rien dire"
      - "Une règle de fusion pour ce champ, ou un seul agent autorisé à l'écrire"
      - "Laisser le modèle choisir après coup"
    reponse: 2
    pourquoi: >-
      « Le dernier gagne » efface silencieusement un travail.
      Une liste peut se concaténer.
      Un statut doit avoir un seul propriétaire.
      La règle est décidée à l'avance, pas découverte en production.

  - id: etat-type-3
    sous_rubrique: etat-type
    question: "Quand valider la sortie d'un sous-agent ?"
    choix:
      - "Avant de la fusionner dans l'état partagé"
      - "Jamais"
      - "Seulement à la toute fin"
      - "Seulement si elle est longue"
    reponse: 0
    pourquoi: >-
      Une sortie invalide qui entre dans l'état contamine les autres agents.
      La trouver à la fin, c'est chercher d'où elle vient.
      À la frontière, on sait exactement quel agent l'a produite.
      Il peut être relancé avec l'erreur.

  - id: conflits-1
    sous_rubrique: conflits
    question: "Cognition (2025) : un sous-agent dessine un décor façon Mario, l'autre un oiseau d'un autre style. Quel principe ont-ils oublié ?"
    choix:
      - "Augmenter la température"
      - "Utiliser un plus gros modèle"
      - "Ajouter plus de sous-agents"
      - "Partager tout le contexte : chaque action porte des décisions implicites"
    reponse: 3
    pourquoi: >-
      La tâche : un clone de Flappy Bird.
      Chaque sous-agent n'a reçu que son morceau.
      Chacun a fait des choix de style sans voir ceux de l'autre.
      Les morceaux ne vont plus ensemble.

  - id: conflits-2
    sous_rubrique: conflits
    question: "Deux sous-agents rapportent des chiffres contradictoires. Que fait le superviseur ?"
    choix:
      - "Il garde le premier arrivé"
      - "Il compare les sources, et tranche ou signale le désaccord"
      - "Il fait la moyenne"
      - "Il cache le désaccord"
    reponse: 1
    pourquoi: >-
      Une moyenne de deux chiffres faux reste fausse.
      Les sources disent lequel est fiable.
      Si on ne peut pas trancher, on le dit.
      Un désaccord visible vaut mieux qu'une certitude inventée.

  - id: conflits-3
    sous_rubrique: conflits
    question: "Pour quel type de question un vote à la majorité entre plusieurs agents marche-t-il bien ?"
    choix:
      - "N'importe lequel"
      - "Un long rapport rédigé"
      - "Un design graphique"
      - "Une réponse courte et comparable : un chiffre, une étiquette, oui ou non"
    reponse: 3
    pourquoi: >-
      Pour voter, il faut compter des réponses identiques.
      Trois rapports différents ne se comptent pas.
      « 42 », « 42 », « 41 » se comptent.
      Le vote réduit les erreurs isolées.

  - id: conflits-4
    sous_rubrique: conflits
    question: "L'étude MAST (2025) classe les échecs des systèmes multi-agents. En quelles catégories ?"
    choix:
      - "Réseau, disque, mémoire"
      - "Conception du système, désalignement entre agents, vérification de la tâche"
      - "Prompt, température, modèle"
      - "Coût, latence, sécurité"
    reponse: 1
    pourquoi: >-
      14 modes d'échec, regroupés en 3 catégories.
      Établis à partir de traces de 7 frameworks multi-agents.
      La plupart ne viennent pas du modèle seul.
      Ils viennent de l'organisation entre agents.

  - id: un-seul-1
    sous_rubrique: un-seul
    question: "Que recommande Cognition à la place d'un système multi-agent ?"
    choix:
      - "Un seul agent, linéaire, avec compression du contexte quand il devient long"
      - "Dix agents qui votent"
      - "Pas d'agent du tout"
      - "Un agent par fichier"
    reponse: 0
    pourquoi: >-
      Un seul fil d'exécution voit toutes ses décisions.
      Rien n'est perdu entre deux agents.
      Quand le contexte grossit, on le résume.
      C'est plus simple et plus fiable pour beaucoup de tâches.

  - id: un-seul-2
    sous_rubrique: un-seul
    question: "Selon Anthropic, pour quelles tâches le multi-agent est-il mal adapté ?"
    choix:
      - "Les questions à plusieurs pistes indépendantes"
      - "La recherche large en parallèle"
      - "Celles où tous les agents doivent partager le même contexte, et la plupart du code"
      - "Aucune"
    reponse: 2
    pourquoi: >-
      Le code est très interdépendant.
      Une décision dans un fichier en contraint dix autres.
      Les sous-agents ne voient pas ces liens.
      Le multi-agent brille quand les pistes sont indépendantes.

  - id: un-seul-3
    sous_rubrique: un-seul
    question: "Par où commencer, selon « Building effective agents » ?"
    choix:
      - "Le système multi-agent le plus complet"
      - "La solution la plus simple. Ajouter de la complexité seulement si elle améliore mesurablement"
      - "Un framework avant tout"
      - "Dix sous-agents par défaut"
    reponse: 1
    pourquoi: >-
      Chaque agent ajouté coûte des tokens, de la latence, des échecs possibles.
      Un appel bien conçu suffit souvent.
      Puis un workflow. Puis un agent.
      Le multi-agent vient en dernier, s'il se justifie.

  - id: un-seul-4
    sous_rubrique: un-seul
    question: "Anthropic a mesuré que l'usage de tokens explique 80 % des écarts de performance sur BrowseComp. Qu'en conclure ?"
    choix:
      - "Le multi-agent est gratuit"
      - "Les tokens ne comptent pas"
      - "Il faut toujours utiliser le plus petit modèle"
      - "Une bonne part du gain multi-agent vient de la dépense de tokens supplémentaire"
    reponse: 3
    pourquoi: >-
      Plusieurs agents explorent plus, donc dépensent plus.
      Cette dépense explique l'essentiel du gain.
      La question devient : la tâche vaut-elle ce coût ?
      Pour une question simple, non.
---

## Topologies {#topologies}

Plusieurs agents peuvent travailler ensemble. Trois formes reviennent souvent.

| Topologie | Principe | Quand |
|---|---|---|
| Orchestrateur et travailleurs | Un agent découpe, délègue, assemble | Découpage inconnu à l'avance |
| Parallélisation par découpage | Chaque agent traite une partie | Parties indépendantes |
| Parallélisation par vote | Plusieurs agents, même tâche | Besoin de confiance |

:::pourquoi
Anthropic décrit ces motifs dans « Building effective agents ».
Chacun répond à un besoin différent : vitesse, couverture, fiabilité.
:::

Anthropic a construit un système de recherche en orchestrateur et travailleurs. Un Opus 4 dirige des sous-agents Sonnet 4. Résultat : +90,2 % par rapport à un Opus 4 seul, sur leur évaluation interne.

:::pourquoi
Les questions de recherche larges s'explorent en parallèle.
Chaque sous-agent fouille une piste avec son propre contexte.
:::

Le prix : environ **15 fois** les tokens d'un chat. Un agent seul : environ 4 fois.

:::pourquoi
Chaque sous-agent a son contexte, ses appels, ses résultats.
La tâche doit valoir ce coût.
:::

## Superviseur {#superviseur}

```mermaid
sequenceDiagram
  participant U as Utilisateur
  participant L as Superviseur
  participant A as Sous-agent A
  participant B as Sous-agent B
  participant C as Agent de citations
  U->>L: Question de recherche
  L->>L: Plan, sauvegardé
  L->>A: Objectif, format, outils, limites
  L->>B: Objectif, format, outils, limites
  A-->>L: Résultats et sources
  B-->>L: Résultats et sources
  L->>L: Assez ? Sinon, relance
  L->>C: Rapport et documents
  C-->>L: Rapport avec citations
  L-->>U: Réponse finale
```

Le superviseur **découpe**, **délègue**, **rassemble**, **décide** de la fin.

:::pourquoi
Chaque sous-agent ne voit qu'un morceau.
Seul le superviseur voit l'ensemble.
:::

Chaque délégation précise : l'objectif, le format de sortie, les outils, les limites.

:::pourquoi
Une consigne vague donne du travail en double, ou à côté.
Les limites disent aussi ce que font les autres.
:::

Une tâche précise peut avoir son agent précis. Chez Anthropic, un agent dédié ajoute les citations à la fin.

:::pourquoi
Relier chaque affirmation à sa source est un travail à part.
Un agent spécialisé le fait mieux qu'en passant.
:::

## État typé {#etat-type}

Les agents partagent un **état typé** : des champs nommés, avec un type et un propriétaire.

```json
{
  "question": "string",
  "pistes": [{ "id": "string", "agent": "string", "statut": "a_faire | en_cours | fini" }],
  "sources": [{ "url": "string", "extrait": "string", "piste": "string" }],
  "rapport": "string | null"
}
```

:::pourquoi
Un résumé en texte libre se lit de dix façons.
Un champ typé se valide.
:::

Chaque champ a une règle d'écriture : un seul propriétaire, ou une fusion définie.

:::pourquoi
« Le dernier qui écrit gagne » efface un travail sans bruit.
Une liste de sources se concatène. Un statut a un seul maître.
:::

La sortie d'un sous-agent est validée **avant** d'entrer dans l'état.

:::pourquoi
À la frontière, on sait quel agent a produit l'erreur.
Après la fusion, elle contamine tout.
:::

## Conflits {#conflits}

Cognition raconte un exemple. Tâche : un clone de Flappy Bird. Un sous-agent fait un décor façon Mario. L'autre, un oiseau qui ne va pas avec.

:::pourquoi
Chaque action porte des décisions implicites : un style, une hypothèse.
Un agent qui ne voit pas les choix de l'autre décide autrement.
Cognition en tire un principe : partager tout le contexte.
:::

Deux résultats se contredisent ? Le superviseur compare les sources. Il tranche, ou il signale le désaccord.

:::pourquoi
Une moyenne de deux chiffres faux reste fausse.
Un désaccord visible vaut mieux qu'une certitude inventée.
:::

Pour une réponse courte et comparable, le **vote** marche bien : plusieurs agents, la majorité gagne.

:::pourquoi
« 42 », « 42 », « 41 » se comptent.
Trois rapports rédigés, non.
:::

L'étude MAST (2025) classe 14 modes d'échec en 3 catégories : conception du système, désalignement entre agents, vérification de la tâche.

:::pourquoi
La plupart des échecs ne viennent pas du modèle seul.
Ils viennent de l'organisation entre agents.
:::

## Quand un seul agent suffit {#un-seul}

Souvent.

Anthropic le dit : le multi-agent convient mal aux tâches où tous doivent partager le même contexte, et à la plupart du code.

:::pourquoi
Dans le code, une décision en contraint dix autres.
Des sous-agents isolés ne voient pas ces liens.
:::

Cognition recommande un seul agent, linéaire. Quand le contexte devient long, on le compresse.

:::pourquoi
Un seul fil voit toutes ses décisions.
Rien ne se perd entre deux agents.
:::

Sur BrowseComp, l'usage de tokens explique à lui seul 80 % des écarts de performance mesurés par Anthropic.

:::pourquoi
Une bonne part du gain vient de la dépense.
La vraie question : la tâche vaut-elle ce coût ?
:::

```mermaid
flowchart TD
  Q["Nouvelle tâche"] --> S{"Un appel bien conçu suffit ?"}
  S -->|"oui"| A1["Un appel"]
  S -->|"non"| W{"Étapes connues à l'avance ?"}
  W -->|"oui"| A2["Workflow"]
  W -->|"non"| P{"Pistes indépendantes, valeur élevée, trop grand pour un contexte ?"}
  P -->|"non"| A3["Un seul agent"]
  P -->|"oui"| A4["Multi-agent"]
```

:::pourquoi
Anthropic conseille de commencer par la solution la plus simple.
On n'ajoute de la complexité que si elle améliore mesurablement le résultat.
:::

## À retenir

- Orchestrateur et travailleurs, découpage, vote : chaque topologie a son usage.
- Le superviseur délègue avec objectif, format, outils, limites. Il décide de la fin.
- État typé, propriétaire par champ, validation à la frontière.
- Les conflits viennent du contexte non partagé. Comparer les sources, voter sur du comparable.
- Un seul agent suffit souvent. Le multi-agent coûte environ 15 fois un chat.
