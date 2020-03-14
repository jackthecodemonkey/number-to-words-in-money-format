const numberOfSlices: f64 = 3.0;

#[derive(Debug)]
struct NumToWords {
    num: u32,
}

impl NumToWords {
    fn ToString(self, num: i64) -> Vec<Vec<char>> {
        let string: String = num.to_string();

        let strVec: Vec<char> = string.chars().collect();

        let len: f64 = string.len() as f64;

        let numberOfVec: i8 = (len / numberOfSlices).ceil() as i8;

        let mut vec: Vec<Vec<char>> = Vec::with_capacity(numberOfVec as usize);

        for v in 0..numberOfVec {
            vec.push(vec![])
        }

        let mut i: i8 = len as i8 - 1;
        let mut j: i8 = 0;

        for _ in &strVec {

            let n = strVec[i as usize];

            let currentIndex: usize = (j as f64 / numberOfSlices as f64).floor() as usize;

            vec[currentIndex].push(n);

            i = i - 1;

            j = j + 1;
        }

        vec.reverse();
 
        for v in (0..vec.len()) {
            vec[v].reverse();
        }

        vec
    }
}

fn main() {
    let num = NumToWords { num: 100 };

    let a = num.ToString(1451122010);

    println!("{:?}", a);
}
