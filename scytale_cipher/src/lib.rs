pub fn scytale_cipher(message: String, i: u32) -> String {
    let c = i as usize;         // Number of columns
    let n = message.len();      // Length of the message
    
    if c == 0 {               
        return String::new();
    }
    
    let r = (n + c - 1) / c;   
    let chars: Vec<char> = message.chars().collect(); 
    let mut result = String::new();


    for j in 0..c {           
        for k in 0..r {       
            let idx = j + k * c; 
            if idx < n {         
                result.push(chars[idx]);
            }
        }
    }
    result
}