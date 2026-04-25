use std::collections::HashMap;

fn main() {
    //median
    let mut integers = vec![3,1,4,1,5,9,2,6,5,5];
    integers.sort();
    let len = integers.len();
    let median;
    if len % 2 == 0 {
        median = (integers[len / 2] + integers[len / 2 - 1]) / 2;
    } else {
        median = integers[len / 2];
    }
    println!("{:?}", integers);
    println!("{:?}", len);
    println!("{:?}", median);

    //mode
    let mut map = HashMap::new();
    for num in &integers{
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max_count =0;
    let mut mode = 0;
    for (num, count) in &map {
        if *count > max_count {
            max_count = *count;
            mode = **num;
        }
    
    }

    println!("{:?}", mode);
    println!("{:?}", map);


    //pig latin
    let word = "apple";
    let first_char = word.chars().next().unwrap();
    println!("{:?}", first_char);
    let vowels = ['a','e','i','o','u'];
    if vowels.contains(&first_char){
        println!("{}-hay",word)
    }else{
      let rest = word.chars().skip(1).collect::<String>();
     let result =  format!("{}-{}ay",rest, first_char);
     println!("{}",result);
    }

    //employee and dept
    let mut employee_dep :HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();
    while input.trim() != "quit"{
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() || parts[0] == "quit" {
            break;
        }
        if parts[0] == "add"{
            employee_dep.entry(parts[3].to_string()).or_insert(Vec::new()).push(parts[1].to_string());
        }
        if parts[0] == "list" && parts[1] == "eng"{
            println!("{:?}",employee_dep.get("eng"));
        }
        if parts[0] == "list" && parts[1] == "all"{
            println!("{:?}",employee_dep);
        }
        println!("{:?}",parts);
        println!("{}",input);
        
    }
}
