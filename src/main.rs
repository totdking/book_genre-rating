use std::io::{self,Write};
#[derive(Debug)]
pub enum Genre{
    Scifi,
    Romance,
    Horror,
    Action,
}
#[derive(Debug)]
struct Book{
    rating: u32,
    genre: Genre,
}
fn book_rating()-> Result<u32, String>{
    //LET A RANGE OF RATING FROM 1 TO 5 
    // let range: u32;
    loop {
        println!("What rating range of book are you looking for?");
        io::stdout().flush().unwrap();
        let mut rating = String::new();
        match io::stdin().read_line(&mut rating) {
            Ok(_) => {
               match rating.trim().parse::<u32>() {
                Ok(parsed_rating) =>{
                    if parsed_rating >= 1 && parsed_rating <= 5 {
                        break Ok(parsed_rating);
                    } else {
                        println!("Please enter a valid rating between 1 and 5.");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
               }
            }
            Err(_) =>{ println!("could not read line");
                        continue;
                    }
        }
    }
}

fn book_genre()-> Result<Genre,String>{
    loop{
        println!("What genre are you looking for?");
        println!("Scifi");
        println!("Romance");
        println!("Horror");
        println!("Action");
        io::stdout().flush().unwrap();
        let mut genre = String::new();
        match io::stdin().read_line( &mut genre) {
            Ok(_) =>{
                match genre.trim().to_lowercase().as_str(){
                    "scifi" => return Ok(Genre::Scifi),
                    "romance" => return Ok(Genre::Romance),
                    "horror" => return Ok(Genre::Horror),
                    "action" => return Ok(Genre::Action),
                    _ => println!("Invalid genre"),
                }
                
            },
            Err(_) => {
                println!("Could not read line");
                continue;
            }
        }
    }
}
fn main(){
    let rating =  match book_rating(){
        Ok(rating) =>{ rating},
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let genre =  match book_genre(){
        Ok(genre) =>{ genre},
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("Rating: {:?}, Genre: {:?}", rating, genre);
}