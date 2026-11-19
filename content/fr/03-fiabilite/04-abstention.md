---
id: "03-04"
titre: "Abstention"
sous_rubriques:
  - id: seuils
    titre: "Seuils de confiance"
  - id: je-ne-sais-pas
    titre: "Dire « je ne sais pas »"
sources:
  - titre: "OpenAI (Kalai, Nachum, Vempala et al.) — Why language models hallucinate, 5 septembre 2025"
    url: "https://openai.com/index/why-language-models-hallucinate/"
  - titre: "Anthropic — Reduce hallucinations (documentation)"
    url: "https://platform.claude.com/docs/en/test-and-evaluate/strengthen-guardrails/reduce-hallucinations"
qcm:
  - id: seuils-1
    sous_rubrique: seuils
    question: "Quels signaux peuvent décider d'une abstention ?"
    choix:
      - "Le score du meilleur document, le score du reranker, le verdict du vérificateur"
      - "La longueur de la question"
      - "L'heure de la requête"
      - "La langue de l'utilisateur"
    reponse: 0
    pourquoi: >-
      Un meilleur document faible : le corpus n'a peut-être pas la réponse.
      Un reranker qui juge tout peu pertinent : même conclusion.
      Un vérificateur qui trouve des affirmations non soutenues : la réponse est fragile.
      On combine ces signaux pour décider.

  - id: seuils-2
    sous_rubrique: seuils
    question: "Le modèle écrit « je suis sûr à 95 % ». Peut-on s'en servir comme seuil ?"
    choix:
      - "Oui, c'est une probabilité exacte"
      - "Non, les modèles ne parlent pas de confiance"
      - "Oui, si la température est à 0"
      - "Avec prudence : la confiance qu'un modèle exprime en mots est souvent mal calibrée"
    reponse: 3
    pourquoi: >-
      « 95 % » est du texte généré, comme le reste.
      Il ne vient pas d'un calcul de fiabilité.
      Préfère des signaux mesurables : scores de recherche, vérification des sources.
      Et vérifie leur calibrage sur ton jeu de test.

  - id: seuils-3
    sous_rubrique: seuils
    question: "Comment fixer le seuil d'abstention ?"
    choix:
      - "À 0,5, par défaut"
      - "Sur un jeu de test avec des questions sans réponse, en mesurant erreurs et abstentions"
      - "Au hasard"
      - "Le plus haut possible"
    reponse: 1
    pourquoi: >-
      Seuil trop bas : le système répond à tout, et se trompe.
      Seuil trop haut : il s'abstient tout le temps, et ne sert à rien.
      Les questions sans réponse du jeu de test montrent où placer la limite.
      Le bon seuil dépend du coût d'une erreur dans ton métier.

  - id: seuils-4
    sous_rubrique: seuils
    question: "Un assistant médical et un assistant de recettes de cuisine. Lequel doit s'abstenir le plus facilement ?"
    choix:
      - "Le culinaire"
      - "Les deux pareil"
      - "Le médical : une erreur y coûte bien plus cher"
      - "Aucun des deux"
    reponse: 2
    pourquoi: >-
      Le seuil traduit un coût.
      Une mauvaise recette gâche un dîner.
      Une mauvaise information médicale peut nuire à quelqu'un.
      Plus l'erreur est grave, plus on accepte de s'abstenir.

  - id: je-ne-sais-pas-1
    sous_rubrique: je-ne-sais-pas
    question: "Selon OpenAI (2025), pourquoi les modèles devinent-ils au lieu de dire « je ne sais pas » ?"
    choix:
      - "Ils ne connaissent pas cette phrase"
      - "Les utilisateurs l'interdisent"
      - "Les évaluations notent la bonne réponse et donnent zéro à l'abstention : deviner rapporte plus"
      - "C'est trop long à écrire"
    reponse: 2
    pourquoi: >-
      C'est comme un examen sans points négatifs.
      Deviner une date d'anniversaire : une chance sur 365 de marquer.
      Dire « je ne sais pas » : zéro point, toujours.
      Un modèle optimisé pour ces scores apprend à deviner.

  - id: je-ne-sais-pas-2
    sous_rubrique: je-ne-sais-pas
    question: "Dans le même article, un modèle s'abstient sur 52 % des questions, l'autre sur 1 %. Ce qu'on observe ?"
    choix:
      - "Celui qui s'abstient à 1 % a une précision un peu meilleure, mais beaucoup plus d'erreurs"
      - "Celui qui s'abstient est inutile"
      - "Les deux font autant d'erreurs"
      - "L'abstention n'a aucun effet"
    reponse: 0
    pourquoi: >-
      Le modèle qui répond à tout gagne quelques bonnes réponses.
      Mais il accumule les réponses fausses.
      Un classement par précision seule le met en tête.
      Un classement qui compte les erreurs dit l'inverse.

  - id: je-ne-sais-pas-3
    sous_rubrique: je-ne-sais-pas
    question: "La technique la plus simple, selon Anthropic, pour réduire les inventions ?"
    choix:
      - "Augmenter la température"
      - "Ajouter plus de documents"
      - "Interdire les réponses courtes"
      - "Autoriser explicitement le modèle à dire qu'il ne sait pas"
    reponse: 3
    pourquoi: >-
      Sans permission, le modèle tend à répondre quoi qu'il arrive.
      Une consigne claire lui donne une sortie honnête.
      Par exemple : « Si les documents ne contiennent pas la réponse, dis-le. »
      Ça ne coûte rien et ça réduit nettement les fausses informations.

  - id: je-ne-sais-pas-4
    sous_rubrique: je-ne-sais-pas
    question: "Une bonne abstention, c'est…"
    choix:
      - "« Je ne sais pas. » et rien d'autre"
      - "Dire ce qui manque, ce qui est sûr, et proposer une suite"
      - "Inventer une réponse prudente"
      - "Renvoyer une erreur technique"
    reponse: 1
    pourquoi: >-
      « Je ne sais pas » seul laisse l'utilisateur sans issue.
      Mieux : « Les documents couvrent les vols intérieurs, pas les vols internationaux. »
      Puis une suite : un lien, un humain, une question de précision.
      L'abstention devient une réponse utile.
---

## Seuils de confiance {#seuils}

Parfois, la bonne réponse est de ne pas répondre.

Mais il faut le décider sur des **signaux**, pas sur une impression.

| Signal | Ce qu'il indique |
|---|---|
| Score du meilleur document | Le corpus contient-il la réponse ? |
| Score du reranker | Les passages répondent-ils vraiment ? |
| Verdict du vérificateur | La réponse est-elle soutenue ? |
| Cohérence entre plusieurs réponses | Le modèle est-il stable ? (rubrique suivante) |

:::pourquoi
Aucun signal n'est parfait seul.
Ensemble, ils disent si la réponse repose sur du solide.
:::

La confiance exprimée en mots (« sûr à 95 % ») est souvent mal calibrée.

:::pourquoi
C'est du texte généré, pas une mesure.
Préfère des scores calculés, et vérifie-les sur ton jeu de test.
:::

Le seuil se règle sur des questions **sans réponse**, et selon le coût d'une erreur.

:::pourquoi
Trop bas : on répond à tout et on se trompe.
Trop haut : on ne répond plus à rien.
En médecine, on accepte plus d'abstentions qu'en cuisine.
:::

```mermaid
flowchart LR
  Q["Question"] --> R["Recherche + reranking"]
  R -->|"scores faibles"| S["Abstention utile"]
  R -->|"scores suffisants"| L["LLM : réponse"]
  L --> V["Vérificateur"]
  V -->|"affirmations non soutenues"| S
  V -->|"soutenue"| F["Réponse avec citations"]
```

## Dire « je ne sais pas » {#je-ne-sais-pas}

Septembre 2025. OpenAI publie *Why language models hallucinate*.

Sa thèse : on entraîne et on note les modèles comme des élèves à un examen sans points négatifs.

:::pourquoi
Deviner une date d'anniversaire : une chance sur 365 de marquer.
« Je ne sais pas » : zéro, toujours.
Un modèle optimisé pour ces scores apprend à deviner.
:::

L'article compare deux modèles. L'un s'abstient sur 52 % des questions. L'autre sur 1 %. Le second a une précision un peu meilleure. Mais beaucoup plus d'erreurs.

:::pourquoi
Répondre à tout gagne quelques bonnes réponses.
Et accumule les fausses.
Ne mesure jamais la précision sans compter les erreurs.
:::

Autorise explicitement l'abstention dans le prompt.

:::pourquoi
Anthropic la présente comme la technique de base contre les inventions.
Sans permission, le modèle cherche à répondre coûte que coûte.
:::

Une bonne abstention aide quand même :

- ce qui est sûr, avec sa source ;
- ce qui manque ;
- une suite : un lien, un humain, une question de précision.

:::pourquoi
Le chatbot d'Air Canada avait la page officielle sous la main.
Il pouvait dire : « Je ne suis pas sûr. Voici la page officielle sur les tarifs de deuil. »
Une réponse moins séduisante. Mais pas une règle inventée.
:::

## À retenir

- On s'abstient sur des signaux mesurés, pas sur une impression.
- Le seuil se règle sur des questions sans réponse, selon le coût d'une erreur.
- Les évaluations qui ignorent les erreurs poussent à deviner.
- Autorise le « je ne sais pas », et rends-le utile.
