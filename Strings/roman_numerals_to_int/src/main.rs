fn main() {
    let in_1: String = "III".to_owned();
    let in_2: String = "LVIII".to_owned();
    let in_3: String = "MCMXCIV".to_owned();

    let i_1 = parse(in_1);
    let i_2 = parse(in_2);
    let i_3 = parse(in_3);

    println!("{}", i_1);
    println!("{}", i_2);
    println!("{}", i_3);
}

fn iterate(s: String) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();

    for c in s.split("") {
        if c != "" { v.push(c.chars().nth(0).unwrap()); }
    }

    v
}

fn parse(s: String) -> i32 {
    let mut i: i32 = 0;
    let mut u: usize = 0;
    let v = iterate(s);

    while u < v.len() {
        match v[u] {
            'I' => {
                        if u + 1 == v.len() {
                            i += 1;
                            break;
                        }
                        if v[u + 1] == 'V' {
                            i += 4;
                            u += 2;
                        } else if v[u + 1] == 'X' {
                            i += 9;
                            u += 2;
                        } else {
                            i += 1;
                            u += 1;
                        }
                    }
                    'V' => {
                        i += 5;
                        u += 1;
                    }
                    'X' => {
                        if u + 1 == v.len() {
                            i += 10;
                            break;
                        }
                        if v[u + 1] == 'L' {
                            i += 40;
                            u += 2;
                        } else if v[u + 1] == 'C' {
                            i += 90;
                            u += 2;
                } else {
                    i += 10;
                    u += 1;
                }
            }
            'L' => {
                i += 50;
                u += 1;
            }
            'C' => {
                if u + 1 == v.len() {
                    i += 100;
                    break;
                }
                if v[u + 1] == 'D' {
                    i += 400;
                    u += 2;
                } else if v[u + 1] == 'M' {
                    i += 900;
                    u += 2;
                } else {
                    i += 100;
                    u += 1;
                }
            }
            'D' => {
                i += 500;
                u += 1;
            }
            'M' => {
                i += 1000;
                u += 1;
            }
            _ => {u += 1;}
        }
    }
    i
}