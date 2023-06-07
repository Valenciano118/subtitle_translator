use subparse::*;
use std::io::Write;
use std::process::exit;
use std::{fs::File,fs::read_to_string};
use std::path::Path;
use rustlate::{self, Translator};
use std::io::stdin;


fn main() {

    let mut route:String = String::new();
    let mut question = String::from("N");

    while question.as_str().to_uppercase() == "N" {
        println!("Input a route: ");
        stdin().read_line(&mut route).unwrap();
        println!("Is the following route correct?: {} (Y/N):",&route);
        stdin().read_line(&mut question).unwrap();
    }

    let mut  target_translation_language = String::new();

    question = String::from("N");

    while question.as_str().to_uppercase() == "N" {
        println!("Input your target translation language abbreviated(E,g en would target English ):");
        stdin().read_line(&mut target_translation_language).unwrap();
        println!("Is the following language correct?: {} (Y/N):",&target_translation_language);
        stdin().read_line(&mut question).unwrap();
    }

    let path = Path::new(route.trim());
    if path.exists(){
        println!("the path exists");
    }
    else{
        println!("the path doesn't exit");
        exit(-1);
    }
    //let mut file = File::open(route.as_str()).unwrap();
    let file_content = read_to_string(path).unwrap();
    
    //file.read_to_string(&mut file_content).unwrap();
    let mut ssa_subtitle = SsaFile::parse(&file_content).unwrap();




    let translator_struct = build_translator_struct(&target_translation_language);

    let mut subs: Vec<SubtitleEntry> = Vec::new();

    for entry in ssa_subtitle.get_subtitle_entries().unwrap(){
        println!("{}",&entry.line.unwrap().clone());
    }
    
    for mut entry in ssa_subtitle.get_subtitle_entries().unwrap(){
        entry.line = match translator_struct.translate(&entry.line.unwrap()){
            Ok(translated) => Some(translated),
            Err(_) => None
        };
        subs.push(entry);
    }
    ssa_subtitle.update_subtitle_entries(&subs[..]).unwrap();

    let filename = format!("subtitle_{}.ass",target_translation_language.trim());
    let mut file = File::create(filename.clone()).unwrap();
    file.write_all(&ssa_subtitle.to_data().unwrap()[..]).unwrap();
    println!("{} created",filename);

   

}


fn build_translator_struct(target_language : &str) -> Translator{
    let temp = Translator{
        to:target_language,
        from:"auto"
    };

    temp
}
