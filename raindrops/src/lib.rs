pub fn raindrops(n: u32) -> String {
    let raindropstring = pling_string(n) + &plang_string(n) + &plong_string(n);
    if raindropstring == *"" {
        n.to_string()
    } else {
        raindropstring
    }
}

fn pling_string(n: u32) -> String {
    if n % 3 == 0 {
        String::from("Pling")
    } else {
        String::from("")
    }
}

fn plang_string(n: u32) -> String {
    if n % 5 == 0 {
        String::from("Plang")
    } else {
        String::from("")
    }
}

fn plong_string(n: u32) -> String {
    if n % 7 == 0 {
        String::from("Plong")
    } else {
        String::from("")
    }
}
