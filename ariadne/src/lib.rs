// In your `proc_macro_crate/src/lib.rs`
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Data, DeriveInput, Fields, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn define_as_grid(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.clone();
    let original = item.clone();
    let input_struct = parse_macro_input!(item as ItemStruct);
    let ast = parse_macro_input!(input as DeriveInput);
    let original_struct = parse_macro_input!(original as DeriveInput);
    let struct_data = if let Data::Struct(data_struct) = ast.data {
        data_struct
    } else {
        panic!("Ariadne can only be applied to structs");
    };
    let fields = if let Fields::Named(fields_named) = struct_data.fields {
        fields_named.named
    } else {
        panic!("Ariadne only supports structs with named fields");
    };
    let required_field_found = fields
        .iter()
        .any(|field| field.ident.as_ref().map_or(false, |ident| ident == "id"));

    if !required_field_found {
        panic!("Struct must contain a field named 'id'");
    }
    // TODO: similarly, we should check that the id field is an i64

    let original_name = input_struct.ident;
    let derived_name = syn::Ident::new(&format!("{}Grid", original_name), original_name.span());

    let exp = quote! {
        #original_struct

        #[derive(Debug)]
        pub struct #derived_name {
            pub entities: HashMap<i64, #original_name>,
            pub map: Array2D<Option<i64>>
        }

        impl #derived_name {
            fn new(width: usize, height: usize) -> Self {
                return #derived_name {
                    entities: HashMap::new(),
                    map: Array2D::filled_with(None, width, height)
                }
            }

            fn add(&mut self, entity: &#original_name, x: usize, y: usize) {
                self.entities.insert(entity.id, entity.clone());
                self.map[(x, y)] = Some(entity.id);
            }

            fn update(&mut self, id: i64, entity: &#original_name) {
                let current_entity = self.entities.get(&id);
                if !current_entity.is_some() {
                    return;
                }
                self.entities.insert(id, entity.clone());
            }

            fn remove_by_id(&mut self, id: i64) {
                self.entities.remove(&id);
                let mut remove_x: Option<usize> = None;
                let mut remove_y: Option<usize> = None;
                for (idx, row_iter) in self.map.rows_iter().enumerate() {
                    for (idy, id_in_map) in row_iter.enumerate() {
                        if !id_in_map.is_some() {
                            continue;
                        }
                        if id_in_map.unwrap() == id {
                            remove_x = Some(idx);
                            remove_y = Some(idy);
                        }
                    }
                }
                if remove_x.is_some() && remove_y.is_some() {
                    self.map[(remove_x.unwrap(), remove_y.unwrap())] = None;
                }
            }

            fn remove_by_position(&mut self, x: usize, y: usize) {
                let id_to_remove = self.map[(x, y)];
                if !id_to_remove.is_some() {
                    return;
                }
                self.entities.remove(&id_to_remove.unwrap());
                self.map[(x, y)] = None;
            }
            fn get_by_id(&mut self, id: i64) -> Option<&#original_name> {
                return self.entities.get(&id);
            }
            fn get_by_position(&mut self, x: usize, y: usize) -> Option<&#original_name> {
                let id_to_find = self.map[(x, y)];
                if !id_to_find.is_some() {
                    return None;
                }
                return self.entities.get(&id_to_find.unwrap());
            }


            /*
            filter closure takes two params:
            let filtered_map: HashMap<&str, i32> = original_map
        .iter() // Iterate over key-value pairs by reference
        .filter(|&(_, &value)| value > 3) // Filter based on the value
        .map(|(&key, &value)| (key, value)) // Map references back to owned values
        .collect(); // Collect into a new HashMap


             */

            /*
                here is how to take in a closure:
                fn apply_operation<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32, // The closure takes an i32 and returns an i32
{
    operation(value)
}

fn main() {
    let result = apply_operation(10, |x| x * 2); // Pass a closure that doubles the value
    println!("Result: {}", result); // Output: Result: 20

    let add_five = |x| x + 5;
    let another_result = apply_operation(7, add_five); // Pass a named closure
    println!("Another Result: {}", another_result); // Output: Another Result: 12
}
             */
            // fn find_by_value<F>(&mut self, operation: F) -> ()//Option<#original_name> 
            // where
            //     F: Fn(&#original_name) -> bool,
            // {
            //     let filtered_items = self.entities.iter().filter(|(&i64, &MyEntity)| {
            //         return true;//operation(value);
            //     }).collect();
            //     println!("filtered items: {:?}", filtered_items);
            //     return ();
            // }

            // TODO: just add the closure now...
            fn find_simple(&mut self) -> Vec<&#original_name> {
                let mut found: Vec<&#original_name> = vec!();
                let filtered_items = self.entities.iter().filter(|(_, v)| true).for_each(|(_, v)| {
                    found.push(v);
                });
                return found;
            }

        }
    };

    exp.into()
}
