use pg_core::impl_repository;

use crate::entity::{prelude::Prompt, prompt};

impl_repository!(PromptRepo, Prompt, prompt::Model);
