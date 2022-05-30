use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::collections::HashMap;

pub struct MsDataSet
{
	entities_map: HashMap<TypeId,Vec<Box<dyn Any>>>,
}

impl MsDataSet
{
	pub fn new() -> Self
	{
		Self { entities_map: HashMap::new(), }
	}

	pub fn get_entities(&mut self, type_id: TypeId) -> &mut Vec<Box<dyn Any>>
	{
		let map = self.entities_map.borrow_mut();

		return if map.contains_key(&type_id)
		{
			map.get_mut(&type_id).unwrap()
		} else
		{
			let vec = Vec::new();
			map.insert(type_id.clone(), vec);
			map.get_mut(&type_id).unwrap()
		}
	}

	pub fn add_entity(&mut self, entity: Box<dyn Any>)
	{
		let actual_id = (*entity).type_id();
		let entities = self.get_entities(actual_id);
		entities.push(entity);
	}
}