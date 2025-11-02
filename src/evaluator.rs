use crate::{Environment, Rule, Value, parser::parse_value};

pub fn evaluate_expression(pair: pest::iterators::Pair<Rule>, env: &Environment) -> Value {
    let mut result = String::new();
    let mut is_string = false;

    for term_pair in pair.into_inner() {
        if term_pair.as_rule() == Rule::TERM {
            let value = parse_value(term_pair, env);
            match value {
                Value::String(s) => {
                    result.push_str(&s);
                    is_string = true;
                }
                Value::Number(n) => {
                    result.push_str(&n.to_string());
                }
            }
        }
    }

    if is_string {
        Value::String(result)
    } else {
        Value::Number(result.parse().unwrap_or(0))
    }
}
