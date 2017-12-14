use super::*;

pub fn exp_box<E: expression::Expression + 'static>(expression: E) -> Box<expression::Expression> {
    log_debug!("Made expression: {:?}", expression);
    Box::new(expression)
}
