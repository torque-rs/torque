use test_log::test;

use crate::{Component, Entity, EntityMethods, EntityRef};

use super::System;

#[derive(Clone, Entity)]
struct TestEntity;

#[derive(Clone, Entity)]
#[extends(TestEntity)]
struct TestEntity2;

#[derive(Clone, Entity)]
struct TestEntity3;

struct TestComponent(#[allow(dead_code)] usize);

impl Component for TestComponent {
	const NAME: &str = "TestComponent";

	type Value = Self;
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

	entity.set::<TestComponent>(TestComponent(0));
}

#[test]
fn entity_cast() {
	let system = System::default();

	let entity = system.create::<TestEntity2>();

	let entity: EntityRef<TestEntity> = entity.upcast::<TestEntity>();
	entity.downcast::<TestEntity2>();
}
