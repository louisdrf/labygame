# Encodage du Radar

La vue _radar_ correspond à la vue du joueur de son environnement (tout autour de lui, devant, derrière, sur les côtés
et en diagonales).

Elle est centrée sur le joueur et représente une zone de taille 3x3 cellules, avec leurs murs:

* 12 horizontaux (ci-dessous représenté par `━`)
* 12 verticaux (ci-dessous représenté par `┃`)
* 9 cellules portant un [`RadarItem`](#les-items-sur-le-radar) (voir section suivante)

```
•━•━•━•
┃ ┃ ┃ ┃
•━•━•━•
┃ ┃H┃ ┃
•━•━•━•
┃ ┃ ┃ ┃
•━•━•━•
```

Cependant, à chaque instant, seuls les cellules visibles sont découvertes; les autres restant 'inconnues'

```
#######
#######
##•-•-•
##|   |
##• • •
|   |
•-• •-•
```

Représentation d'un RadarView. Cette représentation n'est pas contractuelle car c'est uniquement la structure logique
qui compte. Sur la précédente représentation, les `#` représentent des éléments (murs ou cellules) non visibles du fait
de murs bloquant la vision depuis la cellule centrale (où se trouve le joueur).

## Les items sur le radar

Les items identifient:

* les indices (`H`) et la cible (`G`)
* la présence de joueur de la même équipe (`P`), d'une autre équipe (`O`) ou d'un monstre (`M`).

Sur une même cellule, il peut y avoir

* un indice ou (exclusif) la cible
* ou (inclusif) un joueur de la même équipe ou (exclusif) d'une autre équipe ou (exclusif) d'un monstre.

Les items sur le radar sont encodés de manière compacte afin:

1. d'optimiser l'espace de stockage (sur 4 bits seulement)
2. de permettre une identification rapide des éléments importants sur un radar
3. Différencier clairement les entités amies, ennemies et neutres
4. Marquer les objectifs et les indices de manière distincte

C'est sous cette forme (convertie en hexadécimale) que vous recevrez les items du radar.

### Cas spécial

`1111` : Représente une donnée non définie/invalide, pour représenter une cellule non définie dans la vue du radar.

### Cas standard

Pour toutes les autres valeurs, l'encodage se décompose en deux parties :

#### Bits 2-3 : Nature de l'élément

- `00` : Aucune information particulière
- `01` : Indique la présence d'un indice
- `10` : Indique le point d'arrivée

#### Bits 0-1 : Type d'entité

- `00` : Aucune entité
- `01` : Joueur allié
- `10` : Joueur adverse
- `11` : Monstre

### Exemples de combinaisons significatives

- `0000` : Aucune information particulière, aucune entité
- `0001` : Aucune information particulière, entité alliée
- `0010` : Aucune information particulière, entité opposée
- `0011` : Aucune information particulière, entité hostile
- `0100` : Élément d'aide, aucune entité
- `1000` : Objectif, aucune entité
- `1011` : Objectif avec entité hostile

De facto, vous pouvez vous apercevoir qu'il ne peut y avoir plus d'une entité amie,
ennemie ou neutre (monstre) sur une case.

De même, il n'y a jamais d'indice sur la case d'arrivée.

Note : Toute valeur avec les quatre bits à 1 (`1111`) indique une donnée invalide ou non définie.

## L'encodage de la vue Radar

La vue radar est transmise par le réseau sous une forme compressée, non visuelle (contrairemenet au représentation
ci-dessus).

Pour se faire, vous devez:

1. convertir la structure en une suite d'octets dans l'ordre

(en commençant par en haute à droite, puis, ligne par ligne de haut en bas et de gauche à droite):

* Les 12 passages horizontaux (sur 12 * 2 bits = 24 bits = 3 octets)
* Les 12 passages verticaux (sur 12 * 2 bits = 24 bits = 3 octets)
* Les 9 éléments des cellules (sur 9 * 4 bits = 36 bits ~ 5 octets, avec un padding de 0 sur les 4 bits de poids faible)

Les passages sont encodés ainsi:

* 0: pour _Undefined_ (non défini)
* 1: pour _Open_ (passage ouvert)
* 2: pour _Wall_ (passage fermé ou mur)

Les éléments de cellules sont encodés comme décrits [précédemment](#les-items-sur-le-radar).

2. encodage textuel de la séquence d'octets (cf [binary_text_encoding](./binary_text.md))
