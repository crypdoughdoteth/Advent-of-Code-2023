use std::fs; 

pub fn parttwo() -> Result<u32, Box<dyn std::error::Error>> {

    let input = fs::read_to_string("day-1/input.txt")?;
    let codes = input.split("\n").collect::<Vec<&str>>();

    let final_answer: Vec<u32> = codes.iter().map(|e| ->  Result<u32, Box<dyn std::error::Error>>{
        let o = e.replace("one", "o1e");
        let two = o.replace("two", "t2o");
        let three = two.replace("three", "t3e");
        let four = three.replace("four", "f4r");
        let five = four.replace("five", "f5e");
        let six = five.replace("six", "s6x");
        let seven = six.replace("seven", "s7n");
        let eight = seven.replace("eight", "e8t");
        let nine = eight.replace("nine", "n9e");
        let nums =  nine.chars().filter(|c| c.is_numeric()).collect::<String>();
       // println!("{}", nums);
        let mut rstr = String::new(); 
        let first = nums.chars().next().unwrap_or('0');
        let last = nums.chars().last().unwrap_or(first);
        rstr.push(first);
        rstr.push(last);
        Ok(rstr.parse::<u32>()?)
    }).collect::<Result<Vec<u32>, Box<dyn std::error::Error>>>()?;
//    println!("{:#?}", final_answer);
    let fa = final_answer.into_iter().sum::<u32>();
    Ok(fa)
}

