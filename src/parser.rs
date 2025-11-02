use crate::{
    Environment, Rule, Value, chinese_number::chinese_to_number, evaluator::evaluate_expression,
};

pub fn parse_value(pair: pest::iterators::Pair<Rule>, env: &Environment) -> Value {
    match pair.as_rule() {
        Rule::VALUE => {
            // VALUE wraps EXPRESSION
            let inner = pair.into_inner().next().unwrap();
            parse_value(inner, env)
        }
        Rule::EXPRESSION => evaluate_expression(pair, env),
        Rule::TERM => {
            let inner = pair.into_inner().next().unwrap();
            parse_value(inner, env)
        }
        Rule::NUMBER => {
            let num_str = pair.as_str();
            let first_char = num_str.chars().next().unwrap();
            if first_char as u32 > 127 {
                let num = chinese_to_number(num_str)
                    .unwrap_or_else(|| panic!("无效的中文数字: {}", num_str));
                Value::Number(num)
            } else {
                Value::Number(num_str.parse().unwrap())
            }
        }
        Rule::STRING => {
            let s = pair.as_str();
            Value::String(s[1..s.len() - 1].to_string())
        }
        Rule::VAR_NAME => env
            .get_var(pair.as_str())
            .cloned()
            .unwrap_or_else(|| panic!("未定义变量: {}", pair.as_str())),
        _ => panic!("Unexpected rule in parse_value: {:?}", pair.as_rule()),
    }
}
