use ariadne::define_as_grid;
use std::collections::HashMap;
use array2d::Array2D;
use ariadne;

#[derive(Debug, Clone)]
#[define_as_grid]
pub struct MyEntity {
    id: i64,
    name: String
}

fn main() {
    let mut world_grid = MyEntityGrid::new(3, 3);
    let some_ent = MyEntity {
        id: 1,
        name: "foo".to_string()
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
    // work with entity returned by position
    let mut ent = world_grid.get_by_position(2, 2).unwrap().clone();
    // update its data
    ent.name = "pumba".to_string();
    world_grid.update(1, &ent);
    // show that update worked
    println!("entity found after adding again: {:?}", world_grid.get_by_position(2, 2));
}
