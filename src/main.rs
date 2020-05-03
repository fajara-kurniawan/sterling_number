use bigdecimal::BigDecimal;
use std::io;

fn factorial(num: u64) -> BigDecimal {
    match num {
        0 => BigDecimal::from(1),
        1 => BigDecimal::from(1),
        _ => BigDecimal::from(factorial(num - 1) * BigDecimal::from(num)),
    }
}

struct GladiatorsFight{
    count_gladiators : u64
}

impl GladiatorsFight{
    fn change_crown(&self) {
        let stirling_number = BigDecimal::from(self.stirling_number());
        let count_permutation = BigDecimal::from(self.count_permutation());
        let result  = stirling_number/count_permutation;
        println!("There are {} gladiators, after a long fight, The survivor replaced approximately {:.2} times until the King of Gladiator is crowned",self.count_gladiators,result)
    }

    fn count_permutation(&self) -> BigDecimal{
        factorial(self.count_gladiators) /  factorial(self.count_gladiators - self.count_gladiators)
    }

    fn stirling_number(&self) -> BigDecimal{
        let n_factorial = factorial(self.count_gladiators-1);
        let sum_k = self.sum_k();
        n_factorial*BigDecimal::from(sum_k)
    }

    fn sum_k(&self) -> f32 {
        let mut sum : f32 = 0.0;
        let num = self.count_gladiators - 1;
        for i in 0..num{
            let partial_sum : f32 = (i+1) as f32 / (num-i) as f32;
            sum = sum + partial_sum;
        } 
        sum
    }
}

fn main() {

    loop {
    println!("Please input number of gladiators");

    let mut input = String::new();
    
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    if let Err(_) = input.trim().parse::<i32>() {
        println!("{} is not a number",input.trim());
        continue
    }
    else{
        let input : u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
         };

         let x = GladiatorsFight {
            count_gladiators : input
        };
        x.change_crown();
        break
    }

    }
    
}
