# Chuck Norris Fortunes

Collection de citations Chuck Norris pour la commande `fortune`.

## Installation

```bash
# Télécharger et installer
sudo wget -O /usr/share/games/fortunes/ChuckNorrisFact-purifie.txt \
  https://raw.githubusercontent.com/sebman56/chuck-norris-fortunes/main/ChuckNorrisFact-purifie.txt

# Compiler le fichier
sudo strfile /usr/share/games/fortunes/ChuckNorrisFact-purifie.txt

# Utiliser
fortune ChuckNorrisFact-purifie
