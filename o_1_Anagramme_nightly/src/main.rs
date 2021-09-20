#![feature(hash_set_entry)]
#![feature(map_into_keys_values)]
use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU16};
use std::cmp::Ordering;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ptr::hash;
use itertools::Itertools;
use std::time::Instant;
use rayon::prelude::*;
use std::collections::hash_map::RandomState;

fn anagrams_night_par(entree:&str) -> Vec<HashSet<String>>{
    entree.replace(&['.',','][..]," ")
        .par_split_whitespace()
        .fold(|| -> HashMap<String, HashSet<String>, RandomState>{HashMap::new()}, |mut dico, mot:&str|{
            dico
                .entry(mot.chars().sorted().collect::<String>())
                .or_insert_with(HashSet::new)
                .get_or_insert_owned(mot);
            dico })
        .reduce(HashMap::new, |m1,m2| hashMap_merge(m1,m2))
        .into_values()
        .filter_map(|val| if val.len()>1{Option::from(val)}else { None })
        .collect()
}

fn hashMap_merge(map1: HashMap<String, HashSet<String>>, mut map2: HashMap<String, HashSet<String>>) -> HashMap<String, HashSet<String>>{
    let mut lmap;
    let mut bmap;
    if map1.capacity() > map2.capacity(){
        lmap = map2;
        bmap = map1;
    }
    else {
        lmap = map1;
        bmap = map2;
    }
    lmap.into_iter().for_each(|(k,v)| {
        let mut m = bmap.entry(k).or_insert_with(HashSet::new);
        m.extend(v)
    });
    bmap
}

fn anagrams_night(entree:&str) -> Vec<HashSet<String>>{
    entree.split_whitespace().dedup().into_iter().fold(HashMap::new(), |mut dico, mot:&str|{
        dico.entry(mot.chars().sorted().into_iter().collect::<String>())
            .or_insert_with(HashSet::new).get_or_insert_owned(mot);
        dico
    }).into_values().filter_map(|val| if val.len()>1{Option::from(val)}else { None }).collect()
}

fn main() {
    //let phrase: &'static str = "le chien marche vers sa niche et trouve une limace de chine nue pleine de malice qui lui fait du charme";
    let phrase: &'static str ="Lorem ipsum dolor sit amet , consectetur adipiscing elit , sed do eiusmod tempor incididunt ut labore et dolore magna aliqua . Facilisi cras fermentum odio eu feugiat pretium nibh ipsum . Adipiscing diam donec adipiscing tristique risus nec feugiat in . Eu volutpat odio facilisis mauris sit amet massa vitae . Commodo quis imperdiet massa tincidunt . Volutpat ac tincidunt vitae semper . Nec ullamcorper sit amet risus nullam eget felis . Tristique sollicitudin nibh sit amet commodo nulla facilisi nullam vehicula . Risus sed vulputate odio ut enim blandit volutpat maecenas . Faucibus pulvinar elementum integer enim neque volutpat ac tincidunt . Turpis egestas maecenas pharetra convallis posuere morbi leo urna. Scelerisque eleifend donec pretium vulputate . Aenean vel elit scelerisque mauris pellentesque . Maecenas sed enim ut sem. Eu turpis egestas pretium aenean pharetra magna ac placerat. Amet est placerat in egestas erat imperdiet sed euismod nisi. Nunc faucibus a pellentesque sit amet porttitor eget dolor . Quis auctor elit sed vulputate mi sit amet. Ornare lectus sit amet est. Aliquam ultrices sagittis orci a. Odio facilisis mauris sit amet massa. Ullamcorper sit amet risus nullam eget felis eget. Tortor aliquam nulla facilisi cras fermentum odio. Arcu cursus euismod quis viverra nibh cras . Purus sit amet volutpat consequat mauris nunc congue nisi vitae . Mi proin sed libero enim sed faucibus . Vivamus at augue eget arcu dictum varius duis. Nunc lobortis mattis aliquam faucibus. Morbi non arcu risus quis varius quam quisque id diam. Eget aliquet nibh praesent tristique. Erat nam at lectus urna duis. At imperdiet dui accumsan sit. Lorem ipsum dolor sit amet consectetur adipiscing elit duis tristique. Integer vitae justo eget magna fermentum. Et magnis dis parturient montes nascetur ridiculus mus mauris. Elementum facilisis leo vel fringilla est. Sed felis eget velit aliquet sagittis id consectetur purus ut. Vitae elementum curabitur vitae nunc sed velit. Vestibulum sed arcu non odio euismod lacinia. Integer eget aliquet nibh praesent. Eleifend donec pretium vulputate sapien. Neque ornare aenean euismod elementum nisi quis eleifend. Eu turpis egestas pretium aenean pharetra. Tellus orci ac auctor augue mauris augue. A arcu cursus vitae congue mauris rhoncus aenean vel elit. Dictumst vestibulum rhoncus est pellentesque elit ullamcorper dignissim . Nibh ipsum consequat nisl vel pretium lectus quam. Fames ac turpis egestas maecenas pharetra convallis. Sit amet aliquam id diam maecenas ultricies mi eget. Condimentum mattis pellentesque id nibh tortor id aliquet lectus proin. At ultrices mi tempus imperdiet nulla malesuada pellentesque elit eget. Neque sodales ut etiam sit amet nisl purus. Interdum consectetur libero id faucibus. Sapien pellentesque habitant morbi tristique senectus et netus et. At erat pellentesque adipiscing commodo elit at imperdiet. Id porta nibh venenatis cras sed. Dignissim convallis aenean et tortor at risus viverra adipiscing at. Sagittis nisl rhoncus mattis rhoncus urna neque viverra justo. In nisl nisi scelerisque eu ultrices vitae auctor. Aliquam vestibulum morbi blandit cursus risus at ultrices mi. Lorem mollis aliquam ut porttitor leo a diam sollicitudin tempor. Ipsum consequat nisl vel pretium lectus quam id leo in.";
    let n1 = Instant::now();
    let an : Vec<HashSet<String>> = anagrams_night_par(phrase);
    //let an : Vec<HashSet<String>> = anagrams_night(phrase);
    let n2 = Instant::now();
    println!("entree : {}",phrase);
    println!("anagrammes : {:?}",an);
    println!("time : {:?}",n2.duration_since(n1));
}
