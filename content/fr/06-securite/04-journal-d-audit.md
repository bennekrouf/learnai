---
id: "06-04"
titre: "Journal d'audit"
sous_rubriques:
  - id: quoi-tracer
    titre: "Quoi tracer"
  - id: inalterable
    titre: "Rendre inaltérable"
sources:
  - titre: "AI Act (UE) — Article 12, Enregistrement (journaux)"
    url: "https://artificialintelligenceact.eu/article/12/"
  - titre: "AI Act (UE) — Article 19, Journaux générés automatiquement"
    url: "https://artificialintelligenceact.eu/article/19/"
  - titre: "OWASP — Logging Cheat Sheet"
    url: "https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html"
qcm:
  - id: quoi-tracer-1
    sous_rubrique: quoi-tracer
    question: "À quoi sert un journal d'audit pour un agent ?"
    choix:
      - "Accélérer l'agent"
      - "Reconstituer après coup qui a fait quoi, quand, pourquoi, et avec quel résultat"
      - "Réduire le coût en tokens"
      - "Remplacer les tests"
    reponse: 1
    pourquoi: >-
      Un incident arrive toujours après coup.
      Sans trace, on ne sait pas ce que l'agent a lu ni fait.
      Le journal permet de comprendre, de corriger, de prouver.

  - id: quoi-tracer-2
    sous_rubrique: quoi-tracer
    question: "Pour chaque appel d'outil, que faut-il tracer ?"
    choix:
      - "L'identité (utilisateur et agent), l'outil, les arguments, la décision d'autorisation, le résultat, l'heure"
      - "Seulement l'heure"
      - "Seulement les erreurs"
      - "Seulement le nom de l'outil"
    reponse: 0
    pourquoi: >-
      Qui : pour la responsabilité.
      Quoi et avec quels arguments : pour comprendre l'action.
      La décision : autorisé, refusé, validé par qui.
      Le résultat et l'heure : pour reconstruire la chronologie.

  - id: quoi-tracer-3
    sous_rubrique: quoi-tracer
    question: "Pourquoi tracer la provenance des contenus lus par l'agent (quel e-mail, quelle page) ?"
    choix:
      - "Ce n'est pas utile"
      - "Pour facturer les sites web"
      - "Pour retrouver le contenu piégé à l'origine d'une injection"
      - "Pour entraîner le modèle"
    reponse: 2
    pourquoi: >-
      Après EchoLeak, la question est : quel e-mail a déclenché l'action ?
      Sans provenance, on voit l'action, pas la cause.
      Avec, on retrouve la source et on la neutralise.

  - id: quoi-tracer-4
    sous_rubrique: quoi-tracer
    question: "Pourquoi noter la version du modèle et du prompt dans chaque trace ?"
    choix:
      - "Pour la décoration"
      - "Le comportement change d'une version à l'autre. Il faut savoir laquelle a agi"
      - "Parce que le modèle l'exige"
      - "Pour réduire la taille des logs"
    reponse: 1
    pourquoi: >-
      Un incident peut venir d'un nouveau prompt.
      Ou d'une mise à jour du modèle.
      Sans version, on ne peut pas reproduire.
      Ni savoir si le correctif a vraiment changé quelque chose.

  - id: quoi-tracer-5
    sous_rubrique: quoi-tracer
    question: "Faut-il écrire les mots de passe et les données personnelles en clair dans le journal ?"
    choix:
      - "Oui, si le journal est privé"
      - "Oui, tout, pour être complet"
      - "Oui, mais seulement le week-end"
      - "Non : jamais de secret, et les données personnelles masquées ou minimisées"
    reponse: 3
    pourquoi: >-
      Le journal est lu par plus de monde que la base d'origine.
      Il est gardé longtemps.
      Un secret dans un log est une fuite qui dure.
      On trace l'action, pas forcément la donnée brute.

  - id: quoi-tracer-6
    sous_rubrique: quoi-tracer
    question: "Plusieurs agents traitent une même demande. Comment relier leurs traces ?"
    choix:
      - "Par le nom de l'utilisateur"
      - "Par l'heure, approximativement"
      - "Un identifiant de corrélation commun, transmis à chaque agent et chaque outil"
      - "On ne peut pas"
    reponse: 2
    pourquoi: >-
      Une demande peut passer par cinq agents et vingt outils.
      Un même identifiant les relie tous.
      On reconstruit le chemin complet en une recherche.

  - id: quoi-tracer-7
    sous_rubrique: quoi-tracer
    question: "Pour un système IA à haut risque, que prévoit l'AI Act européen sur les journaux ?"
    choix:
      - "L'enregistrement automatique d'événements, et une conservation d'au moins six mois en principe"
      - "Rien"
      - "Une conservation d'une journée"
      - "L'interdiction de tout journal"
    reponse: 0
    pourquoi: >-
      L'article 12 exige que ces systèmes permettent l'enregistrement automatique d'événements.
      Objectif : la traçabilité de leur fonctionnement.
      L'article 19 fixe une conservation d'au moins six mois, sauf autre règle.
      Ces obligations visent les systèmes classés à haut risque.

  - id: inalterable-1
    sous_rubrique: inalterable
    question: "Pourquoi un journal d'audit doit-il être inaltérable ?"
    choix:
      - "Pour qu'il soit plus rapide"
      - "Pour économiser du disque"
      - "Un attaquant, ou l'agent piégé, ne doit pas pouvoir effacer ses traces"
      - "Parce que JSON l'exige"
    reponse: 2
    pourquoi: >-
      Un journal modifiable ne prouve rien.
      Celui qui a fait la faute pourrait la retirer.
      Un journal en ajout seul garde la vérité, même gênante.

  - id: inalterable-2
    sous_rubrique: inalterable
    question: "Qu'est-ce qu'une chaîne de hachage dans un journal ?"
    choix:
      - "Chaque entrée contient l'empreinte de la précédente. Modifier une entrée casse toute la suite"
      - "Un chiffrement des entrées"
      - "Une compression des logs"
      - "Un tri par date"
    reponse: 0
    pourquoi: >-
      L'entrée 42 contient le hash de l'entrée 41.
      Si on modifie l'entrée 41, son hash change.
      L'entrée 42 ne correspond plus.
      La modification devient visible.

  - id: inalterable-3
    sous_rubrique: inalterable
    question: "L'agent peut-il avoir le droit de supprimer ou modifier ses propres entrées de journal ?"
    choix:
      - "Oui, en fin de journée"
      - "Oui, pour corriger ses erreurs"
      - "Oui, s'il le demande poliment"
      - "Non : il peut seulement ajouter. Lecture et administration sont séparées"
    reponse: 3
    pourquoi: >-
      Un agent piégé pourrait effacer la preuve de l'attaque.
      Droit d'ajout seulement.
      Les auditeurs lisent. Personne ne réécrit.

  - id: inalterable-4
    sous_rubrique: inalterable
    question: "Une chaîne de hachage suffit-elle si l'attaquant peut tout réécrire, chaîne comprise ?"
    choix:
      - "Oui, toujours"
      - "Non : il faut ancrer régulièrement le dernier hash ailleurs, ou stocker en écriture unique"
      - "Oui, si les hash sont longs"
      - "Oui, si le fichier est caché"
    reponse: 1
    pourquoi: >-
      Qui contrôle tout peut recalculer toute la chaîne.
      On publie donc le dernier hash ailleurs, régulièrement.
      Ou on utilise un stockage en écriture unique.
      La réécriture devient alors détectable ou impossible.

  - id: inalterable-5
    sous_rubrique: inalterable
    question: "Comment savoir qu'un journal n'a pas été modifié ?"
    choix:
      - "Compter les lignes"
      - "Le relire à l'œil"
      - "Faire confiance"
      - "Vérifier régulièrement la chaîne de hachage, et alerter si elle est cassée"
    reponse: 3
    pourquoi: >-
      Une protection jamais vérifiée ne protège rien.
      Un contrôle automatique recalcule la chaîne.
      Une rupture déclenche une alerte.
      On sait quand et où la modification a eu lieu.
---

## Quoi tracer {#quoi-tracer}

Un incident se découvre **après**. Le journal d'audit permet de le reconstituer.

Pour chaque action :

| Champ | Exemple | Sert à |
|---|---|---|
| Qui | utilisateur 812, agent « support-v3 » | Responsabilité |
| Quoi | `rembourser`, `{client: 812, montant: 50}` | Comprendre l'action |
| Provenance | e-mail 9f3a, page doc.interne/x | Retrouver une injection |
| Décision | autorisé par la politique, validé par Alice | Contrôle |
| Résultat | succès, refus, erreur | Conséquence |
| Versions | modèle, prompt, code | Reproduire |
| Quand | horodatage | Chronologie |
| Corrélation | `req-7b21` | Relier les agents |

:::pourquoi
Sans provenance, on voit l'action mais pas sa cause.
Sans version, on ne peut pas reproduire.
Sans corrélation, une demande éclatée sur cinq agents est illisible.
:::

```mermaid
sequenceDiagram
  participant U as Utilisateur
  participant S as Superviseur
  participant A as Sous-agent
  participant T as Outil
  participant J as Journal d'audit
  U->>S: Demande (req-7b21)
  S->>J: req-7b21 : demande reçue, versions
  S->>A: Délègue (req-7b21)
  A->>T: lire_mail(id=9f3a)
  A->>J: req-7b21 : lecture, provenance mail 9f3a
  A->>T: rembourser(812, 50)
  A->>J: req-7b21 : action, arguments, autorisé, succès
```

Pas de secret dans le journal. Les données personnelles masquées ou réduites.

:::pourquoi
Le journal est lu par plus de monde, et gardé plus longtemps, que la base d'origine.
Un secret dans un log est une fuite qui dure.
:::

L'AI Act européen exige pour les systèmes à haut risque un enregistrement automatique des événements (article 12). Les journaux sont conservés au moins six mois, sauf autre règle (article 19).

:::pourquoi
Le législateur veut la traçabilité de ces systèmes.
Même hors haut risque, c'est la base pour enquêter.
:::

## Rendre inaltérable {#inalterable}

Un journal qu'on peut modifier ne prouve rien.

**Ajout seul** : l'agent écrit de nouvelles entrées. Il ne modifie ni ne supprime.

:::pourquoi
Un agent piégé pourrait effacer la trace de l'attaque.
:::

**Chaîne de hachage** : chaque entrée contient l'empreinte de la précédente.

```mermaid
flowchart LR
  E1["Entrée 41<br/>hash = H41"] --> E2["Entrée 42<br/>précédent = H41<br/>hash = H42"]
  E2 --> E3["Entrée 43<br/>précédent = H42<br/>hash = H43"]
  E3 --> A["H43 publié ailleurs<br/>(ancrage)"]
```

:::pourquoi
Modifier l'entrée 41 change son hash.
L'entrée 42 ne correspond plus. La modification se voit.
:::

**Ancrage** : publie régulièrement le dernier hash ailleurs. Ou stocke en **écriture unique**.

:::pourquoi
Qui contrôle tout peut recalculer toute la chaîne.
Un hash publié ailleurs l'en empêche.
:::

**Vérifie** la chaîne automatiquement, et alerte si elle casse.

:::pourquoi
Une protection jamais vérifiée ne protège rien.
:::

**Sépare** les rôles : l'agent ajoute, les auditeurs lisent, personne ne réécrit.

## À retenir

- Tracer : qui, quoi, provenance, décision, résultat, versions, heure, corrélation.
- Pas de secret. Données personnelles masquées.
- AI Act, haut risque : journaux automatiques, au moins six mois.
- Ajout seul, chaîne de hachage, ancrage externe, vérification régulière.
