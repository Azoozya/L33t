use lazy_static::lazy_static;

lazy_static! {
    // could use is_numeric to recognize numbers but utf8 give access to other kinds of numbers
    static ref LEET: [char;36] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','1','2','3','4','5','6','7','8','9','0'];
    static ref L33T: [char;36] = ['4','8','c','d','3','f','9','h','1','j','k','l','m','n','0','p','q','r','5','7','u','v','w','x','y','2','1','2','3','4','5','6','7','8','9','0'];
    static ref L337: [char;36] = ['@','8','(',')','3','/','9','#','1','j','k','\\','m','n','0','p','q','r','5','7','u','v','w','x','y','2','1','2','3','4','5','6','7','8','9','0'];
    static ref EXCEPT: [char; 6] = [ '@', '(' , ')' , '#' , '\\' , '/'];
}

#[derive(Debug)]
pub enum LEETError {
    NotLEETChar,
    NotL33TChar,
    NotL337Char,
    NotSupportedAlphabet,
}

#[derive(Copy, Clone,PartialEq,Eq)]
pub enum Alphabet {
    Leet,
    L33t,
    L337,
    Other,
}

// For reverse L337 we need to consider some non-alphanumeric chars as "alphanumeric" for this alphabet
fn is_exept(c: char) -> bool {
        for i in 0..EXCEPT.len() {
        if c.eq_ignore_ascii_case(&EXCEPT[i])
        {    return true;       }
    }
    false
}

//
fn is_leet(c: char) -> (bool,usize) {
    for i in 0..LEET.len() {
        if c.eq_ignore_ascii_case(&LEET[i])
        {    return (true,i);       }
    }
    (false,usize::MAX)
}

fn is_l33t(c: char) -> (bool,usize) {
    for i in 0..L33T.len() {
        if c.eq_ignore_ascii_case(&L33T[i])
        {    return (true,i);       }
    }
    (false,usize::MAX)
}

fn is_l337(c: char) -> (bool,usize) {
    for i in 0..L337.len() {
        if c.eq_ignore_ascii_case(&L337[i])
        {    return (true,i);       }
    }
    (false,usize::MAX)
}

//

fn is_char(al: Alphabet, c: char) -> (bool,usize) {
    match al {
        Alphabet::Leet => is_leet(c),
        Alphabet::L33t => is_l33t(c),
        Alphabet::L337 => is_l337(c),
        _ => (false,usize::MAX)
    }
}

//

fn convert_char(alsrc: Alphabet, aldst: Alphabet, c: char) -> Result<char,LEETError> {
    let (success,rank) = is_char(alsrc,c);
    if !success {
        match alsrc {
            Alphabet::Leet => { return Err(LEETError::NotLEETChar); },
            Alphabet::L33t => { return Err(LEETError::NotL33TChar); },
            Alphabet::L337 => { return Err(LEETError::NotL337Char); },
            _ => { return Err(LEETError::NotSupportedAlphabet); },
        };
    }

    match aldst {
        Alphabet::Leet => Ok(LEET[rank]),
        Alphabet::L33t => Ok(L33T[rank]),
        Alphabet::L337 => Ok(L337[rank]),
        _ => Err(LEETError::NotSupportedAlphabet),
    }
}

pub fn convert_string(alsrc: Alphabet, aldst: Alphabet, s: String) -> Result<String,LEETError>{
    let mut result = String::new();
    for c in s.chars(){
        if !c.is_alphanumeric() && (
            (alsrc == Alphabet::Leet)
            || (alsrc == Alphabet::L33t)
            || (alsrc == Alphabet::L337 && !is_exept(c))) {
            result.push(c);
        }
        else {
            result.push(convert_char(alsrc,aldst,c)?);
        }
    }
    Ok(result)
}
