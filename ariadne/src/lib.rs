// In your `proc_macro_crate/src/lib.rs`
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use std::{any::{Any, TypeId}, collections::HashMap};
use syn::{parse::Parser, parse_macro_input, parse_quote, parse_str, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields, ItemStruct, Type};
use uuid::{Uuid};

#[proc_macro_attribute]
pub fn define_as_grid(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = item.clone();
    let original = item.clone();
    let input_struct = parse_macro_input!(item as ItemStruct);
    let ast = parse_macro_input!(input as DeriveInput);
    let mut original_struct = parse_macro_input!(original as DeriveInput);
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
    let id_field_found = validate_field(&fields, "id".to_string(), "Uuid".to_string());
    let x_field_found = validate_field(&fields, "x".to_string(), "usize".to_string());
    let y_field_found = validate_field(&fields, "y".to_string(), "usize".to_string());

        
    // cool way to add fields to the struct. Unfortunately we don't really need this. Keeping it here for reference for now
    // match &mut original_struct.data {
    //     syn::Data::Struct(ast_struct_data) => {
    //                     if let syn::Fields::Named(fields) = &mut ast_struct_data.fields {
    //                         // Create the new field
    //                         let x_field: syn::Field = syn::Field::parse_named
    //                             .parse2(quote! { pub x: usize })
    //                             .unwrap();
    //                         let y_field: syn::Field = syn::Field::parse_named
    //                             .parse2(quote! { pub y: usize })
    //                             .unwrap();
        
    //                         // Add the new field to the struct
    //                         fields.named.push(x_field);
    //                         fields.named.push(y_field);
    //                     }
    //         }
    //     Data::Struct(data_struct) => todo!(),
    //     Data::Enum(data_enum) => todo!(),
    //     Data::Union(data_union) => todo!(),
    // }

    if !id_field_found {
        panic!("Struct must contain a field named 'id' of type Uuid");
    }

    if !x_field_found {
        panic!("Struct must contain a field named 'x' of type usize");
    }

    if !y_field_found {
        panic!("Struct must contain a field named 'y' of type usize");
    }
    let original_name = input_struct.ident;
    let derived_name = syn::Ident::new(&format!("{}Grid", original_name), original_name.span());

    let exp = quote! {
        #[derive(Debug, Clone)]
        #original_struct

        #[derive(Debug)]
        pub struct #derived_name {
            pub entities: HashMap<Uuid, #original_name>,
            pub map: Array2D<Option<Uuid>>
        }

        impl #derived_name {
            fn new(width: usize, height: usize) -> Self {
                return #derived_name {
                    entities: HashMap::new(),
                    map: Array2D::filled_with(None, width, height)
                }
            }

            fn add(&mut self, entity: &#original_name) {
                self.entities.insert(entity.id, entity.clone());
                self.map[(entity.x, entity.y)] = Some(entity.id);
            }

            fn update_by_id<F>(&mut self, id: Uuid, updater: F)
            where
                F: Fn(&#original_name) -> #original_name {
                let current_entity = self.entities.get(&id);
                if !current_entity.is_some() {
                    return;
                }
                let updated = updater(current_entity.unwrap());
                if updated.x != current_entity.unwrap().x || updated.y != current_entity.unwrap().y {
                    self.map[(current_entity.unwrap().x, current_entity.unwrap().y)] = None;
                    self.map[(updated.x, updated.y)] = Some(updated.id);
                }
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
                if updated.x != current_entity.unwrap().x || updated.y != current_entity.unwrap().y {
                    self.map[(current_entity.unwrap().x, current_entity.unwrap().y)] = None;
                    self.map[(updated.x, updated.y)] = Some(updated.id);
                }
                self.entities.insert(id_to_use.unwrap(), updated);
            }

            fn remove_by_id(&mut self, id: Uuid) {
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

            fn get_by_id(&mut self, id: Uuid) -> Option<#original_name> {
                let found = self.entities.get(&id);
                if found.is_some() {
                    return Some(found.unwrap().clone());
                }
                return None;
            }

            fn get_by_position(&mut self, x: usize, y: usize) -> Option<#original_name> {
                let id_to_find = self.map[(x, y)];
                if !id_to_find.is_some() {
                    return None;
                }
                let found = self.entities.get(&id_to_find.unwrap());
                if found.is_some() {
                    return Some(found.unwrap().clone());
                }
                return None;
            }

            fn find_by_value<F>(&mut self, filter_func: F) -> Vec<#original_name> 
            where
                F: Fn(&#original_name) -> bool,
            {
                let mut found: Vec<#original_name> = vec!();
                let filtered_items = self.entities.iter().filter(|(_, v)| filter_func(v)).for_each(|(_, v)| {
                    found.push(v.clone());
                });
                return found;
            }

        }
    };

    exp.into()
}

fn validate_field(fields: &Punctuated<Field, Comma>, field_name: String, field_type: String) -> bool {
    let required_field_found = fields
        .iter()
        .any(|field| field.ident.as_ref().map_or(false, |ident| *ident == field_name) && match &field.ty {
            // make sure the field is Uuid
            Type::Path(type_path) => {
                if type_path.path.is_ident(&field_type) {
                    return true;
                }
                return false;
            },
            _ => {
                return false;
            }
        });
        return required_field_found;
}
