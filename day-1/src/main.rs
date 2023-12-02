pub mod part1; 
pub mod part2; 

use part1::partone; 
use part2::parttwo;

fn main() -> Result<(), Box<dyn std::error::Error>>{
//    let sum =  partone()? +  parttwo()?; 
//    println!("{}", sum);
    partone()?;
    println!("{}", parttwo()?);
    Ok(())
}
