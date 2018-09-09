#![feature(test)]

extern crate rand;
extern crate test;

pub mod phrases {
    use rand::{thread_rng, Rng};
    // Compiled by http://www.bobrosslipsum.com/ 2016 January
    static PHRASES: [&str; 4] = [  
        "A beautiful little sunset.",
        "A big strong tree needs big strong roots.",
        "A fan brush is a fantastic piece of equipment.",
        "A happy cloud."];

    fn get_random_phrase() -> &'static str {        
        let mut rng = thread_rng();
        let idx: usize = rng.gen_range(0, PHRASES.len());
        PHRASES[idx]        
    }

    fn get_phrases(len: usize) -> Vec<&'static str> {
        (0..len)
            .map(|_| get_random_phrase())
            .collect()        
    }

    fn need_newline(newline: usize, idx: usize) -> bool {
        //idx+1 because idx is zero-based but we want a new line after "every x phrases".
        (newline > 0) && (idx > 0) && ((idx + 1) % newline == 0)
    }

    fn need_space(newline: usize, idx: usize) -> bool {
        !need_newline(newline, idx)
    }
    
    pub fn get_phrase_text(phrase_cnt: usize, newline: usize) -> String {
        
        let phrases_vec = get_phrases(phrase_cnt);
        let mut string = String::new();
        for i in 0..phrases_vec.len() {

            println!("{}", string.capacity());

            //the phrase
            string.push_str(phrases_vec[i]);

            //spaces between phrases
            if need_space(newline, i) {
                string.push(' ');
            }

            //new lines
            if need_newline(newline, i) {
                string.push_str("\n\n");
            }
        }
        string
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test::Bencher;

        #[test]
        fn get_random() {
            let phrase = get_random_phrase();
            println!("{}", phrase);
        }

        #[test]
        fn get_phrases_len() {
            let phrases = get_phrases(10);
            assert_eq!(phrases.len(), 10);
            println!("{:?}", phrases);
        }

        #[test]
        fn is_newline_tests() {
            assert!(!need_newline(5, 3));
            assert!(need_newline(5, 4)); //idx of 4 = 5 phrases (0 based)
            assert!(!need_newline(5, 5));
            assert!(!need_newline(5, 6));
        }

         #[test]
        fn is_space_tests() {
            assert!(need_space(5, 3));
            assert!(!need_space(5, 4));
            assert!(need_space(5, 5));
            assert!(need_space(5, 6));
        }

         #[test]
        fn get_phrase_text_dump() {
            let string = get_phrase_text(16, 5);
            println!("{:?}", string);
        }

        #[bench]
        fn bench_get_phrases(b: &mut Bencher) {
            b.iter(|| get_phrases(1000));
        }
        
        #[bench]
        fn bench_get_phrase_text(b: &mut Bencher) {
            b.iter(|| println!("{}", get_phrase_text(1000, 5)))
        }
    }

}