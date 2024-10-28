# Utilisation du serveur de référence

Une fois téléchargé et dézippé, vous pouvez le lancer depuis la ligne de commandes (sur certains OS, il faudra peut-être
ajuster les permissions d'exécution).

# Les options en ligne de commandes

Une aide en ligne de commande est disponible:

```bash
$ ./server --help
```

```
Usage: server [OPTIONS]

Options:
      --port <PORT>
          Port to listen to

          [default: 8778]

      --host-address <HOST_ADDRESS>
          Address allowed to connect to.

          Should be different from localhost to allow external connections

          [default: localhost]

  -v, --verbose
          Display detail

          log will show code path

      --debug
          Log additional debug info

      --trace
          Log additional details

      --screen <SCREEN>
          Initial screen size (width,height) at startup

          [default: 800,800]

      --maze <MAZE>
          Maze size (width,height)

          [default: 50,50]

      --seed <SEED>
          Random generator seed

      --generator <GENERATOR>
          Random generator seed

          [default: recursive-backtracking]
          [possible values: recursive-backtracking, prims, ellers, growing-tree]

      --refresh-rate <REFRESH_RATE>
          [default: 1]

      --output-maze <OUTPUT_MAZE>
          Generate grid and export it

  -V, --version
          Print version and exit

  -h, --help
          Print help (see a summary with '-h')
```

Lors de la mise au point, pour voir quelques détails sur la communication worker<->server, l'option `--debug` est
recommandée.

Par défaut, l'interface réseau d'écoute est celle associée à `localhost`. Ainsi, si vous souhaitez l'utiliser sur un
réseau (local par exemple), il faudra lui préciser l'interface d'écoute sous la forme:

```bash
$ ./server --host-address=192.168.1.99
```

(l'adresse n'est qu'un exemple)

## Les commandes

- <kbd>P</kbd>: Add a local player

- <kbd>escape</kbd>: Quit

## Environnements testés:

* Windows: pas besoin d'installation complémentaire; doit fonctionner *out of the box*.

* macOS: pas besoin d'installation complémentaire; doit fonctionner *out of the box*.

* Linux: si ce n'est pas déjà le cas, vous aurez besoin de driver OpenGL.

  D'expérience, sur Ubuntu, j'ai eu à installer les packages suivants:

  ```bash
  apt install libegl1 libegl1-mesa
  ```