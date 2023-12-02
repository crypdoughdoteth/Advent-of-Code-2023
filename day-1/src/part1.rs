use std::fs;

pub fn partone() -> Result<u32, Box<dyn std::error::Error>>{    
    let input = fs::read_to_string("day-1/input.txt")?;
    let codes = input.split("\n").collect::<Vec<&str>>();
    let final_answer: Vec<u32>  = codes.iter().map(|e| -> Result<u32, Box<dyn std::error::Error>> {
        let nums =  e.chars().filter(|c| c.is_numeric()).collect::<String>();
        let mut rstr = String::new(); 
        let first = nums.chars().next().unwrap_or('0');
        let last = nums.chars().last().unwrap_or(first);
        rstr.push(first);
        rstr.push(last);
        Ok(rstr.parse::<u32>()?)
    }).collect::<Result<Vec<u32>, Box<dyn std::error::Error>>>()?;
    let fa = final_answer.into_iter().sum::<u32>(); 
    println!("{}", fa);
    Ok(fa)
}
