# Projet Rust: *Sauve qui peut* <br> Architecture des Logiciels - 4<sup>ème</sup> année - ESGI

![team](images/team-with-title.png "Team 'Sauve qui peut'")

Les membres de votre équipe sont projetés dans un labyrinthe inconnu, loin les uns des autres.
Vous n'avez que des talkies-walkies pour communiquer.

Votre objectif est de sortir rapidement du labyrinthe avant que...

## Errata

Rien à signaler pour l'instant

## Changements

* `Wed Nov 13 22:15:28 CET 2024`: `ActionError` remplace `ActionResult`. Un mouvement légal n'est pas suivi d'un message
  de validation. C'est la
  prochaine vue qui est immédiatement envoyée.
* `Wed Nov 13 22:15:28 CET 2024`: Les indices `Hint` sont désormais actifs
* `Sun Nov 17 23:16:30 CET 2024`: Le processus d'enregistrement des équipes et des joueurs est stabilisé.

* Notez que des variantes sont en approche:
    * Pour l'instant, la cible est toujours en bas à droite, cela changera bientôt...
  
    * Pour l'instant, dès qu'un joueur est inscrit il participe à la partie. A terme, il faudra attendre l'inscription
      complète de l'équipe pour que le jeu démarre.

    * Pour l'instant les parties sont non scorées. Un simple log sur le serveur donne le nombre de mouvements effectués
      quand un joueur trouve la sortie.

    * Challenges (contenu assuré, faisant partie du projet)
        * _spoiler_: saurez-vous sauver un de vos coéquipiers en danger (avec de la résolution de labyrinthe au passage)

          inclus l'introduction des types `Challenge`, `ChallengeAnswer`, `ChallengeError`

    * Monster is coming (contenu optionnel, _for fun_)
        * _spoiler_: seul face à un monstre, vous perdrez; unis vous gagnerez

          (inclus une modification du type `Action`)

    * Team fight (contenu optionnel, _for fun_)
        * _spoiler_: seuls les membres d’une même équipe peuvent être sur une même case...

          (inclus une modification du type `Action`)

## Déroulement du jeu en mode solo ou team

### Préparation

1. Le serveur démarre en attente d'équipes pour jouer
2. Chaque équipe se connecte au serveur et s'enregistre avec un nom unique.
3. Le serveur vous renvoie le nombre de joueurs à fournir pour chaque équipe 
   et un code d'accès avec lequel les joueurs pourront s'inscrire.
4. Les membres se connectent avant le timeout avec le code d'accès de l'équipe

### L'évasion

Quand une partie commence, les joueurs sont propulsés dans le labyrinthe sans information sur leurs positions.

5. Le serveur envoie la vue (cf [RadarView](./encodings/RadarView.md)) autour du joueur avec une information sur
   les cases autour de lui, les cases libres, les murs et les éventuels autres items du jeu.
6. Le serveur peut parfois envoyer des indices de type "boussole" (absolue ou relative) indiquant la direction de la
   sortie. Celle-ci est exprimé en degrées.

   Quand elle est relative, c'est par rapport à votre déplacement. 0 degrée, signifiant ainsi " tout droit" et 90 degrée
   signifiant à droite.
   (NB: ce n'est qu'un indice; la présence des murs peut tout à fait vous empêcher de suivre cette direction).

   Le serveur peut aussi vous envoyer en indice la dimension de la grille du labyrinthe (en nombre de cellules en
   largeur et hauteur).

7. Chaque joueur formule l'action qu'il souhaite effectuer. Le plus simple est un déplacement vers une case libre.
8. La cible est identifiée sur le [RadarView](./encodings/RadarView.md) et dès lors que le joueur est dessus, le serveur
   considère qu'il a trouvé la sortie.

La partie dure jusqu'à ce que l'ensemble d'une équipe soit sorti du labyrinthe.

### Variante en cours de partie

* Avec les challenges et les indices, il sera peut-être avantageux de communiquer avec vos talkies-walkies...

## Score

Le score d'une partie est calculée comme la somme des déplacements effectués par l'ensemble des membres d'une équipe
avant de sortie, divisé par le nombre de membres dans l'équipe.

```
Score = nombre_de_mouvements / nb_de_participants
```

## Votre objectif

* Réaliser un client écrit en Rust sans bibliothèque extérieure autres que celles autorisées.

  **C'est la partie principale du projet.**

  Le client *doit* pouvoir être lancé de la manière suivante: `worker [server_address]`

  où
    * `server_address` représente l'adresse du serveur (nom ou IP).
    * le port de connexion est par défaut `8778`
    * le nom de connexion au serveur doit être celui de votre groupe

      (tel que défini dans myges, vous avez le droit d'y mettre un suffixe personnalisé et *inspiré*)

      (vous pouvez ajouter aussi des options complémentaires)

* Réaliser un serveur minimal qui permette de tester un client.

  Un serveur de référence vous est fourni pour tester votre client. Vous pouvez le télécharger en tant que documents
  fournis pour les cours (sur https://myGES.fr). Les présentes instructions contiennent
  sa [documentation](DemoServer.md).

  Vous devrez en particulier tester votre client (en mode offline) sur une grille générée par le serveur de référence.

* Il ne doit pas y avoir de duplication de code entre le client et le serveur.

  Vous définirez un "crate" pour:
    * Le client
    * Le serveur
    * Les éléments communs au client et au serveur
    * Les manipulations de grille ou autres algorithmes de résolutions

## Les modalités de réalisation

* Le projet doit être traité par groupe de 3 ou 4 personnes

* Le code doit remis sous Git (github ou gitlab) **avec** une archive déposée dans MyGES (c'est cette archive qui fait
  foi en cas de litige).

  Le projet Git devra être créé à partir d'un *fork* du projet portant le sujet (et n'oubliez pas de m'en donner l'accès
  en lecture).

* Le code doit être fonctionnel sous Linux, macOS et Windows

* Le code devra être raisonnablement testé (par des tests unitaires et des tests d'intégration)

* Le code devra suivre les règles de codage défini par `rustfmt`

* Le code devra être documenté avec `rustdoc`

* La documentation devra être intégrée au dépôt du code et écrite au format Markdown.

* Les seuls modules (*aka* crates) autorisés ici sont:
    * [`serde`](https://crates.io/crates/serde) et [`serde_json`](https://crates.io/crates/serde_json) pour la
      sérilalisation/désérialisation
    * [`image`](https://crates.io/crates/image) pour l'export d'images

  et éventuellement si besoin (en rien indispensable):
    * `rand`
    * `clap`
    * `tracing`
    * [`pixels`](https://crates.io/crates/pixels), [`egui`](https://github.com/emilk/egui), [
      `druid`](https://github.com/linebender/druid) ou [`piston`](https://github.com/pistondevelopers/piston)[[
      `exemples`](https://github.com/pistondevelopers/piston-examples)]
      si vous envisagez de faire un mode graphique.

  Pour tout autre package, **vous devrez demander un accord préalable**.

Le jour de la soutenance orale, vous serez évalués sur:

* Le respect des consignes
* La fiabilité et le respect du protocole entre les clients et serveur
* Le respect des idiomes Rust (dont la gestion des erreurs)
* L'organisation et la lisibilité du code
* Je veux tous les commits (depuis le premier qui est le clone de ce dépôt) avec l’identité de chacun des contributeurs;
  si vous n’apparaissez pas dans les commits de code, vous serez considérés avec un Malus
* Il y aura une note collective et une note individuelle.
* La doc Markdown doit mettre en évidence
    * Votre organisation du travail en tant qu'équipe
    * Votre démarche d'élaboration des différents composants du projet
    * Les spécificités de votre projet (i.e. ce qui n'est pas déjà dans le sujet)
    * Vos éventuels bonus (parmi la liste présentée ou bien d'autres si validés au préalable par l'enseignant)

      Les bonus ne sont pris en compte uniquement si le client est fonctionnel (fonctionnement raisonnablement
      sans planter dans des situations "normales" de jeu).

  Le niveau minimal fonctionnel du client défini la note de 10/20.
* Vous aurez aussi une modification, un petit développement à faire en live sur votre code pendant la soutenance.

## Bonus possibles:

* Réaliser une interface pour le client et/ou le serveur.

* Ajouter une intégration continue qui permette de tester votre code client et serveur (sous GitHub ou GitLab)

* Utilisation d'un fichier externe pour recharger des configurations intéressantes ou pour sauvegarder la partie
  courante.

* Réduire au maximum (voire à zéro) les éléments suivants

  (ce qui est un élément très qualitatif pour vos codes en Rust en plus d'être un bonus dans le cadre de ce projet)
    * les `unwrap()`, les `expect()`, les `panic!()`
    * les `mut` (variables mutables)
    * les *warnings* de compilation

* Réussir à faire *crasher* le serveur de référence (bonus activé à partir de décembre, dès lors que la version stable
  vous aura été remise)

NB: Pour les *Bonus*, vous avez le droit d'employer des modules (*aka* crates) additionnels après une approbation
explicite de celui-ci (il pourra vous être demandé de justifier ce besoin).

## Le protocole d'échange (**format non stabilisé**)

Tous les messages se passent sur un flux TCP qui doit rester ouvert pendant toute la durée de la partie (et fermer
*proprement* en fin de partie).

Tous les messages sont de la forme:

| Message size                  | JSON message     | other data or message... |
|-------------------------------|------------------|--------------------------|
| (u32 encodé en Little Endian) | (encodé en utf8) |                          |

### Description des messages

Tous ces messages sont transmis sous la forme d'une
sérialisation [JSON](https://fr.wikipedia.org/wiki/JavaScript_Object_Notation).

| Nom du message          | Champs du message                                                                          | Exemple                                                                                                                             | Commentaires                              |
|-------------------------|--------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------|
| `RegisterTeam`          | `name: String`                                                                             | `{"RegisterTeam":{"name":"curious_broccoli"}}`                                                                                      |                                           | 
| `RegisterTeamResult`    | `enum { Ok { expected_players: u8, registration_token: String }, Err(RegistrationError) }` | `{"RegisterTeamResult":{"Ok":{"expected_players":3,"registration_token":"SECRET"}}}`<br>`{"SubscribeResult":{"Err":"InvalidName"}}` |                                           | 
| `SubscribePlayer`       | `name: String, registration_token: String`                                                 | `{"SubscribePlayer":{"name":"flower_power","registration_token":"SECRET"}}`                                                         |                                           | 
| `SubscribePlayerResult` | `enum { Ok, Err(RegistrationError) }`                                                      | `{"SubscribePlayerResult":{"Err":"InvalidName"}}`                                                                                   |                                           | 
| `RadarView`             | `Sring`                                                                                    | `{"RadarView":"sgvSBg8SifDVCMXKiq"}`                                                                                                | Le radar est fourni dans un format encodé |
| `Hint`                  | `enum { RelativeCompass { angle: f32 }, GridSize { columns: u32, rows: u32 }, ... }`       | `{"Hint":{"RelativeCompass":{"angle":12.0}}}`                                                                                       | D'autres indices sont à venir             |
| `Action`                | `enum { MoveTo(RelativeDirection) }`                                                       | `{"Action":{"MoveTo":"Right"}}`                                                                                                     | D'autres actions sont à venir             | 
| `ActionError`           | `enum { CannotPassThroughWall, ... }`                                                      | `{"ActionError":"CannotPassThroughWall"}`                                                                                           | D'autres erreurs sont à venir             | 
| `Challenge`             |                                                                                            |                                                                                                                                     | Encore à définir                          |
| `ChallengeAnswer`       |                                                                                            |                                                                                                                                     | Encore à définir                          |
| `ChallengeResult`       |                                                                                            |                                                                                                                                     | Encore à définir                          |

### Séquencement des messages

![Séquencement des messages](images/Sequence.drawio.svg "Séquencement des messages")

### Les types complémentaires

| Nom du type         | Description du type                                                                 | Commentaires |
|---------------------|-------------------------------------------------------------------------------------|--------------|
| `RegistrationError` | `enum { AlreadyRegistered, InvalidName, InvalidRegistrationToken, TooManyPlayers }` |              |
| `RelativeDirection` | `enum { Front, Right, Back, Left }`                                                 |              |

## Notions abordées

* Réseau / mémoire partagée / multithreading
* Respect d'une API réseau
* Segmentation d'un projet en composants faiblement couplés
* Décomposition et implémentation en structures et traits
* `serde` pour le transfert des données
* Mise en place de tests unitaires et d'intégration

<!-- for PDF export using pandoc
---
title: "Project Rust"
subtitle: "Architecture des logiciels - 4ème année - ESGI"
author: Pascal HAVÉ \<training+esgi@haveneer.com\>
date: 25 octobre 2024
geometry: "left=1cm,right=1cm,top=1cm,bottom=2cm"
output: pdf_document
---
-->
