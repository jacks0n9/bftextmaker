/*!
 * # Usage
 * ```
 * use bftextmaker::gen_code;
 * let (code,memory_used)=gen_code("The quick brown fox jumps over the lazy dog",15);
 * println!("Your code is: {}",code);
 * println!("Used {} cells",memory_used);
 * ```
 */
use std::collections::HashSet;

/// Generates code for the given text.
/// Uses up to the max_memory. Reccomended max is 15.
/// Returns the code as well as the memory actually used
pub fn gen_code(input: &str, max_memory: usize) ->(String,usize) {
    let mut absolute_best:Option<String>=None;
    let chars = input.as_bytes();
    let mut unique:Vec<u8>=chars.iter().fold(HashSet::new(), |mut set,item|{
        set.insert(*item);
        set
    }).into_iter().collect();
    unique.sort();
    let unique_amount=unique.len();
    let mut best_mem=0;
    // just brute force pretending we have different amounts of memory available
    for mem in 2..max_memory {
        let memory_for_chars = std::cmp::min(mem - 1, unique_amount);
        // good enough
        let mut initializers:Vec<u8> = unique.clone().into_iter().take(memory_for_chars).collect();
        let mut shortest_code = gen_init_code(&initializers, 1);
        let max=match chars.iter().max(){
            Some(max)=>max,
            None=>return (String::new(),0)
        };
        for i in 2..*max*2 {
            let code = gen_init_code(&initializers, i);
            if code.len() < shortest_code.len() {
                shortest_code = code;
            }
        }
        let mut pointer = initializers.len() - 1;
        for char in chars {
            // find the best cell to edit in order to get our desired letter
            let mut best_score: Option<(usize, &u8, usize)> = None;
            for (i, init) in initializers.iter().enumerate() {
                let score = init.abs_diff(*char) as u32 + pointer.abs_diff(i) as u32;
                if let Some(old) = best_score {
                    if score < old.1.abs_diff(*char) as u32 + old.2.abs_diff(i) as u32 {
                        best_score = Some((i, init, pointer))
                    }
                } else {
                    best_score = Some((i, init, pointer))
                }
            }
            let target = match best_score {
                Some(t) => (t.0, t.1),
                None => continue,
            };
            if pointer > target.0 {
                shortest_code += &"<".repeat(pointer.abs_diff(target.0))
            } else {
                shortest_code += &">".repeat(pointer.abs_diff(target.0))
            }
            pointer = target.0;
            if target.1 > char {
                shortest_code += &"-".repeat(target.1.abs_diff(*char).into())
            } else {
                shortest_code += &"+".repeat(target.1.abs_diff(*char).into())
            }
            shortest_code += ".";
            initializers[pointer] = *char;
        }
        if let Some(ref current)=absolute_best{
            if shortest_code.len()<current.len(){
                absolute_best=Some(shortest_code);
                best_mem=mem
            }
        }else{
            absolute_best=Some(shortest_code);
            best_mem=mem
        }
    }
    (absolute_best.unwrap_or_default(),best_mem)
}

fn gen_init_code(initializers: &[u8], divisor: u8) -> String {
    let mut remainders = Vec::new();
    let mut divided = Vec::new();
    for init in initializers {
        divided.push(init / divisor);
        remainders.push(init % divisor);
    }
    let mut code = "+".repeat(divisor as usize);
    code += "[-";
    for cell_goal in divided.iter() {
        code += ">";
        code += &"+".repeat(*cell_goal as usize);
    }
    code += &"<".repeat(initializers.len());
    code += "]";
    for remainder in remainders {
        code += ">";
        code += &"+".repeat(remainder as usize)
    }
    code
}
