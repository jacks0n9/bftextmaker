# Bftextmaker
## Usage
```rs
use bftextmaker::gen_code;
let (code,memory_used)=gen_code("The quick brown fox jumps over the lazy dog",15);
println!("Your code is: {}",code);
println!("Used {} cells",memory_used);
```