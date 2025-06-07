use unialias_core::trie::Trie;

fn main() {
    let mut mytrie = Trie::new();
    println!("{:?}", mytrie.append_leaf("example".to_string(), 'e'));
    println!("{:?}", mytrie.append_leaf("examine".to_string(), 'e'));
    println!("{:?}", mytrie.append_leaf("except".to_string(), 'e'));
    println!("{:?}", mytrie.append_leaf("execution".to_string(), 'e'));
    println!("{:?}", mytrie.append_leaf("element".to_string(), 'e'));
    println!("{:?}", mytrie.append_leaf("alpha".to_string(), 'e'));
    println!("{:?}", mytrie.append_leaf("alpaca".to_string(), 'e'));
    println!("{:?}", mytrie.append_leaf("alphamale".to_string(), 'e'));
    println!("{:?}", mytrie.nodes);
    for x in mytrie.iter(&0) {
        println!("{:?}", x);
    }
    println!("Current Trie: {}", &mytrie);

    let trial = "execute";
    let (midx, mlen) = mytrie.find_max_match(trial.as_bytes());
    println!("{} {}", &mytrie.nodes[midx].value_str(), &trial[..mlen]);
}
