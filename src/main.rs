use std::io::{self, Write};
use rand::prelude::*;
use std::process::*;
// use std::collections::HashMap;

fn main() {
    let emoji_warning = char::from_u32(0x2757).unwrap();
    let emoji_question = char::from_u32(0x2753).unwrap();
    let emoji_lose = char::from_u32(0x274C).unwrap();
    let emoji_win = char::from_u32(0x2728).unwrap();
    let seta_e = ">".repeat(25);
    let seta_d = "<".repeat(25);

    loop {
        clear_screen();

        println!("{} Master Mind! {}\n", seta_e, seta_d);

        let codigo = generate_code();

        println!("{:?}", codigo);

        let mut guesses_number = get_guesses_number();

        clear_screen();

        loop {
            let mut pos_correta = 0;
            let mut num_existente = 0;

            println!("{} Master Mind! {}\n", seta_e, seta_d);
            
            let mut buf = String::new();
            
            print!("[{}] Chance(s) sobrando...\n\n[{}] Digite seu palpite: \n>>> ", guesses_number, emoji_question);
            std::io::stdout().flush().unwrap();
            
            std::io::stdin().read_line(&mut buf).unwrap();

            let buf: Vec<char> = buf.chars().filter(|&c| !c.is_whitespace()).collect();

            let mut guess_is_valid: bool = true;
            for i in 0..buf.len() { 
                let ver = buf[i].to_string().parse::<i32>();
                if ver.is_err() {
                    guess_is_valid = false;
                    break;
                } 
            }
            if !guess_is_valid {
                println!("\n[{}] Digite apenas valores válidos!\n\n", emoji_warning);
                continue;
            }

            if buf.len() != codigo.len() {
                println!("\n[{}] Digite a quantidade certa de valores!\n\n", emoji_warning);
                continue;
            }

            let stringed_vec_codigo: Vec<String> = codigo.iter().map(|&num| num.to_string()).collect();
            let stringed_vec_guess: Vec<String> = buf.iter().map(|&num| num.to_string()).collect();
            
            if stringed_vec_guess == stringed_vec_codigo {
                let seta_e = ">".repeat(24);
                let seta_d = "<".repeat(24);
                println!("{} Você acertou!! {}", seta_e, seta_d);
                std::io::stdin().read_line(&mut "".to_string()).unwrap();
                break;
            }
            
            guesses_number -= 1;
            if guesses_number == 0 {
                let seta_e = ">".repeat(26);
                let seta_d = "<".repeat(26);
                println!("[{}] O valor era => {:?}\n\n", emoji_lose, stringed_vec_codigo.join(""));
                println!("Pressione ENTER para continuar...");
                println!("{} Game Over! {}", seta_e, seta_d);
                std::io::stdin().read_line(&mut "".to_string()).unwrap();
                break;
            }
            
            for i in 0..buf.len() {
                let ver = buf[i].to_string().parse::<i32>().unwrap();

                if ver == codigo[i] {
                    pos_correta += 1;
                }

                if codigo.contains(&ver) {
                    num_existente += 1;
                }
            }
            println!("\n[{}] {} número(s) correto(s), [{}{}] {} número(s) na(s) posição(ões) correta(s)\n\n", emoji_win, num_existente, emoji_win, emoji_win, pos_correta);
        }
    }    
}
    
#[cfg(target_os = "linux")]
fn clear_screen() {
    Command::new("sh").args(["-c", "clear"]).spawn().unwrap().wait().unwrap();
}

#[cfg(target_os = "windows")]
fn clear_screen() {
    Command::new("cmd").args(["/C", "cls"]).spawn().unwrap().wait().unwrap();
}

fn generate_code() -> Vec<i32> {
    let lista = [1, 2, 3, 4, 5, 6];
    let mut rng = thread_rng();

    let mut codigo = Vec::with_capacity(4);

    while codigo.len() < 4 {
        let num = *lista.choose(&mut rng).unwrap();
        if !codigo.contains(&num) {
            codigo.push(num);
        }
    }

    codigo
}

fn get_guesses_number() -> i32 {
    let emoji_warning = char::from_u32(0x2757).unwrap();
    loop {
        print!("Número de palpites: \n>>> ");
        io::stdout().flush().unwrap();
        
        let mut palpites = String::new();
        io::stdin().read_line(&mut palpites).unwrap();
        
        let num = palpites.trim().parse::<i32>();
        if num.is_err() {
            println!("\n[{}] Digite apenas valores válidos\n\n", emoji_warning);
            continue;
        }

        let num = num.unwrap();
        if num <= 0 {
            println!("\n[{}] Digite um número maior que zero!\n\n", emoji_warning);
            continue;
        }
        return num;
    }
}


        // let mut code_array: Vec<String> = vec![];
        // for _ in 0..299 {
        //     let code = generate_code();
        //     code_array.push(code.iter().map(|&num| num.to_string()).collect());
        // }

        // let mut code_counts: HashMap<&String, u32> = HashMap::new();

        // for code in &code_array {
        //     let count = code_counts.entry(code).or_insert(0);
        //     *count += 1;
        // }

        // let mut sorted_code_counts: Vec<(&&String, &u32)> = code_counts.iter().collect();
        // sorted_code_counts.sort_by(|a, b| a.0.cmp(b.0));

        // for (code, count) in &sorted_code_counts {
        //     println!("Código {}: ocorre {} vez(es)", code, count);
        // }

                // let mut code_array: Vec<String> = vec![];
        // for _ in 0..299 {
        //     let code = generate_code();
        //     code_array.push(code.iter().map(|&num| num.to_string()).collect());
        // }

        // let mut code_counts: HashMap<&String, u32> = HashMap::new();

        // for code in &code_array {
        //     let count = code_counts.entry(code).or_insert(0);
        //     *count += 1;
        // }

        // let mut sorted_code_counts: Vec<(&&String, &u32)> = code_counts.iter().collect();
        // sorted_code_counts.sort_by(|a, b| a.0.cmp(b.0));

        // for (code, count) in &sorted_code_counts {
        //     println!("Código {}: ocorre {} vez(es)", code, count);
        // }