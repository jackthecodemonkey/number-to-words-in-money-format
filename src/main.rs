const NUMBER_OF_SLICES: f64 = 3.0;

enum Number {
    ONE(String),
    TWO(String),
    THREE(String),
    FOUR(String),
    FIVE(String),
    SIX(String),
    SEVEN(String),
    EIGHT(String),
    NINE(String),
}

enum MoneyUnit {
    HUNDRED(String),
    THOUSAND(String),
    MILLION(String),
    BILLION(String),
    TRILLION(String),
    QUADRILLION(String),
}

#[derive(Debug)]
enum Lan {
    EN,
}
#[derive(Debug)]
struct MoneySpeaker {
    num: usize,
    lan: Lan,
}

impl MoneySpeaker {
    fn new(num: usize, lan: Lan) -> Self {
        MoneySpeaker { num, lan }
    }
}

impl MoneySpeaker {
    fn money_format(&self) -> Vec<Vec<char>> {
        let string: String = self.num.to_string();

        let str_vec: Vec<char> = string.chars().collect();

        let len: f64 = string.len() as f64;

        let number_of_vec: i8 = (len / NUMBER_OF_SLICES).ceil() as i8;

        let mut vec: Vec<Vec<char>> = Vec::with_capacity(number_of_vec as usize);

        for _ in 0..number_of_vec {
            vec.push(vec![])
        }

        let mut i: i8 = len as i8 - 1;
        let mut j: i8 = 0;

        for _ in &str_vec {
            let n = str_vec[i as usize];

            let current_index: usize = (j as f64 / NUMBER_OF_SLICES as f64).floor() as usize;

            vec[current_index].push(n);

            i = i - 1;

            j = j + 1;
        }

        vec.reverse();
        for v in 0..vec.len() {
            vec[v].reverse();
        }
        vec
    }

    fn Speak(&self) -> String {
        return String::from("Hello");
    }
}

fn main() {
    let money = MoneySpeaker::new(1450120, Lan::EN);

    let a = money.money_format();

    println!("{:?}", money.Speak());

    println!("{:?}", a);
}
