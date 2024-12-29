use std::{
	collections::VecDeque,
	ops::{Deref, DerefMut},
};

use super::NodeRef;

#[derive(Debug, Default)]
pub struct Children(VecDeque<NodeRef>);

impl Deref for Children {
	type Target = VecDeque<NodeRef>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Children {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
