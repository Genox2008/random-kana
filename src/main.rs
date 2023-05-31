use rand::Rng;
use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();
   let amount_of_printed_kana = &args[1];

   let kana_vec = vec!["nothing","a", "e","i","o","u","ka","ke","ki","ko","ku",
   "sa","se","shi","so","su","ta","te","chi","to","tsu","ji", "zu", "ga","ge","gi","go",
   "gu", "za","ze","zo","da","de","do", "na", "ni","nu", "ne", "no","ba","bi","bu", "be",
   "bo", "pa", "pi", "pu", "pe", "po", "ha", "hi", "fu", "he", "ho", "ma", "mi", "mu", 
   "me", "mo", "ya", "yu", "yo", "ra", "ri", "ru", "re","ro", "wa", "kya", "kyu", "kyo",
   "sha", "shu", "sho", "cha", "chu", "cho", "nya", "nyu", "nyo", "hya", "hyu", "hyo", 
   "mya","myu", "myo", "rya", "ryu", "ryo","gya", "gyu","gyo","ja", "ju", "jo", "bya", 
   "byu", "byo","pya", "pyu","pyo"
   ];

   println!("Write the following in Hiragana/Katakana: ");
   for i in 1..amount_of_printed_kana.parse::<i32>().unwrap()+1 {
      let kana_indx = rand::thread_rng().gen_range(1..kana_vec.len()+1);
      println!("{}", kana_vec[kana_indx]);
   }
}
