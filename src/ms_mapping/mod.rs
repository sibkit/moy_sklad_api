use crate::ms_configuration::nodes::MsNode;

pub trait MsMapper<'a, T>
{
	fn bind_to_entity(obj: String, entity: T);
	fn create_new_entity() -> T;

	fn set_node(node: &Box<dyn MsNode<'a>>);
}