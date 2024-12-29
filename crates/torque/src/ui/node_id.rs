use slotmap::{new_key_type, KeyData};

new_key_type! {
	pub struct NodeId;
}

impl From<NodeId> for taffy::NodeId {
	fn from(value: NodeId) -> Self {
		Self::new(value.0.as_ffi())
	}
}

impl From<taffy::NodeId> for NodeId {
	fn from(value: taffy::NodeId) -> Self {
		KeyData::from_ffi(value.into()).into()
	}
}
