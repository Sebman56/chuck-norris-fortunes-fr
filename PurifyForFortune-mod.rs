use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Ouvrir le fichier source
    let source_file = File::open("ChuckNorrisFact-Brut.txt")?;
    let mut reader = BufReader::new(source_file);
    
    // Lire le contenu entier
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    
    // Processus de purification
    let purified_content = purify_content(&content);
    
    // Créer le fichier de destination
    let dest_file = File::create("ChuckNorrisFact-purifie.txt")?;
    let mut writer = BufWriter::new(dest_file);
    
    // Écrire le contenu purifié
    writer.write_all(purified_content.as_bytes())?;
    
    println!("Purification terminée ! Le fichier a été créé avec succès.");
    Ok(())
}

fn purify_content(content: &str) -> String {
    let mut result = String::new();
    let mut in_section = false;
    let target = "[+++]";
    
    for line in content.lines() {
        if in_section {
            // Si nous sommes dans une section à supprimer,
            // vérifier si la ligne contient la cible de fin
            if line.contains(target) {
                in_section = false;
                result.push('\n'); // Ajouter un saut de ligne à la fin de la section
            }
            // Ignorer le contenu entre # et [+++]
        } else if line.starts_with('#') {
            // Détecter le début d'une section à supprimer
            in_section = true;
            // Vérifier si la cible de fin est sur la même ligne
            if line.contains(target) {
                in_section = false;
                result.push('\n');
            }
        } else {
            // Ajouter les lignes normales au résultat
            result.push_str(line);
            result.push('\n');
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purify_content() {
        let input = "Première ligne normale\n# Section à supprimer\nContenu à ignorer\n[+++]\nDeuxième ligne normale\n#Section courte[+++]Troisième ligne";
        let expected = "Première ligne normale\n\nDeuxième ligne normale\n\nTroisième ligne\n";
        
        assert_eq!(purify_content(input), expected);
    }
}
