use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ast::expressions::tables;
use interpreter::{self, environment, types};

type TableHashMap = HashMap<types::Type, Rc<RefCell<types::Type>>>;

fn update_table_border(table: &TableHashMap, key: &types::Type, border: &mut usize) {
    if let types::Type::Number(keynum) = key {
        // If we've filled element at border, our border moved
        if (*border + 1) as f64 == *keynum {
            loop {
                *border = *border + 1;
                if !table.contains_key(&types::Type::Number(*border as f64)) {
                    break
                }
            }
        }
    } else {
        panic!("Not a number as a key for table. Something is terribly wrong");
    }
}

impl interpreter::Eval for tables::Table {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        let mut map: TableHashMap = HashMap::new();
        let mut border: usize = 0;

        for ref field_expression in &self.0 {
            if let types::Type::Vector(mut key_value) = field_expression.eval(env) {
                // Key AND value
                if key_value.len() == 2 {
                    let value = key_value.pop().unwrap();
                    let key = key_value.pop().unwrap();

                    if let types::Type::Number(_) = key {
                        update_table_border(&map, &key, &mut border);
                    }

                    map.insert(key, Rc::new(RefCell::new(value)));
                // Only key
                } else if key_value.len() == 1 {
                    let mut key: types::Type;
                    let value = key_value.pop().unwrap();

                    loop {
                        border = border + 1;
                        key = types::Type::Number(border as f64);

                        if !map.contains_key(&key) {
                            break;
                        }
                    }


                    update_table_border(&map, &key, &mut border);
                    map.insert(key, Rc::new(RefCell::new(value)));
                } else {
                    panic!("Internal interpreter error. Table constructor returns invalid number of elements: {}", key_value.len());
                }
            } else {
                panic!("Internal interpreter error. Table constructor returns not a vector");
            }
        }

        types::Type::Table {
            id: env.next_global_id(),
            map,
            metatable: HashMap::new(),
            border
        }
    }
}

impl interpreter::Eval for tables::TableField {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        let mut result_vector: Vec<types::Type> = vec![];

        if let Some(ref expression) = self.key {
            result_vector.push(expression.eval(env));
        }

        result_vector.push(self.value.eval(env));

        types::Type::Vector(result_vector)
    }
}

impl interpreter::Eval for tables::Indexing {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        if let types::Type::Table { ref map, .. } = self.object.eval(env).unwrap() {
            let key = self.index.eval(env);

            if let Some(result) = map.get(&key) {
                types::Type::Reference(result.clone())
            } else {
                self.runtime_error(format!("Can't find {:?} key in a table", key))
            }
        } else {
            self.runtime_error(format!("Indexing requested, but object is not a table, but {:?}", self.object))
        }
    }
}