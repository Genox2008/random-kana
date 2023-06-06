use std::env;

fn main() {
   // For argument input
   let args: Vec<String> = env::args().collect();
   // Kana which can be printed 
   let kana_vec = vec!["n", "a", "e","i","o","u","ka","ke","ki","ko","ku",
   "sa","se","shi","so","su","ta","te","chi","to","tsu","ji", "zu", "ga","ge","gi","go",
   "gu", "za","ze","zo","da","de","do", "na", "ni","nu", "ne", "no","ba","bi","bu", "be",
   "bo", "pa", "pi", "pu", "pe", "po", "ha", "hi", "fu", "he", "ho", "ma", "mi", "mu", 
   "me", "mo", "ya", "yu", "yo", "ra", "ri", "ru", "re","ro", "wa", "kya", "kyu", "kyo",
   "sha", "shu", "sho", "cha", "chu", "cho", "nya", "nyu", "nyo", "hya", "hyu", "hyo", 
   "mya","myu", "myo", "rya", "ryu", "ryo","gya", "gyu","gyo","ja", "ju", "jo", "bya", 
   "byu", "byo","pya", "pyu","pyo"
   ];
   println!("Write the following in Hiragana/Katakana: ");
   rand_kana::print_random_kana(kana_vec, args);
}

mod rand_kana {
   use rand::Rng;
   // keep in mind that we take ownership here, since we don't need "args" and "kana_vec" anymore
   pub fn print_random_kana(string_vector: Vec<&str>, argument: Vec<String>) {
      let amount_of_printed_kana = &argument[1];
      for i in 1..amount_of_printed_kana.parse::<i32>().unwrap()+1 {
         let kana_indx = rand::thread_rng().gen_range(1..string_vector.len());
         println!("{}", string_vector[kana_indx]);
      }
   }
}