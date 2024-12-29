use torque_ecs::{Entity, EntityMethods, EntityRef, WeakEntityRef};
use torque_style::{Layout, MaxSize, MinSize, Resolve, Size, Style};

use crate::{layout, Element, Parent};

pub trait NodeMethods: EntityMethods {
	fn parent(&self) -> Option<WeakEntityRef<Element>> {
		self.get_or_default::<Parent>()
	}

	fn with_style<R>(&self, f: impl FnOnce(&Style) -> R) -> R {
		self.with_or_default::<Style, _>(f)
	}

	fn with_style_mut<R>(&self, f: impl FnOnce(&mut Style) -> R) -> R {
		self.with_mut_or_default::<Style, _>(f)
	}

	fn compute_layout(&self, input: layout::Input) -> layout::Output {
		let (layout, size, min_size, max_size) = self.with_style(|style| {
			(
				style.get_or_default::<Layout>(),
				style.get_or_default::<Size>(),
				style.get_or_default::<MinSize>(),
				style.get_or_default::<MaxSize>(),
			)
		});

		let size = size.resolve(input.parent_size);
		let min_size = min_size.resolve(input.parent_size);
		let max_size = max_size.resolve(input.parent_size);

		match layout {
			Layout::Row => todo!(),
			Layout::Column => todo!(),
		}

		layout::Output { size: todo!() }
	}
}

#[derive(Clone, Entity)]
pub struct Node;

impl NodeMethods for EntityRef<Node> {}
