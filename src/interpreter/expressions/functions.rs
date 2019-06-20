use std::collections::VecDeque;

use crate::ast::expressions::{self, function};
use crate::interpreter::{self, environment, types};
use crate::utils;

impl interpreter::Eval for function::Closure {
    // pub struct Closure {
    //     pub params: VecDeque<Box<expressions::Expression>>,
    //     pub varargs: bool,
    //     pub body: Rc<Box<expressions::Expression>>,
    // }
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let parameters: Vec<String> = self
            .params
            .iter()
            .map(|exp| {
                if let types::Type::String(string) = exp.eval(env) {
                    string
                } else {
                    self.runtime_error(format!(
                        "Function arguments contains not a string, but {:?}",
                        exp
                    ));
                }
            })
            .collect();

        types::Type::Function {
            id: env.borrow_mut().next_global_id(),
            parameters,
            varargs: self.varargs,
            body: self.body.clone(),
            env: env.clone(),
        }
    }
}

fn eval_args(
    args: &VecDeque<Box<expressions::Expression>>,
    call_env: &mut utils::Shared<environment::Environment>,
) -> VecDeque<types::Type> {
    let mut result = VecDeque::new();

    for arg in args {
        match arg.eval(call_env) {
            types::Type::Vector(vec) => result.extend(vec.into_iter()),
            value => result.push_back(value),
        }
    }

    result
}

fn call_function(
    this: &expressions::Expression,
    call_object: Option<types::Type>,
    function: types::Type,
    args: &VecDeque<Box<expressions::Expression>>,
    call_env: &mut utils::Shared<environment::Environment>,
) -> types::Type {
    match_type!(&function,
        types::Type::Function { parameters, varargs, body, env, .. } => {
            let mut local_env = environment::Environment::new(Some(env.clone()), environment::BreakFlag::Return(None));
            let mut args = eval_args(args, call_env);

            // self
            if let Some(obj) = call_object {
                args.push_front(obj)
            }

            // Bing args to parameters
            for parameter in parameters {
                if let Some(arg) = args.pop_front() {
                    local_env.add_variable(parameter.clone(), arg);
                }
            }

            // Varargs
            if *varargs {
                local_env.add_variable("arg".to_string(), types::Type::Vector(args));
            }

            let mut shared_env = utils::Shared::new(local_env);
            body.eval(&mut shared_env);

            let mut borrow = shared_env.borrow_mut();
            borrow.retval()
        },
        _ => this.runtime_error(format!("Cannot call {:?}, not a function", function))
    )
}

impl interpreter::Eval for function::Funcall {
    // pub struct Funcall {
    //     pub object: Box<expressions::Expression>,
    //     pub args: VecDeque<Box<expressions::Expression>>,
    //     pub method: Option<Box<expressions::Expression>>,
    // }
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let call_object = self.object.eval(env);

        // First let check if we have method name, in this case we expect table on method inside
        if let Some(ref method_exp) = self.method {
            let method_name = method_exp.eval(env);

            match_type!(&method_name,
                types::Type::String(_) => {
                    // This must be a table, because we call its method
                    match_type!(&call_object,
                        types::Type::Table{ ref map, .. } => {
                            if let Some(method) = map.get(&method_name).cloned() {
                                call_function(self, Some(call_object), types::Type::Reference(method), &self.args, env)
                            } else {
                                self.runtime_error(format!("Object doesn't contain method {:?}", method_name))
                            }
                        },
                        _ => self.runtime_error(format!("Method call object is not a table, but {:?}", call_object))
                    )
                },
                _ => self.runtime_error(format!("Method call method name is not a string, but {:?}", method_name))
            )
        // Function
        } else {
            call_function(self, None, call_object, &self.args, env)
        }
    }
}
