use std::io;
use std::fs;
use sha2::{Sha256, Digest};

fn calc_hash(caminho: &str) -> io::Result<String> {
    let mut arquivo = fs::File::open(caminho)?;
    let mut hasher = Sha256::new();
    io::copy(&mut arquivo, &mut hasher)?;
    let resultado = hasher.finalize();
    Ok(format!("{:x}", resultado))
}

fn main() {
    let origem = "codes.txt";
    let destino = "backup-codes.txt";

    if fs::metadata(origem).is_err() {
        println!("\x1b[31mErro: O arquivo de origem '{}' não existe.\x1b[0m", origem);
        return;
    }

    if fs::metadata(destino).is_err() {
        println!("Destino não existe. Iniciando primeira cópia...");
        fs::copy(origem, destino).expect("Falha ao copiar arquivos");
    } 
    else {
        match (calc_hash(origem), calc_hash(destino)) {
            (Ok(h1), Ok(h2)) if h1 == h2 => {
                println!("\x1b[32mArquivos já sincronizados. Não há nada a fazer\x1b[0m");
            }
            (Ok(_), Ok(_)) => {
                println!("\x1b[33mMudança detectada. Sincronizando arquivos...\x1b[0m");
                fs::copy(origem, destino).expect("Falha ao copiar arquivo");
                println!("\x1b[32mSincronizado com sucesso!\x1b[0m");
            }
            _ => println!("\x1b[31mErro ao ler arquivos para comparação\x1b[0m"),
        }
    }
}