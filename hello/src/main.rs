
fn main(){
    
    let scan = std::io::stdin();
    let mut line = String::new();
    let _ = scan.read_line(&mut line);
    line = line.trim_right().to_owned();

    if line == "1"{
        println!("Hello World");
    }else if line == "2"{
        let mut a_s = String::new();
        let _=scan.read_line(&mut a_s);
        let mut b_s = String::new();
        let _=scan.read_line(&mut b_s);
        let a = &a_s.trim_right().to_owned().parse::<i32>().unwrap();
        let b = &b_s.trim_right().to_owned().parse::<i32>().unwrap();
        println!("{}",a+b);
    }
}