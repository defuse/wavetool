#[derive(Debug)]
pub struct FilterArgs {
    pub input_file: String,
    pub output_file: String,
    pub keep_even: bool,
    pub keep_odd: bool,
    pub keep_bitmap: Option<String>,
    pub keep_pattern: Option<String>,
    pub remove_primes: Option<String>,
    pub keep_primes: Option<String>,
    pub protect_fundamental: bool,
    pub normalize: bool
}

pub fn run_filter(args: &FilterArgs) -> () {
    println!("{:?}", args);

}