use std::collections::HashMap;
use std::default::Default;

const NUMBER_OF_SLICES: f64 = 3.0;

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
enum MoneyUnit {
    HUNDRED,
    THOUSAND,
    MILLION,
    BILLION,
    TRILLION,
    QUADRILLION,
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

#[derive(Debug)]
struct SpeakInLocalLang {
    numbers: Vec<Vec<char>>,
    stringConverter: NumberToStringInLocalLang,
}

impl SpeakInLocalLang {
    fn convert_number_to_local_words(&self) -> String {
        String::from("hello")
    }
}

#[derive(Debug)]
struct NumberToStringInLocalLang {
    lan: Lan,
    numbers: HashMap<char, Number>,
    money_units: HashMap<MoneyUnit, String>,
}

fn main() {
    let money = MoneySpeaker::new(1450120, Lan::EN);

    let a = money.money_format();

    let mut numToString = NumberToStringInLocalLang {
        lan: Lan::EN,
        numbers: HashMap::new(),
        money_units: HashMap::new(),
    };

    match numToString.lan {
        Lan::EN => {
            numToString
                .numbers
                .insert('1', Number::ONE(String::from("One")));
            numToString
                .numbers
                .insert('2', Number::TWO(String::from("Two")));
            numToString
                .numbers
                .insert('3', Number::THREE(String::from("Three")));
            numToString
                .numbers
                .insert('4', Number::FOUR(String::from("Four")));
            numToString
                .numbers
                .insert('5', Number::FIVE(String::from("Five")));
            numToString
                .numbers
                .insert('6', Number::SIX(String::from("Six")));
            numToString
                .numbers
                .insert('7', Number::SEVEN(String::from("Seven")));
            numToString
                .numbers
                .insert('8', Number::EIGHT(String::from("Eight")));
            numToString
                .numbers
                .insert('9', Number::NINE(String::from("Nine")));
            numToString
                .money_units
                .insert(MoneyUnit::HUNDRED, String::from("Hundred"));
            numToString
                .money_units
                .insert(MoneyUnit::THOUSAND, String::from("Thousand"));
            numToString
                .money_units
                .insert(MoneyUnit::MILLION, String::from("Million"));
            numToString
                .money_units
                .insert(MoneyUnit::BILLION, String::from("Billion"));
            numToString
                .money_units
                .insert(MoneyUnit::TRILLION, String::from("Trillion"));
            numToString
                .money_units
                .insert(MoneyUnit::QUADRILLION, String::from("Quadrillion"));
        }
    }

    let money_speaker = SpeakInLocalLang {
        numbers: money.money_format(),
        stringConverter: numToString,
    };

    println!("{:?}", money_speaker);

}
