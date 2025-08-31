use std::io;
use rand::Rng;

fn jeu_devin() {
    let mut rn: rand::rngs::ThreadRng = rand::thread_rng();
    let mut found : bool = false;
    let expected : u8 = rn.gen_range(0..=100);
    let mut input : String = String::new();
    let mut inputparse : u8;

    while !found {
        input.clear();
        println!("veuillez saisir un nombre entre 0 et 100");
        io::stdin()
            .read_line(&mut input)
            .expect("erreur dans l'entrée utilisateur");
        inputparse = input.trim().parse().expect("erreur de parsing");
        if inputparse > expected{
            println!("le nombre que tu as choisi est trop grand");
        }
        else if inputparse < expected {
            println!("le nombre queu as choisi est trop petit");
        }
        else if inputparse == expected{
            println!("Félicitation tu as trouvé le nombre mystère");
            found = true;
        }
    }
}

fn rdm_mdp(size: u8) -> String {
    let mut mdp : String = String::new();
    let mut rn: rand::rngs::ThreadRng = rand::thread_rng();
    let speciaux : Vec<char> = vec![
    '!', '"', '#', '$', '%', '&', '*', '+', ',', '-', '.', '/',
    ':', ';', '<', '=', '>', '?', '@', '\\', '^', '_', '`', '|', '~'];
    let nb : Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let lettre_min : Vec<char> = vec![
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z'];
    let lettre_maj : Vec<char> = vec![
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z'];
    while (mdp.chars().count() as u8) != size {
        let nb_rdm : u8 = rn.gen_range(0..=3);
        match nb_rdm {
            0 => mdp.push(speciaux[rn.gen_range(0..speciaux.len())]),
            1 => mdp.push(nb[rn.gen_range(0..nb.len())]),
            2 => mdp.push(lettre_min[rn.gen_range(0..lettre_min.len())]),
            3 => mdp.push(lettre_maj[rn.gen_range(0..lettre_maj.len())]),
            _ => unreachable!(),
        };
    }
    return mdp;
}

fn main() {
    let size : u8 = 8;
    println!("Hello, world!");
    jeu_devin();
    println!("voici ton mdp généré aléatoirement : {}",rdm_mdp(size));
}
