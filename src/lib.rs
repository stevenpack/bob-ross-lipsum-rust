// #![feature(extern_prelude)]
extern crate rand;

mod phrases {
    use rand::{thread_rng, Rng};
    // Compiled by http://www.bobrosslipsum.com/ 2016 January
    static PHRASES: [&str; 4] = [  
        "A beautiful little sunset.",
        "A big strong tree needs big strong roots.",
        "A fan brush is a fantastic piece of equipment. Use it. Make friends with it.",
        "A happy cloud."];

    fn get_random_phrase() -> &'static str {        
        let mut rng = thread_rng();
        let idx: usize = rng.gen_range(0, PHRASES.len());
        PHRASES[idx]        
    }

    fn get_phrases(len: usize) -> Vec<&'static str> {
        let mut phrases = Vec::with_capacity(len);
        for _ in 0..len {
            phrases.push(get_random_phrase())
        }
        phrases
    }

    fn break_lines(phrases: Vec<&'static str>, new_line: usize) -> Vec<&'static str> {
        let len = phrases.len();
        let new_lines = len / new_line;
        let capacity = len + new_lines;
        let mut phrases_with_newlines = Vec::with_capacity(capacity);
        for i in 0..len {
            if i > 0 && i % new_line == 0 {
                phrases_with_newlines.push("\n\n");
            } 
            //intention here is not to resize the vec, so check our math is right
            debug_assert!(phrases_with_newlines.len() < capacity);

            phrases_with_newlines.push(phrases[i]);            
        }
        phrases_with_newlines
    }

    pub fn get_phrases_with_newlines(len: usize, new_line: usize) -> Vec<&'static str> {
        let phrases = get_phrases(len);
        break_lines(phrases, new_line)
    }

    pub fn get_phrase_text(phrases: usize, new_line: usize) -> String {
        
        let phrases_vec = get_phrases(phrases);
        let mut string = String::new();
        for i in 0..phrases_vec.len() {
            if i > 0 && new_line > 0 && i % new_line == 0 {
                string.push_str("\n\n");
            }
            string.push_str(phrases_vec[i]);
        }
        string
    }
//     let happyPhrases = new Array(length);
//     for (let i=0; i<length; i++) {
//         happyPhrases[i] = getPhrase() + " ";
//     }
//     return happyPhrases;
// }

    #[cfg(test)]
    mod tests {
        use super::*;

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
        fn get_phrases_with_newlines_counts() {
            let phrases = get_phrases_with_newlines(101, 5);
            
            let new_line_cnt = phrases.iter().filter(|&n| *n == "\n\n").count();
            let phrase_cnt = phrases.len() - new_line_cnt;
            assert_eq!(new_line_cnt, 20);
            assert_eq!(phrase_cnt, 101);

            println!("{:?}", phrases);
        }

         #[test]
        fn get_phrase_text_dump() {
            let string = get_phrase_text(101, 5);
            println!("{:?}", string);
        }
    }

}




// function breakLines(phraseArr, newLine) {
//     let lines = [];
//     for (let i=0; i < phraseArr.length; i++) {
//         lines.push(phraseArr[i]);
//         if (i > 0 && i % newLine === 0) {
//             lines.push("\n\n");
//         }
//     }
//     return lines;
// }

