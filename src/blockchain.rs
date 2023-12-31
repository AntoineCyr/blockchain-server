use crate::block::Block;
use crate::block::Transaction;
pub type Result<T> = std::result::Result<T,failure::Error>;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::thread::sleep;


#[derive(Debug, Clone)]
pub struct Blockchain {
    blockchain_data: BlockchainData,
}

#[derive(Debug, Clone)]
pub struct BlockchainData {
    current_hash: i32,
    mempool: Vec<Transaction>,
    chain: HashMap<i32,Block>,
    state: HashMap<String,i32>
}

impl Blockchain {
    pub fn get_balance(&mut self,address:String) -> i32 {
        return match self.blockchain_data.state.get(&address) {
            Some(&number) => number,
            _ => -1,
        }
    }

    pub fn create_blockchain() -> Result<Blockchain> {
        let mut chain = HashMap::new();
        let state = HashMap::new();
        let last_hash = 0;
        let mempool = Vec::new();
        let block = Block::new_block(mempool.clone(),last_hash);
        let block_hash = block.get_hash();
        chain.insert(block_hash.clone(),block);
        let bc_data: BlockchainData = BlockchainData{
            current_hash: block_hash,
            mempool,
            chain,
            state
        };

        Ok(Blockchain{blockchain_data:bc_data})
    }

    pub fn add_block(&mut self) -> Result<()>{
        let block = Block::new_block(self.blockchain_data.mempool.clone(),self.blockchain_data.current_hash.clone());
        for transaction in self.blockchain_data.mempool.clone() {
            let from: String = transaction.get_from();
            let to: String = transaction.get_to();
            let amount: i32 = transaction.get_amount();

            let number_from = match self.blockchain_data.state.get(&from) {
                Some(&number) => number,
                _ => 0,
            };
            let number_to = match self.blockchain_data.state.get(&to) {
                Some(&number) => number,
                _ => 0,
            };

            self.update_state(from,number_from-amount)?;
            self.update_state(to,number_to+amount)?;
            

        }

        println!("new block, number of transactions confirmed: {}",self.blockchain_data.mempool.len());
        self.blockchain_data.mempool.clear();
        self.blockchain_data.chain.insert(block.get_hash(),block.clone());
        self.blockchain_data.current_hash = block.get_hash();
    

        Ok(())
    }

    pub fn add_transaction(&mut self,from:String,to:String,amount:i32) -> Result<()>{
        let transaction = Transaction::new(from,to,amount);
        self.blockchain_data.mempool.push(transaction);

        Ok(())
    }

    fn update_state(&mut self,address:String,amount:i32) -> Result<()>{
        self.blockchain_data.state.insert(address,amount);
        Ok(())
    }

    pub fn run_blockchain(&mut self) -> Result<()>{
        let mut now = SystemTime::now();
        //bc.add_block()?;
        //bc.clear();
        loop {
            match now.elapsed() {
                Ok(elapsed) => {
                    // it prints '2'
                    sleep(Duration::new(10-elapsed.as_secs(), 0));
                }
                Err(e) => {
                    // an error occurred!
                    println!("Error: {e:?}");
                }
            }
            now = SystemTime::now();
            //bc.add_block()?;
        }
    }


}