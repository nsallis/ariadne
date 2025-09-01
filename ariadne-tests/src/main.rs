use ariadne::define_as_grid;
use std::collections::HashMap;
use array2d::Array2D;
use ariadne;

#[derive(Debug, Clone)]
#[define_as_grid]
pub struct MyEntity {
    id: i64,
    name: String,
}

#[cfg(test)]
mod world_grid {
    use super::*; // Import items from the parent module

    #[test]
    fn it_creates_a_grid_and_adds_an_entity() {
        let mut world_grid = MyEntityGrid::new(3, 3);
        let some_ent = MyEntity {
            id: 1,
            name: "foo".to_string(),
        };
        world_grid.add(&some_ent, 1, 1);
        let found_entity = world_grid.get_by_id(1).unwrap();
        assert_eq!(found_entity.name, "foo".to_string());
        assert_eq!(found_entity.id, 1);
    }

    #[test]
    fn it_removes_an_entity_by_id() {
        let mut world_grid = MyEntityGrid::new(3, 3);
        let some_ent = MyEntity {
            id: 1,
            name: "foo".to_string(),
        };
        world_grid.add(&some_ent, 1, 1);
        world_grid.remove_by_id(1);
        let found_entity = world_grid.get_by_id(1);
        assert_eq!(found_entity.is_some(), false);
    }
}

fn main() {
    let mut world_grid = MyEntityGrid::new(3, 3);
    let some_ent = MyEntity {
        id: 1,
        name: "foo".to_string(),
    };
    // add entity
    world_grid.add(&some_ent, 1, 1);
    println!("world grid after add: {:?}", world_grid);
    // remove_by_id
    world_grid.remove_by_id(1);
    println!("world grid after remove by id: {:?}", world_grid);
    // add again with different position
    world_grid.add(&some_ent, 2, 2);
    println!("world grid after add again: {:?}", world_grid);
    // remove by position
    world_grid.remove_by_position(2, 2);
    println!("world grid after remove by position: {:?}", world_grid);

    // add again
    world_grid.add(&some_ent, 1, 1);
    // show that get_by_id works
    println!("entity found after add: {:?}", world_grid.get_by_id(1));
    world_grid.remove_by_id(1);
    // show that get_by_id with non-existent id returns None
    println!("entity found after remove (none): {:?}", world_grid.get_by_id(1));
    world_grid.add(&some_ent, 2, 2);
    // update its data
    world_grid.update_by_id(1, |entity| {
        let mut updated = entity.to_owned();
        // let mut updated = entity.clone();
        updated.name = "pumba".to_string();
        return updated;
    });
    world_grid.update_by_position(2, 2, |entity| {
        let mut updated = entity.to_owned();
        // let mut updated = entity.clone();
        updated.name = "pumba2".to_string();
        return updated;
    });
    // show that update worked
    println!("entity found after adding again: {:?}", world_grid.get_by_position(2, 2));
    let found = world_grid.find_by_value(|entity| entity.name == "pumba".to_string());
    // let found = world_grid.find_by_value(|entity| {entity.name == "pumba".to_string()});
    println!("found by value: {:?}", found);
}
