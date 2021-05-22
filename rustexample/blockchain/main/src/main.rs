use core::blockchain;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    bc.add_block("a -> b: 5btc".to_owned());
    bc.add_block("c -> d: 1btc".to_owned());

    for elem in bc.blocks {
        println!("=========================");
        println!("{:#?}", elem);
        println!("");
    }
}
