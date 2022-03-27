# L33t
Rust library for translate text as/from *custom* leet.

Alphabet:
- Leet , is the natural alphabet
- L33t , is leet using only alphanumeric chars
- L337 , is a custom leet using only transformation 1:1
 
> Tanslate from L33t or L337 to Leet will not preserve numbers.


Errors:

If a character is not recognized (not part of the lazy_static ref you target with Alphabet::) you'll get an LEETError corresponding to the alphabet. 


```
use l33t::convert_string;
use l33t::Alphabet;

fn your_fn() {
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

    match convert_string(Alphabet::L33t,Alphabet::Leet,String::from("0123456789")) {
        Ok(res) => println!("Leet -> Alpha : {}",res),
        Err(e) => println!("{:#?}",e),
    }
}
```
