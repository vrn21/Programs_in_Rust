use std::io;

fn first_word(sentence: &String) -> usize{
    let bytes = sentenc.as_bytes();
    for (i, &item) in  bytes.iter().enumerate(){
        if &item == b' '{
            return i;
        }
    }
    return sentence.len();

}

fn main(){
    println!("Enter a sentence");
    let mut sentence = String::new();


}