/// Parse user's input to a handful of options, either directly as a `usize`,
/// or, if they entered Python exponent notation (base**exponent), which
/// we'll need to evaluate as an exponent. Either way, return a `usize`
/// or `expect`/`panic!`.
///  
/// This is useful when making lists fit to a specific amount of dice and
/// dice sides. (As an example, five rolls of a six-sided dice would be: 6**5).
pub fn eval_list_length(input: &str) -> Result<usize, String> {
    match input.split("**").collect::<Vec<&str>>().as_slice() {
        [] => Err("Please specify a number.".to_string()),
        [num_string] => num_string.parse::<usize>().map_err(|_| {
            format!(
                "Unable to parse input {}. Enter a number or a base**exponent",
                input
            )
        }),
        [base_string, exponent_string] => {
            let base: usize = base_string
                .parse::<usize>()
                .map_err(|_| format!("Unable to parse input {}. Positive integers only.", input))?;
            let exponent: u32 = exponent_string
                .parse::<u32>()
                .map_err(|_| format!("Unable to parse input {}. Positive integers only.", input))?;
            Ok(base.pow(exponent))
        }
        _ => Err("You can only specify one exponent! Use format: base**exponent".to_string()),
    }
}
