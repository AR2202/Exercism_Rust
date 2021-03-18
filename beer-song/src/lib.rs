pub fn verse(n: u32) -> String {
    let standard_verse = String::from("num_old bottles of beer on the wall, num_old bottles of beer.\nTake one down and pass it around, num_new bottles of beer on the wall.\n");
    let mut new_verse: String = standard_verse.replace("num_old", &n.to_string());
    if n > 0 {
        new_verse = new_verse
            .replace("num_new", &(n - 1).to_string())
            .replace("1 bottles", "1 bottle");
    } else {
        new_verse = new_verse.replace(
            "Take one down and pass it around, num_new",
            "Go to the store and buy some more, 99",
        );
    };
    if n == 1 {
        new_verse = new_verse.replace("Take one", "Take it");
    }
    new_verse = uppercase_first(&new_verse.replace("0", "no more"));
    new_verse
}

pub fn sing(start: u32, end: u32) -> String {
    let mut n = start;
    let mut song = String::new();
    while n > end {
        song.push_str(&verse(n));
        song.push('\n');
        n -= 1;
    }
    song.push_str(&verse(n));
    song
}

fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
