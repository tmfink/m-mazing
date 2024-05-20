use m_mazing_core::bevy::{
    ecs::{
        archetype::Archetypes,
        component::Components,
        entity::{Entities, Entity},
        system::{Query, Res},
    },
    hierarchy::Parent,
    input::{keyboard::KeyCode, Input},
};

use crate::*;

fn print_entity(
    level: usize,
    all_entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
    entities: &[(Entity, &Parent)],
    entity: Entity,
) {
    let indent = 4 * level;
    println!("{:indent$}{entity:?}", "");
    println!("{:indent$}  {}", "", 42);
    if let Some(entity_location) = all_entities.get(entity) {
        if let Some(archetype) = archetypes.get(entity_location.archetype_id) {
            for component in archetype.components() {
                if let Some(info) = components.get_info(component) {
                    println!("{:indent$}  -> Component: {}", "", info.name());
                }
            }
        }
    }
    entities
        .iter()
        .filter_map(|(child, parent)| {
            if parent.get() == entity {
                Some(*child)
            } else {
                None
            }
        })
        .for_each(|child| {
            print_entity(
                level + 1,
                all_entities,
                archetypes,
                components,
                entities,
                child,
            )
        });
}

pub fn debug_entity(
    keyboard_input: Res<Input<KeyCode>>,
    root_entities: Query<Entity, Without<Parent>>,
    entities_with_parent: Query<(Entity, &Parent)>,
    all_entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    if !keyboard_input.just_pressed(KeyCode::F1) {
        return;
    }

    let entities: Vec<_> = entities_with_parent.iter().collect();

    println!("DEBUG entities:");
    for entity in root_entities.iter() {
        print_entity(0, all_entities, archetypes, components, &entities, entity);
    }
}
