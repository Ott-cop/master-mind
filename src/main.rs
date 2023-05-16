use std::io::{Write};
use rand::prelude::*;
use std::process::*;

#[cfg(target_os = "linux")]
fn clear_screen() {
    Command::new("sh").args(["-c", "clear"]).spawn().unwrap().wait().unwrap();
}

#[cfg(target_os = "windows")]
fn clear_screen() {
    Command::new("cmd").args(["/C", "cls"]).spawn().unwrap().wait().unwrap();
}


fn code_gen() -> Vec<i32> {
    let lista = vec![1, 2, 3, 4, 5, 6];
    let mut codigo = vec![1; 4];
    let mut rng = thread_rng();
    loop {
        codigo[0] = *lista.choose(&mut rng).unwrap();
        codigo[1] = *lista.choose(&mut rng).unwrap();
        codigo[2] = *lista.choose(&mut rng).unwrap();
        codigo[3] = *lista.choose(&mut rng).unwrap();

        if codigo[0] != codigo[1] && 
                                    codigo[0] != codigo[2] &&
                                    codigo[0] != codigo[3] &&
                                    codigo[1] != codigo[0] && 
                                    codigo[1] != codigo[2] &&
                                    codigo[1] != codigo[3] &&
                                    codigo[2] != codigo[0] && 
                                    codigo[2] != codigo[1] &&
                                    codigo[2] != codigo[3] &&
                                    codigo[3] != codigo[0] && 
                                    codigo[3] != codigo[1] &&
                                    codigo[3] != codigo[2] {                      
            break;
        }
    }
    codigo
}

fn main() {
    let emoji_warning = char::from_u32(0x2757).unwrap();
    let emoji_question = char::from_u32(0x2753).unwrap();
    let emoji_lose = char::from_u32(0x274C).unwrap();
    let emoji_win = char::from_u32(0x2728).unwrap();

    loop {
        clear_screen();

        let seta_e = ">".repeat(25);
        let seta_d = "<".repeat(25);
        println!("{} Master Mind! {}\n", seta_e, seta_d);

        let codigo = code_gen();

        println!("{:?}", codigo);

        let mut num_palpites = loop {
            let mut palpites = String::new();
            print!("Número de palpites: \n>>> ");
            std::io::stdout().flush().unwrap();
            
            std::io::stdin().read_line(&mut palpites).unwrap();
            let palpites = palpites.trim().parse::<i32>();
            if palpites.is_err() {
                println!("\n[{}] Digite apenas valores válidos\n\n", emoji_warning);
            }
            else {
                let palpites = palpites.unwrap();
                if palpites == 0 {
                    println!("\n[{}] Digite um número maior!\n\n", emoji_warning);
                }
                else {
                    break palpites;
                }
            }
            
        };
        clear_screen();

        'master_mind: loop {
            

            println!("{} Master Mind! {}\n", seta_e, seta_d);

            let mut pos_correta = 0;
            let mut num_existente = 0;
            
            let mut buf = String::new();
            
            print!("[{}] Chance(s) sobrando...\n\n[{}] Digite seu palpite: \n>>> ", num_palpites, emoji_question);
            std::io::stdout().flush().unwrap();
            
            std::io::stdin().read_line(&mut buf).unwrap();

            let buf: Vec<&str> = buf.split(' ').collect();

            let mut buf_int: Vec<i32> = Vec::new();

            if buf.len() == 4 {
                'ver: for i in 0..buf.len() {
                    let ver = buf[i].trim().parse::<i32>();

                    if ver.is_err() {
                        println!("\n[{}] Digite apenas valores válidos!\n\n", emoji_warning);

                        break 'ver;
                    } 
                    else {
                        let ver = ver.unwrap();

                        buf_int.push(ver);

                        if ver == codigo[i] {
                            pos_correta += 1;
                            
                        }
                        
                        if buf_int.len() == 4 {
                            num_palpites = num_palpites - 1;
                            
                            if buf_int[0] == codigo[0] || 
                                                            buf_int[0] == codigo[1] ||
                                                            buf_int[0] == codigo[2] ||
                                                            buf_int[0] == codigo[3] {
                                num_existente += 1;
                            }

                            if buf_int[1] == codigo[0] || 
                                                            buf_int[1] == codigo[1] ||
                                                            buf_int[1] == codigo[2] ||
                                                            buf_int[1] == codigo[3] {
                                num_existente += 1;
                            }

                            if buf_int[2] == codigo[0] || 
                                                            buf_int[2] == codigo[1] ||
                                                            buf_int[2] == codigo[2] ||
                                                            buf_int[2] == codigo[3] {
                                num_existente += 1;
                            }

                            if buf_int[3] == codigo[0] || 
                                                            buf_int[3] == codigo[1] ||
                                                            buf_int[3] == codigo[2] ||
                                                            buf_int[3] == codigo[3] {
                                num_existente += 1;
                            }

                            println!("\n[{}] {} número(s) correto(s), [{}{}] {} número(s) na(s) posição(ões) correta(s)\n\n", emoji_win, num_existente, emoji_win, emoji_win, pos_correta);

                            if buf_int == codigo {
                                let seta_e = ">".repeat(24);
                                let seta_d = "<".repeat(24);
                                println!("{} Você acertou!! {}", seta_e, seta_d);
                                std::io::stdin().read_line(&mut "".to_string()).unwrap();
                                
                                break 'master_mind;
                            }

                            if num_palpites == 0 {
                                let seta_e = ">".repeat(26);
                                let seta_d = "<".repeat(26);
                                
                                println!("[{}] O valor era => {:?}\n\n", emoji_lose, codigo);
                                println!("Pressione ENTER para continuar...");
                                println!("{} Game Over! {}", seta_e, seta_d);
                                
                                std::io::stdin().read_line(&mut "".to_string()).unwrap();
                              
                                break 'master_mind;
                            }
                        }
                    }
                }
            }
            else {
                println!("\n[{}] Digite a quantidade certa de valores!\n\n", emoji_warning);
            }  
        }
    }    
}
    
    
