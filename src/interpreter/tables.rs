use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ast::expressions::tables;
use interpreter::{self, environment, types};

type TableHashMap = HashMap<types::Type, Rc<RefCell<types::Type>>>;

fn update_table_border(table: &TableHashMap, border: &mut usize) {
    while table.contains_key(&types::Type::Number((*border + 1) as f64)) {
        *border = *border + 1;
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

                    map.insert(key, Rc::new(RefCell::new(value)));
                    update_table_border(&map, &mut border);
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
            match expression.eval(env) {
                types::Type::Id(id) => {
                    if let Some (id_value) = env.get(&id) {
                        result_vector.push(types::Type::Reference(id_value))
                    } else {
                        self.runtime_error(format!("Unknown variable '{}'", id))
                    }
                }
                key => result_vector.push(key)
            }
        }

        result_vector.push(self.value.eval(env));

        types::Type::Vector(result_vector)
    }
}

impl interpreter::Eval for tables::Indexing {
    fn eval(&self, env: &mut environment::Environment) -> types::Type {
        let table = self.object.eval(env).as_ref();
        // This is bullshit. WTF Ref
        let mut table_borrow = table.borrow_mut();

        // We can index only tables
        if let types::Type::Table { ref mut map, ref mut border, .. } = *table_borrow {
            let key = self.index.eval(env);

            // If we have this entry, return reference to it
            if let Some(result) = map.get(&key).cloned() {
                types::Type::Reference(result)
            // If we have no such entry in the table, we add new entry with Nil value
            // In case of chaind indexing, we must get an error about indexing Nil value
            } else {
                let new_entry = Rc::new(RefCell::new(types::Type::Nil));

                map.insert(key, new_entry.clone());
                update_table_border(&map, border);

                types::Type::Reference(new_entry)
            }
        } else {
            self.runtime_error(format!("Attemp to index not a table, but {}", table.borrow()))
        }
    }
}
