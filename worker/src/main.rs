use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let server_address = &args[1];
            let port = &args[2];
            println!("Connexion à l'adresse : {}:{}", server_address, port);
        },
        2 => {
            let server_address = &args[1];
            println!("Connexion à l'adresse : {}", server_address);
        }
        1 => {
            eprintln!("Erreur : aucune adresse spécifiée.");
            eprintln!("Usage: worker [server_address]");
            std::process::exit(1);
        }
        _ => {
            eprintln!("Erreur : trop d'arguments spécifiés.");
            eprintln!("Usage: worker [server_address]");
            std::process::exit(1);
        }
    }
}
