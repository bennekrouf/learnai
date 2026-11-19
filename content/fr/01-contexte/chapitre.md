---
etape: 1
titre: "Maîtriser le contexte"
accroche: "Une phrase ajoutée au prompt fait passer un modèle de 27 % à 98 %."
sources:
  - titre: "Anthropic — Long context prompting for Claude 2.1, 6 décembre 2023"
    url: "https://claude.com/blog/claude-2-1-prompting"
  - titre: "Greg Kamradt — LLMTest_NeedleInAHaystack"
    url: "https://github.com/gkamradt/LLMTest_NeedleInAHaystack"
  - titre: "Arize — The Needle In a Haystack Test (tentative de reproduction)"
    url: "https://arize.com/blog/the-needle-in-a-haystack-test-evaluating-the-performance-of-llm-rag-systems/"
---

## Une aiguille dans une botte de foin

Novembre 2023. Anthropic sort Claude 2.1.

Sa fenêtre de contexte : 200 000 tokens. Un livre entier.

Un développeur, Greg Kamradt, veut savoir si le modèle lit vraiment tout.

Il prend des dizaines d'essais de Paul Graham sur les startups.

Quelque part au milieu, il cache une phrase sans rapport :

« La meilleure chose à faire à San Francisco, c'est manger un sandwich et s'asseoir à Dolores Park par une journée ensoleillée. »

Puis il demande au modèle : quelle est la meilleure chose à faire à San Francisco ?

Souvent, le modèle ne la trouve pas.

## Une phrase change tout

Décembre 2023. Anthropic répond.

Le modèle n'est pas aveugle. Il est méfiant.

Une phrase sur un sandwich, au milieu d'essais sur les startups, semble hors sujet.

Il a appris à ne pas répondre sur la foi d'un passage douteux.

La correction tient en une phrase. On l'écrit au début de la réponse du modèle :

« Voici la phrase la plus pertinente du contexte : »

Score avant : 27 %. Score après : 98 %.

:::pourquoi
La phrase oblige le modèle à chercher d'abord, puis à répondre.
Il ne décide plus seul si le passage est fiable.
Une équipe d'Arize n'a pas retrouvé 98 %. Mais elle a vu les échecs passer de 165 à 74.
L'effet est réel, même si le chiffre exact varie.
:::

## La leçon

Une grande fenêtre ne garantit pas que tout est lu de la même façon.

**Ce que tu mets** dans le contexte compte.

**Où tu le mets** compte.

**Comment tu l'annonces** compte.

Assembler le contexte, c'est un travail d'ingénieur. Pas un copier-coller.

## Dans ce chapitre

- **Budget de tokens** : répartir la fenêtre, et garder de la place pour répondre.
- **Assemblage par priorité** : quoi garder, quoi couper, où placer chaque morceau.
- **Mémoire à 3 niveaux** : la tâche en cours, ce qui s'est passé, ce qu'on sait.
- **Cycle de vie de la mémoire** : ce qui mérite d'être retenu, et quand oublier.
