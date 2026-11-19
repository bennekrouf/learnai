---
etape: 2
titre: "Retrouver l'information"
accroche: "Google conseille de mettre de la colle sur la pizza. Le modèle n'a rien inventé."
sources:
  - titre: "Gizmodo — Google explains why it suggested adding glue to pizza, mai 2024"
    url: "https://gizmodo.com/google-explains-why-it-suggested-adding-glue-to-pizza-1851511786"
  - titre: "MIT Technology Review — Why are Google's AI Overviews results so bad?, 31 mai 2024"
    url: "https://www.technologyreview.com/2024/05/31/1093019/why-are-googles-ai-overviews-results-so-bad/"
  - titre: "Engadget — Google is putting more restrictions on AI Overviews after it told people to put glue on pizza"
    url: "https://www.engadget.com/google-is-putting-more-restrictions-on-ai-overviews-after-it-told-people-to-put-glue-on-pizza-011316780.html"
---

## De la colle sur la pizza

Mai 2024. Google lance AI Overviews aux États-Unis.

En haut des résultats, une IA résume le web pour toi.

Quelques jours plus tard, un utilisateur demande : comment empêcher le fromage de glisser de la pizza ?

La réponse : ajouter de la colle non toxique à la sauce.

Un autre demande : combien de cailloux faut-il manger ?

La réponse : au moins un petit caillou par jour.

Les captures d'écran font le tour du monde.

## Le modèle n'a rien inventé

La colle vient d'un vieux commentaire Reddit. Une blague, signée d'un pseudo grossier.

Les cailloux viennent de *The Onion*, un site satirique. Titre : « Les géologues recommandent de manger au moins un petit caillou par jour ».

Le modèle a résumé fidèlement ce qu'on lui a donné.

Le problème était **en amont** : dans ce qu'on lui a donné.

:::pourquoi
Un système RAG a deux étages : retrouver, puis rédiger.
Le second ne peut pas être meilleur que le premier.
Une blague bien retrouvée devient une réponse bien rédigée.
:::

## L'explication de Google

Liz Reid, responsable de la recherche chez Google, publie une explication.

Pour des questions rares, il existe peu de bonnes sources. Elle parle de « vide de données ».

Dans ce vide, la satire et les blagues de forum remontent.

Google ajoute des filtres : moins de satire, moins de contenus de forum pour ces questions.

:::pourquoi
Filtrer sur la nature de la source, c'est filtrer sur des métadonnées.
Le texte d'une blague ressemble au texte d'un conseil.
Seule l'information « vient d'un site satirique » permet de les distinguer.
:::

## La leçon

Le « R » de RAG fait la moitié du travail. Souvent plus.

Retrouver, ce n'est pas trouver un texte proche. C'est trouver le **bon** texte.

## Dans ce chapitre

- **Préparer le corpus** : découper, décrire, indexer.
- **Recherche** : par mots-clés, par sens, et les deux ensemble.
- **Reranking** : un second tri, plus lent et plus fin.
- **Mesurer la recherche** : savoir si elle trouve vraiment.
- **GraphRAG** : relier les informations, et savoir quand ça ne vaut pas le coût.
