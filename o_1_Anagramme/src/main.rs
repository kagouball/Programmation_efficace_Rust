use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU16};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ptr::hash;
use itertools::Itertools;
use std::time::Instant;

/*
fn is_in(mot : &str, dico : Vec<Vec<&str>>)->(bool,u16){
    let is_in_dico = AtomicBool::new(false);
    let indice = AtomicU16::new(0);
    dico.into_par_iter().for_each(|&sig| {
        if sig.contains(&mot){
            is_in_dico.store(true,Ordering::SeqCst);
            indice.store(sig.index(&mot).unwrap(),Ordering::SeqCst);
        }
    });
    (is_in_dico.load(Ordering::SeqCst) , indice.load(Ordering::SeqCst))
}*/

fn anagrams(entree:&str) -> Vec<HashSet<String>>{   //pk on passe de vec a HashSet??
    entree.split_whitespace().dedup().into_iter().fold(HashMap::new(), |mut dico, mot:&str|{
        let signature = mot.chars().sorted().into_iter().collect::<String>();
        let values = dico.entry(signature).or_insert_with(HashSet::new);
        if !values.iter().any(|e| e == mot){v.push(mot.to_owned())}
        dico
    });
    dico.into_iter()
        .filter_map(|val| if val.len()>1{Option::from(val)}else { None })
        .collect()
}

fn anagrams_par(entree:&str) -> Vec<Vec<String>>{
    //traitement entree --> O(n)
    let mut mots= entree.split_whitespace().dedup();
    //traitement --> O(n)
    let mut dico : HashMap<String,Vec<String>> = HashMap::new();
    mots.into_iter().for_each(|mot|{
        let mut signature : Vec<char> = mot.chars().collect();
        signature.sort();
        let s = String::from_iter(signature);
        let v = dico.entry(s).or_insert_with(||Vec::new());
        if !v.iter().any(|e| e == mot){v.push(mot.to_owned())}  //v.iter.any(e==mot) <==> v.contains(mot) but without malloc
    });
    //construction reponse --> O(n)
    dico.into_iter()
        .filter(|(_,val)| val.len()>1)      //utiles? pas trop overkill?
        .map(|(_,val)|{ val })
        .collect()
}

fn anagrams_opti(entree:&str) -> Vec<HashSet<String>>{
    //traitement entree --> O(n)
    let mut mots= entree.split_whitespace().dedup();
    //traitement --> O(n)
    let mut dico : HashMap<String,HashSet<String>> = HashMap::new();
    mots.into_iter().for_each(|mot|{
        let mut signature : Vec<char> = mot.chars().collect();
        signature.sort();
        let s = String::from_iter(signature);
        dico.entry(s).or_insert_with(HashSet::new).insert(mot.to_owned());
    });
    //construction reponse --> O(n)
    dico.into_iter()
        .filter(|(_,val)| val.len()>1)      //utiles? pas trop overkill?
        .map(|(_,val)|{ val })
        .collect()
}

fn anagrams_vf(entree:&str) -> Vec<Vec<String>>{
    //traitement entree --> O(n)
    let mut mots: Vec<&str> = entree.split_whitespace().collect();
    mots.dedup();
    //traitement --> O(n)
    let mut dico : HashMap<String,Vec<String>> = HashMap::new();
    mots.into_iter().for_each(|mot|{
        let mut signature : Vec<char> = mot.chars().collect();
        signature.sort();
        let s = String::from_iter(signature);
        if dico.contains_key(&s){
            let mut v = dico.get_mut(&s).unwrap();
            if !v.contains(&String::from_iter(mot.chars())){
                v.push(String::from(mot));
            }
        }
        else
        {
            let mut sig : Vec<String> = vec![];
            sig.push(String::from(mot));
            dico.insert(s,sig);
        }
    });
    //construction reponse --> O(n)
    let mut reponse:Vec<Vec<String>> = vec![];
    dico.into_iter()
        .for_each(|v|{
            if v.1.len()>1{             //filter a la place?
                reponse.push(v.1);
            }
    });
    reponse
}

fn anagrams_v3(entree:&str) -> Vec<Vec<String>>{
    //on prend les mots de l'entree qu'on met dans un vecteur
    let mut mots :Vec<&str> = entree.split_whitespace().collect();
    //println!("mots\t\t\t\t: {:?}", mots);
    //On enlève les doublons
    mots.dedup();
    //println!("mots sans doublons\t: {:?}", mots);
    //dico contenant en clé la signature des mots et un valeurs les mots correspondants à la signature
    let mut dico : HashMap<String,Vec<String>> = HashMap::new();

    mots.into_iter().for_each(|m|{
        //println!("mot traité\t: {}", &m);
        let mut signature :Vec<char> = m.chars().collect();
        signature.sort();
        let s = String::from_iter(signature);
        //println!("signature\t: {}", s);
        if dico.contains_key(&s){
            dico.get_mut(&s).unwrap().push(String::from(m));
        }
        else
        {
            let mut sig : Vec<String> = vec![];
            sig.push(String::from(m));
            dico.insert(s,sig);
        }
    });

    /*for (k,v) in &dico{
        println!("{}: \"{:?}\"",k,v);
    }*/
    //contruction de la reposne
    let mut reponse:Vec<Vec<String>> = vec![];
    dico.into_iter().filter(|v|v.1.len()>1).for_each(|v|{
        reponse.push(v.1);
    });
    reponse
}

//fn anagrams_v2(entree:&str) -> Vec<&Vec<&str>> {
    /*
    //on prend les mots de l'entree qu'on met dans un vecteur
    let mut mots :Vec<&str> = entree.split_whitespace().collect();
    //On enlève les doublons
    mots.dedup();
    //dico contenant en clé la signature des mots et un valeurs les mots correspondants à la signature
    let mut dico : HashMap<String,Vec<&str>>;

    mots.iter()
        .map(|&mot|{
            let mut signature :Vec<char> = mot.chars().collect();
            signature.sort();
            String::from_iter(signature)
            //&signature.iter().collect::<String>()
        })
        .zip(mots)
        .for_each(|(signature,mot)|{
            //let s: String = signature.as_str().to_string();
            if dico.contains_key(&*signature){
                dico.get_mut(&*signature).unwrap().push(mot);
            }
            else
            {
                let mut sig : Vec<&str> = vec![];
                sig.push(mot);
                dico.insert(signature,sig);
            }
    });*/
/*
    mots.iter().map(|&mot|{
        let mut signature :Vec<char> = mot.chars().collect();
        signature.sort();
        let s : &str = &signature.iter().collect::<String>();
        if dico.contains_key(s){
            dico.get_mut(s).unwrap().push(mot);
        }
        else
        {
            let mut sig : Vec<&str> = vec![];
            sig.push(mot);
            dico.insert(s,sig);
        }
    }); */

    //contruction de la reposne
    //let mut reponse:Vec<&Vec<&str>> = vec![];
    //dico.values().for_each(|values|{
    //    reponse.push(values);
    //});
    //reponse
//}

/*
fn anagrams(entree:&str)->u16{
    //on prend les mots de l'entree
    let mut mots :Vec<&str> = entree.split_whitespace().collect();
    //On enlève les doublons
    mots.dedup();
    //dico resultat
    let mut res : Vec<Vec<&str>> = vec![];

    (0..mots.len()).iter().for_each(|i|{
        let mot : &str = mots.get(i).unwrap();
        let mut chars : Vec<char> = mot.chars().collect();
        chars.sort();
        let signature : &str = chars.iter().collect();
        (present, indice) = is_in(signature,res);
        if present{
            res[indice].append(i);
        }
    });
    /*
    mots.iter()//pour tout les mots dans la phrase
        .map(|&m|{              //pour un mot m
            let mut chars : Vec<char> = m.chars().collect();
            chars.sort();           //signature du mot
            if is_in(m,res) {
                //si la signature existe dans le dico

            }

    }).all(|&m| )
*/
    let mut reponse = vec![];

    res.iter().for_each(|&s|{
        if res.get(s)
    })

    println!("mots : {:?}", mots);
    0
}
*/

fn main() {
    let phrase: &'static str = "le chien marche vers sa niche et trouve une limace de chine nue pleine de malice qui lui fait du charme";
    let n1 = Instant::now();
    let an : Vec<HashSet<String>> = anagrams_opti(phrase);
    //let an : Vec<Vec<String>> = anagrams_v3(phrase);
    let n2 = Instant::now();
    println!("entree : {}",phrase);
    println!("anagrammes : {:?}",an);
    println!("time : {:?}",n2.duration_since(n1));

}
