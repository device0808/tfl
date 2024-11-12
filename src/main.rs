use lstar::LStarTable;
use tokio;
mod lstar;
// main.rs
#[tokio::main]
async fn main() {
    
    let mut lstar = LStarTable::new();
   
    lstar.extend_table().await;
    let (mut exmpl, mut is_eq) = lstar.is_table_eq().await ;
    let mut buff = String::new();
    println!("{}",exmpl);
    
    while !is_eq{
        lstar.add_counter_exempel(&exmpl).await;
        lstar.close().await;
        lstar.extend_table().await;
        
        (exmpl,is_eq) = lstar.is_table_eq().await ;
        if is_eq{
            println!("Eq");
            return;
        }
        println!("{}",exmpl);
        
        //io::stdin().read_line(&mut buff);
        
    }
}
