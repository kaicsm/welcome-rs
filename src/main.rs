use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: String,
}

fn main() {
    if verify_exists() {
        login()
    } else {
        register()
    }
}

fn verify_exists() -> bool {
    match fs::read("user.json") {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Registrar usuario
fn register() {
    println!("Bem vindo!");
    print!("Qual e o seu nome: ");
    io::stdout().flush().unwrap();

    let name = {
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Erro ao ler entrada");
        name
    };

    println!("Certo!");
    print!("Qual a sua idade: ");
    io::stdout().flush().unwrap();

    let age = {
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Erro ao ler entrada");
        age
    };

    let user = User {
        name: name.trim().to_string(),
        age: age.trim().to_string(),
    };

    let json_content =
        serde_json::to_string(&user).expect("Erro ao serializar o usuario no conteudo json");

    // Salva as informacoes do usuario na maquina local
    fs::write("user.json", json_content).expect("Erro ao salvar o arquivo json");
}

fn login() {
    let json_content = fs::read_to_string("user.json").expect("Erro ao ler arquivo json");
    let user: User = serde_json::from_str(&json_content).expect("Erro ao deserializar o json");

    println!("Bem vindo {}!", user.name);
}
