use unialias_lib::trie::Trie;

fn main() {
    let mut mytrie = Trie::new();
    mytrie.append_leaf("example".to_string(), 'e').unwrap();
    mytrie.append_leaf("except".to_string(), 'e').unwrap();
    mytrie.append_leaf("execution".to_string(), 'e').unwrap();
    mytrie.append_leaf("element".to_string(), 'e').unwrap();
    mytrie.append_leaf("alpha".to_string(), 'e').unwrap();
    mytrie.append_leaf("alpaca".to_string(), 'e').unwrap();
    mytrie.append_leaf("alphamale".to_string(), 'e').unwrap();
    println!("Current Trie: {}", &mytrie);

    let trial = "execute";
    let (midx, mlen) = mytrie.find_max_match(trial.as_bytes());
    println!("{} {}", &mytrie.nodes[midx].value_str(), &trial[..mlen]);
}
