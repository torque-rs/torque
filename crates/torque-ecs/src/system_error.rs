use crate::EntityId;

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
	#[error("entity {0} not found")]
	EntityNotFound(EntityId),
	#[error("component {1} not found for entity {0}")]
	ComponentNotFound(EntityId, &'static str),
	#[error("invalid cast to {1} for entity {0}")]
	InvalidCast(EntityId, &'static str),
}
