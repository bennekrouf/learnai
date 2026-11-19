---
id: "06-02"
titre: "Données personnelles"
sous_rubriques:
  - id: masquage
    titre: "Masquage réversible"
  - id: jamais
    titre: "Ce qui ne sort jamais"
sources:
  - titre: "OWASP — Top 10 for LLM Applications 2025, LLM02 Sensitive Information Disclosure"
    url: "https://genai.owasp.org/llmrisk/llm022025-sensitive-information-disclosure/"
  - titre: "Fortune — Samsung bans employee use of ChatGPT after data leak, 2 mai 2023"
    url: "https://fortune.com/2023/05/02/samsung-bans-employee-use-chatgpt-data-leak/"
  - titre: "RGPD — article 4 (définition de la pseudonymisation) et considérant 26"
    url: "https://eur-lex.europa.eu/legal-content/FR/TXT/?uri=CELEX:32016R0679"
qcm:
  - id: masquage-1
    sous_rubrique: masquage
    question: "Qu'est-ce que le masquage réversible avant un appel au modèle ?"
    choix:
      - "Remplacer chaque donnée personnelle par un jeton, garder la table de correspondance chez soi, et restaurer dans la réponse"
      - "Supprimer définitivement les données personnelles"
      - "Chiffrer tout le prompt"
      - "Envoyer les données en majuscules"
    reponse: 0
    pourquoi: >-
      « Marie Dupont » devient « PERSONNE_1 ».
      Le modèle travaille sur PERSONNE_1.
      La table PERSONNE_1 = Marie Dupont reste sur ton serveur.
      Dans la réponse, on remet le vrai nom.

  - id: masquage-2
    sous_rubrique: masquage
    question: "Où doit vivre la table de correspondance jeton ↔ vraie valeur ?"
    choix:
      - "Dans les logs, pour déboguer"
      - "Dans le prompt, pour que le modèle comprenne"
      - "Sur ton infrastructure, jamais envoyée au fournisseur du modèle"
      - "Chez le fournisseur du modèle"
    reponse: 2
    pourquoi: >-
      La table est la clé du masquage.
      L'envoyer avec le prompt annule tout.
      La mettre dans les logs crée une nouvelle fuite.
      Elle se protège comme les données d'origine.

  - id: masquage-3
    sous_rubrique: masquage
    question: "Pourquoi garder le même jeton pour la même personne dans toute la conversation ?"
    choix:
      - "Pour économiser de la mémoire"
      - "Pour que le modèle comprenne que c'est la même personne d'un message à l'autre"
      - "Ce n'est pas utile"
      - "Pour tromper le modèle"
    reponse: 1
    pourquoi: >-
      Si Marie devient PERSONNE_1, puis PERSONNE_7, le modèle voit deux personnes.
      Son raisonnement devient faux.
      Un jeton stable préserve le sens.
      Le masquage ne doit pas casser la tâche.

  - id: masquage-4
    sous_rubrique: masquage
    question: "Ton détecteur trouve 95 % des noms. Que faire des 5 % restants ?"
    choix:
      - "Désactiver le masquage"
      - "Les ignorer, 95 % c'est très bien"
      - "Demander au modèle de les oublier"
      - "Les traiter comme un risque réel : combiner détecteurs, règles strictes, et réduire ce qu'on envoie"
    reponse: 3
    pourquoi: >-
      Sur un million de messages, 5 % c'est 50 000 fuites.
      Un détecteur statistique rate des cas.
      On ajoute des règles exactes pour les formats connus.
      Et on n'envoie que les champs dont la tâche a besoin.

  - id: masquage-5
    sous_rubrique: masquage
    question: "Des données pseudonymisées (masquées avec une table) sont-elles encore des données personnelles au sens du RGPD ?"
    choix:
      - "Seulement le week-end"
      - "Non, plus jamais"
      - "Oui : avec la table, on peut retrouver la personne"
      - "Seulement si elles sont en anglais"
    reponse: 2
    pourquoi: >-
      Le RGPD distingue pseudonymisation et anonymisation.
      Pseudonymiser réduit le risque, mais la personne reste identifiable.
      Les obligations du RGPD continuent de s'appliquer.
      Seule une anonymisation vraie les fait sortir du champ.

  - id: masquage-6
    sous_rubrique: masquage
    question: "Qui a le droit de voir la réponse restaurée, avec les vrais noms ?"
    choix:
      - "Seulement un utilisateur autorisé à voir ces données d'origine"
      - "Tout le monde"
      - "Le modèle"
      - "Les sous-traitants"
    reponse: 0
    pourquoi: >-
      Restaurer, c'est réafficher la donnée personnelle.
      On applique les mêmes droits qu'à la source.
      Un stagiaire qui n'a pas accès au dossier ne le voit pas via l'agent.
      L'agent ne doit pas devenir une porte dérobée.

  - id: jamais-1
    sous_rubrique: jamais
    question: "Avril 2023. Que s'est-il passé chez Samsung ?"
    choix:
      - "Samsung a acheté OpenAI"
      - "ChatGPT a piraté Samsung"
      - "Des ingénieurs ont collé du code source interne dans ChatGPT. Samsung a ensuite restreint ces outils"
      - "Un modèle a écrit du code faux pour Samsung"
    reponse: 2
    pourquoi: >-
      Les ingénieurs voulaient de l'aide pour corriger du code.
      Le code est sorti de l'entreprise.
      Début mai 2023, Samsung interdit ces outils sur ses appareils et réseaux.
      La fuite venait de l'utilisateur, pas du modèle.

  - id: jamais-2
    sous_rubrique: jamais
    question: "Quelles données ne doivent jamais être envoyées au modèle ?"
    choix:
      - "Les secrets : mots de passe, clés d'API, jetons. Et ce que la tâche n'exige pas"
      - "Les questions de l'utilisateur"
      - "Les descriptions d'outils"
      - "Les dates"
    reponse: 0
    pourquoi: >-
      Un secret dans le contexte peut ressortir dans une réponse.
      Ou dans une URL, comme avec EchoLeak.
      Le modèle n'a jamais besoin de la clé : l'outil, lui, l'utilise.
      Le reste : on envoie le minimum nécessaire.

  - id: jamais-3
    sous_rubrique: jamais
    question: "Un outil doit appeler une API avec une clé secrète. Comment faire ?"
    choix:
      - "La passer en argument d'outil"
      - "Mettre la clé dans le prompt système"
      - "Demander au modèle de la mémoriser"
      - "Le code de l'outil lit la clé côté serveur. Le modèle ne la voit jamais"
    reponse: 3
    pourquoi: >-
      Le modèle choisit l'outil et ses arguments métier.
      Le code ajoute l'authentification.
      Une clé que le modèle ne voit pas, il ne peut pas la fuiter.
      Même sous injection.

  - id: jamais-4
    sous_rubrique: jamais
    question: "Peut-on mettre un secret dans le prompt système, s'il est « caché » à l'utilisateur ?"
    choix:
      - "Oui, il est invisible"
      - "Non : le prompt système peut être extrait par l'utilisateur"
      - "Oui, s'il est en fin de prompt"
      - "Oui, si on écrit « confidentiel »"
    reponse: 1
    pourquoi: >-
      Le prompt système est du texte dans le contexte.
      Des utilisateurs ont souvent réussi à le faire réciter.
      Considère-le comme public.
      Aucun secret, aucune règle de sécurité qui ne repose que sur lui.

  - id: jamais-5
    sous_rubrique: jamais
    question: "Comment filtrer de façon fiable un numéro de carte bancaire avant l'envoi ?"
    choix:
      - "Le mettre en italique"
      - "Demander au modèle de ne pas le lire"
      - "Rien, le modèle l'ignorera"
      - "Une règle exacte : motif de chiffres, plus contrôle de Luhn. Bloquer ou masquer avant l'appel"
    reponse: 3
    pourquoi: >-
      Un format connu se détecte avec une règle exacte.
      Le contrôle de Luhn réduit les faux positifs.
      La règle s'applique avant l'appel, dans le code.
      Ce qui n'est pas envoyé ne peut pas fuiter.

  - id: jamais-6
    sous_rubrique: jamais
    question: "Le filtre de sortie sert à quoi, si on filtre déjà l'entrée ?"
    choix:
      - "À rien"
      - "À rattraper ce qui ressort quand même : donnée devinée, mal masquée, ou injectée"
      - "À accélérer la réponse"
      - "À traduire la réponse"
    reponse: 1
    pourquoi: >-
      Le contexte peut contenir des données venant d'outils.
      Le modèle peut recombiner des morceaux.
      Une injection peut lui demander de recopier.
      On vérifie aussi ce qui sort : réponse, URL, arguments d'outils.
---

## Masquage réversible {#masquage}

Avant d'envoyer un texte au modèle, on **remplace** les données personnelles par des jetons.

```mermaid
sequenceDiagram
  participant U as Utilisateur
  participant M as Masqueur (ton serveur)
  participant T as Table de correspondance
  participant L as Modèle
  U->>M: « Rédige un mail à Marie Dupont, 06 12 34 56 78 »
  M->>T: PERSONNE_1 = Marie Dupont, TEL_1 = 06 12 34 56 78
  M->>L: « Rédige un mail à PERSONNE_1, TEL_1 »
  L-->>M: « Bonjour PERSONNE_1, … »
  M->>T: Lit les vraies valeurs
  M-->>U: « Bonjour Marie Dupont, … »
```

:::pourquoi
Le modèle n'a pas besoin du vrai nom pour rédiger.
Ce qu'il ne reçoit pas, il ne peut ni le garder ni le fuiter.
:::

La **table** reste chez toi. Ni dans le prompt, ni dans les logs.

:::pourquoi
La table est la clé.
L'envoyer ou la journaliser annule le masquage.
:::

Un même nom garde le **même jeton** dans toute la conversation.

:::pourquoi
Sinon le modèle croit voir plusieurs personnes.
Le masquage ne doit pas casser la tâche.
:::

Combine les détecteurs : règles exactes pour les formats connus (e-mail, téléphone, IBAN), modèle de détection pour les noms.

:::pourquoi
Un détecteur statistique rate des cas.
5 % de ratés sur un million de messages, c'est 50 000 fuites.
:::

La restauration suit les **droits** de la source.

:::pourquoi
L'agent ne doit pas montrer à quelqu'un ce qu'il n'aurait pas pu voir lui-même.
:::

Pseudonymiser n'est pas anonymiser. Pour le RGPD, ce sont encore des données personnelles.

:::pourquoi
Avec la table, on retrouve la personne.
Le risque baisse. Les obligations restent.
:::

## Ce qui ne sort jamais {#jamais}

Avril 2023. Des ingénieurs de Samsung collent du code source interne dans ChatGPT pour le corriger. Début mai, Samsung restreint ces outils sur ses appareils et réseaux.

:::pourquoi
La fuite ne venait pas d'une attaque.
Elle venait d'un usage normal, sans garde-fou.
OWASP classe ce risque en deuxième position en 2025.
:::

Trois catégories :

| Catégorie | Exemples | Règle |
|---|---|---|
| Secrets | Mots de passe, clés d'API, jetons | Jamais dans le contexte |
| Données très sensibles | Carte bancaire, santé, pièces d'identité | Bloquées ou masquées avant l'appel |
| Le reste | Tout ce que la tâche n'exige pas | Pas envoyé : minimisation |

:::pourquoi
Ce qui n'entre pas dans le contexte ne peut pas en sortir.
:::

Les clés d'API vivent dans le **code de l'outil**. Le modèle ne les voit jamais.

:::pourquoi
Le modèle choisit l'outil. Le code ajoute l'authentification.
Même sous injection, il ne peut pas révéler une clé qu'il n'a pas.
:::

Le prompt système n'est pas un coffre-fort.

:::pourquoi
C'est du texte dans le contexte.
Des utilisateurs réussissent régulièrement à le faire réciter.
:::

Filtre **à l'entrée** et **à la sortie** : réponse, URL, arguments d'outils.

```mermaid
flowchart LR
  I["Entrée"] --> F1["Filtre : secrets, cartes, masquage"]
  F1 --> L["Modèle"]
  L --> F2["Filtre : données sensibles, URL externes"]
  F2 --> O["Réponse ou appel d'outil"]
```

:::pourquoi
L'entrée filtrée peut laisser passer un cas.
Un outil peut ramener une donnée sensible en cours de route.
La sortie est le dernier point de contrôle.
:::

## À retenir

- Masquer avant l'appel, restaurer après. La table reste chez toi.
- Jetons stables, détecteurs combinés, restauration selon les droits.
- Pseudonymisé reste personnel au sens du RGPD.
- Jamais de secret dans le contexte. Le prompt système n'est pas secret.
- Filtrer à l'entrée et à la sortie.
