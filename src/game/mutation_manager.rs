use bevy::prelude::*;

use crate::app_state::AppState;

pub struct MutationManagerPlugin;

impl Plugin for MutationManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(mutation_manager_setup.in_schedule(OnEnter(AppState::InGame)))
            .insert_resource(MutationManager::new())
            .add_system(mutation_manager_cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Debug, PartialEq, Clone, Resource)]
pub struct MutationManager {
    pub all_mutations: Vec<Mutation>,
    pub player_mutations: Vec<Mutation>,
    pub player_mutations_version: u32,
}

impl MutationManager {
    fn new() -> Self {
        let all_mutations = vec![
            Mutation::new(MutationType::Hemophilia, 0),
            Mutation::new(MutationType::HeavyBones, 1),
            Mutation::new(MutationType::DrySkin, 2),
            Mutation::new(MutationType::Rage, 3),
            Mutation::new(MutationType::BrittleBones, 4),
            Mutation::new(MutationType::Vampirism, 5),
            Mutation::new(MutationType::Dizziness, 6),
            Mutation::new(MutationType::Cyclone, 7),
            Mutation::new(MutationType::Repulsion, 8),
            Mutation::new(MutationType::PoisonBlood, 9),
            Mutation::new(MutationType::Drowziness, 10),
            Mutation::new(MutationType::Lasers, 11),
            Mutation::new(MutationType::Shrink, 12),
            Mutation::new(MutationType::Grow, 13),
            Mutation::new(MutationType::Reflect, 14),
            Mutation::new(MutationType::RubberBody, 15),
            Mutation::new(MutationType::BowlingBall, 16),
        ];

        Self {
            all_mutations,
            player_mutations: Vec::new(),
            player_mutations_version: 0,
        }
    }

    pub fn has_mutation(&self, mutation_type: MutationType) -> bool {
        self.player_mutations
            .iter()
            .any(|m| m.mutation_type == mutation_type)
    }

    pub fn add_mutation(&mut self, mutation_type: MutationType) {
        if !self.has_mutation(mutation_type) {
            if let Some(mutation) = self
                .all_mutations
                .iter()
                .find(|m| m.mutation_type == mutation_type)
            {
                self.player_mutations.push(mutation.clone());
                self.player_mutations_version += 1;
            }
        }
    }

    pub fn remove_mutation(&mut self, mutation_type: MutationType) {
        self.player_mutations
            .retain(|m| m.mutation_type != mutation_type);
        self.player_mutations_version += 1;
    }

    pub fn unselected_mutations(&self) -> Vec<Mutation> {
        self.all_mutations
            .iter()
            .filter(|m| !self.has_mutation(m.mutation_type))
            .cloned()
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MutationType {
    Hemophilia,
    HeavyBones,
    DrySkin,
    Rage,
    BrittleBones,
    Vampirism,
    Dizziness,
    Cyclone,
    Repulsion,
    PoisonBlood,
    Drowziness,
    Lasers,
    Shrink,
    Grow,
    Reflect,
    RubberBody,
    BowlingBall,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mutation {
    pub mutation_type: MutationType,
    pub icon_index: usize,
}

impl Mutation {
    fn new(mutation_type: MutationType, icon_index: usize) -> Self {
        Self {
            mutation_type,
            icon_index,
        }
    }
}

pub fn mutation_manager_setup(_commands: Commands) {}

fn mutation_manager_cleanup(_commands: Commands, mut mutation_manager: ResMut<MutationManager>) {
    mutation_manager.player_mutations.clear();
}
