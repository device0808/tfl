use std::collections::{btree_map::Range, HashMap, HashSet, VecDeque};

use reqwest;
use serde_json::json;
pub struct LStarTable {
    is_row_unique: HashMap<Vec<bool>, bool>,
    prefixes: Vec<String>,
    suffixes: Vec<String>,
    table: Vec<Vec<bool>>,
    alphabet: Vec<String>,
    client: reqwest::Client,
    extend_point: usize,
    extended_table: usize,
}
fn clean_concat(prefix: &str, suffix: &str) -> String {
    let mut req_word = format!("{}{}", prefix, suffix);
    req_word = req_word.replace("e", "");
    if req_word.is_empty() {
        "e".to_string()
    } else {
        req_word
    }
}
impl LStarTable {
    pub fn new() -> Self {
        LStarTable {
            is_row_unique: HashMap::new(),
            prefixes: vec!["e".to_string()],
            suffixes: vec!["e".to_string()],
            table: vec![vec![false]],
            alphabet: vec![
                "L".to_string(),
                "R".to_string(),
            ],
            client: reqwest::Client::new(),
            extend_point: 0,
            extended_table: 1,
        }
    }


    pub async fn check_table(
        &self,
        main_prefixes: String,
        complementary_prefixes: String,
        suffixes: String,
        table: String,
    ) -> String {
        let data = json!({
            "main_prefixes": main_prefixes,
            "non_main_prefixes": complementary_prefixes,
            "suffixes": suffixes,
            "table": table
        });
        //println!("main_prefixes:{}", main_prefixes);
        //println!("complementary_prefixes:{}", complementary_prefixes);
        //println!("suffixes:{}", suffixes);
        //println!("table:{}", table);

        let res = self
            .client
            .post("http://localhost:8080/check_table")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .await
            .unwrap();

        res.text().await.unwrap()
    }

    pub async fn check_membership(&self, word: String) -> bool {
        let res = self
            .client
            .post("http://localhost:8080/checkWord")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json!({ "word": word }))
            .send()
            .await
            .unwrap();
    
        let ret_text = res.json::<String>().await.unwrap();
        ret_text["response"].as_bool().unwrap()
    }

    pub async fn get_path(&self) -> String {
        let res = self
            .client
            .get("http://localhost:8080/get_path")
            .send()
            .await
            .unwrap();

        res.text().await.unwrap()
    }

    pub async fn add_prefix(&mut self, prefix: &String) {
        self.prefixes.push(prefix.to_string());
        let mut new_row = vec![false; self.suffixes.len()];
        for (i, suffix) in self.suffixes.iter().enumerate() {
            let new_word = clean_concat(prefix, &suffix);
            new_row[i] = self.check_membership(new_word).await;
        }
        self.table.push(new_row);
    }
    pub async fn add_suffix(&mut self, suffix: &String) {
        self.suffixes.push(suffix.to_string());
        for (i, prefix) in self.prefixes.iter().enumerate() {
            let new_word = clean_concat(prefix, &suffix);
            let buff = self.check_membership(new_word).await;
            self.table[i].push(buff);
        }
    }

    pub async fn extend_table(&mut self) {
        //println!("extend_table");
        
        while self.extend_point < self.extended_table {
            for letter in self.alphabet.clone() {
                let new_suffix = &clean_concat(&self.prefixes[self.extend_point], &letter);
                //println!("{} , {} , {}, {}",new_suffix,self.extend_point, self.extended_table ,letter);
                self.add_prefix(new_suffix).await;
                self.close().await;
            }
            self.extend_point += 1;
        }
    }
    pub async fn add_counter_exempel(&mut self, string: &str) {
        for i in (0..string.len()).rev() {
            let suffix = &string[i..];
            if !self.suffixes.contains(&suffix.to_string()) {
                self.add_suffix(&suffix.to_string()).await;
            }
        }
    }
    
    
    pub async fn close(&mut self) -> bool {
        let mut i = self.extended_table;
        while i < self.prefixes.len() {
            let row = self.table[i].clone();
            if self.is_row_unique(i) {
                self.move_prefix_to_base(i)
            }
            i += 1;
        }
        return true;
    }
    pub fn is_row_unique(&self, index: usize) -> bool {
        for j in 0..self.extended_table {
            if self.table[index] == self.table[j] {
                return false;
            }
        }
        true
    }
    
    pub fn move_prefix_to_base(&mut self, prefix_indx: usize) {
        let row = self.table[prefix_indx].clone();
        //self.is_row_unique.insert(row.clone(), true);
        let prefix = self.prefixes[prefix_indx].clone();
        
        if prefix_indx > self.extended_table {
            //print!("move_prefix_to_base: ");
            let mut i = prefix_indx;
            while i > self.extended_table {
                //print!("{} ",i);
                self.table[i] = self.table[i - 1].clone();
                self.prefixes[i] = self.prefixes[i - 1].clone();
                i-=1
            }
        }
        //println!("");
        self.table[self.extended_table] = row;
        self.prefixes[self.extended_table] = prefix.to_string();
        self.extended_table += 1;
    }
    pub async fn is_table_eq(&mut self) -> (String, bool) {
        let table = self
            .table
            .iter()
            .flatten()
            .map(|&b| if b { '1' } else { '0' })
            .collect();
        let main_prefixes = self.prefixes[..self.extended_table].join(" ");
        let mut complementary_prefixes = "".to_string();
        if (self.extended_table < self.prefixes.len()) {
            complementary_prefixes = self.prefixes[self.extended_table..].join(" ");
        }
        let str = self
            .check_table(
                main_prefixes,
                complementary_prefixes,
                self.suffixes.join(" "),
                table,
            )
            .await;
        if str == "true" {
            return (str, true);
        } else {
            return (str, false);
        }
        /* `std::string::String` value */
    }
}
