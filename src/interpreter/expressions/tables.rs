use std::cell::RefCell;
use std::clone::Clone;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::ast::expressions::tables;
use crate::interpreter::{self, environment, types};
use crate::utils;

type TableHashMap = HashMap<types::Type, Rc<RefCell<types::Type>>>;

fn update_table_border(table: &TableHashMap, border: &mut usize) {
    while table.contains_key(&types::Type::Number((*border + 1) as f64)) {
        *border += 1;
    }
}

impl interpreter::Eval for tables::Table {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let mut map: TableHashMap = HashMap::new();
        let mut border: usize = 0;

        for field_expression in &self.0 {
            if let types::Type::Vector(mut key_value) = field_expression.eval(env) {
                // Key AND value
                if key_value.len() == 2 {
                    let value = key_value.pop_back().unwrap();
                    let key = key_value.pop_back().unwrap();

                    map.insert(key, Rc::new(RefCell::new(value)));
                    update_table_border(&map, &mut border);
                // Only key
                } else if key_value.len() == 1 {
                    let mut key: types::Type;
                    let value = key_value.pop_back().unwrap();

                    loop {
                        border += 1;
                        key = types::Type::Number(border as f64);

                        if !map.contains_key(&key) {
                            break;
                        }
                    }

                    map.insert(key, Rc::new(RefCell::new(value)));
                    update_table_border(&map, &mut border);
                } else {
                    panic!("Internal interpreter error. Table constructor returns invalid number of elements: {}", key_value.len());
                }
            } else {
                panic!("Internal interpreter error. Table constructor returns not a vector");
            }
        }

        types::Type::Table {
            id: env.borrow_mut().next_global_id(),
            map,
            metatable: HashMap::new(),
            border,
        }
    }
}

impl interpreter::Eval for tables::TableField {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let mut result_vector: VecDeque<types::Type> = VecDeque::new();

        if let Some(ref expression) = self.key {
            let key = expression.eval(env);

            if key.is_nil() {
                self.runtime_error("Cannot use `nil` as a table key".to_string())
            }

            result_vector.push_back(key);
        }

        result_vector.push_back(self.value.eval(env));

        types::Type::Vector(result_vector)
    }
}

impl interpreter::Eval for tables::Indexing {
    fn eval(&self, env: &mut utils::Shared<environment::Environment>) -> types::Type {
        let table = self.object.eval(env).into_reference();
        // This is bullshit. WTF Ref
        let mut table_borrow = table.borrow_mut();

        // We can index only tables
        if let types::Type::Table {
            ref mut map,
            ref mut border,
            id,
            ..
        } = *table_borrow
        {
            if let Some(cached_value) = self.cache.borrow().get(id) {
                return cached_value
            }

            let key = self.index.eval(env);

            // If we have this entry, return reference to it
            return if let Some(result) = map.get(&key).cloned() {
                self.cache.borrow_mut().set(id, &result);

                types::Type::Reference(result)
            // If we have no such entry in the table, we add new entry with Nil value
            // In case of chaind indexing, we must get an error about indexing Nil value
            } else {
                let new_entry = Rc::new(RefCell::new(types::Type::Nil));
                self.cache.borrow_mut().set(id, &new_entry);

                map.insert(key, new_entry.clone());
                update_table_border(&map, border);

                types::Type::Reference(new_entry)
            };
        }

        // Because of NLL
        self.runtime_error(format!("Attempt to index `{}` value, not a table", table_borrow))
    }
}
