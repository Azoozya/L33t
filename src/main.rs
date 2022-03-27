use l33t::convert_string;
use l33t::Alphabet;

fn main() {
    match convert_string(Alphabet::Leet,Alphabet::L33t,String::from("Hello, world!")) {
        Ok(res) => println!("Alpha -> Leet : {}",res),
        Err(e) => println!("{:#?}",e),
    }

    match convert_string(Alphabet::L33t,Alphabet::L337,String::from("h3ll0, w0rld!")) {
        Ok(res) => println!("Leet -> L337 : {}",res),
        Err(e) => println!("{:#?}",e),
    }

    match convert_string(Alphabet::L337,Alphabet::Leet,String::from("#3\\\\0, w0r\\)!")) {
        Ok(res) => println!("L337 -> Alpha : {}",res),
        Err(e) => println!("{:#?}",e),
    }


    match convert_string(Alphabet::Leet,Alphabet::L33t,String::from("abcdefghijklmnopqrstuvwxyz")) {
        Ok(res) => println!("Alpha -> Leet : {}",res),
        Err(e) => println!("{:#?}",e),
    }

    match convert_string(Alphabet::L33t,Alphabet::Leet,String::from("48cd3f9h1jklmn0pqr57uvwxy2")) {
        Ok(res) => println!("Leet -> Alpha : {}",res),
        Err(e) => println!("{:#?}",e),
    }

    match convert_string(Alphabet::L33t,Alphabet::Leet,String::from("0123456789")) {
        Ok(res) => println!("Leet -> Alpha : {}",res),
        Err(e) => println!("{:#?}",e),
    }

    match convert_string(Alphabet::L337,Alphabet::Leet,String::from("0123456789")) {
        Ok(res) => println!("L337 -> Alpha : {}",res),
        Err(e) => println!("{:#?}",e),
    }

}
