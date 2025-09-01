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

            fn update_by_id<F>(&mut self, id: i64, updater: F)
            where
                F: Fn(&#original_name) -> #original_name {
                let current_entity = self.entities.get(&id);
                if !current_entity.is_some() {
                    return;
                }
                let updated = updater(current_entity.unwrap());
                self.entities.insert(id, updated);
            }

            fn update_by_position<F>(&mut self, x: usize, y: usize, updater: F)
            where
                F: Fn(&#original_name) -> #original_name {
                let id_to_use = self.map[(x, y)];
                if !id_to_use.is_some() {
                    return;
                }
                let current_entity = self.entities.get(&id_to_use.unwrap());
                if !current_entity.is_some() {
                    return;
                }
                let updated = updater(current_entity.unwrap());
                self.entities.insert(id_to_use.unwrap(), updated);
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

            fn find_by_value<F>(&mut self, filter_func: F) -> Vec<&#original_name> 
            where
                F: Fn(&#original_name) -> bool,
            {
                let mut found: Vec<&#original_name> = vec!();
                let filtered_items = self.entities.iter().filter(|(_, v)| filter_func(v)).for_each(|(_, v)| {
                    found.push(v);
                });
                return found;
            }

        }
    };

    exp.into()
}
