use std::{
	any::TypeId,
	ops::Deref,
	sync::{LazyLock, Once},
};
use test_log::test;

use crate::{Component, Entity, EntityRef};

use super::System;

#[derive(Clone)]
struct TestEntity(EntityRef);

impl Deref for TestEntity {
	type Target = EntityRef;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

static TYPE_ID: LazyLock<TypeId> = LazyLock::new(TypeId::of::<TestEntity>);
static TYPE_IDS: LazyLock<[TypeId; 1]> = LazyLock::new(|| [*TYPE_ID]);

impl Entity for TestEntity {
	const NAME: &'static str = "TestEntity";

	fn type_id() -> TypeId {
		*TYPE_ID
	}

	fn type_ids() -> &'static [TypeId] {
		&*TYPE_IDS
	}

	fn new(entity_ref: EntityRef) -> Self {
		Self(entity_ref)
	}
}

struct TestComponent(usize);

impl Component for TestComponent {
	const NAME: &str = "TestComponent";
}

#[test]
fn system_create() {
	let system = System::default();

	system.create::<TestEntity>();
}

#[test]
fn system_create_and_set() {
	let system = System::default();

	let entity = system.create::<TestEntity>();

	entity.set(TestComponent(0));
}
