use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::mem;
use std::mem::swap;

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

	pub fn get_entity<T: 'static>(&mut self, index: usize) -> &mut Box<T>
	{
		let type_id = TypeId::of::<T>();
		let map = &mut self.entities_map;
		let vec = map.get_mut(&type_id).unwrap();

		let mut any_e = vec.get_mut(index).unwrap();
		unsafe
		{
			let tr_result = mem::transmute::<&mut Box<dyn Any>, &mut Box<T>>(any_e);
			tr_result
		}
	}


	pub fn add_entity<T: 'static +>(&mut self, entity: Box<T>)
	{
		let actual_id = (*entity).type_id();
		let entities;
		match self.entities_map.get_mut(&actual_id)
		{
			Some(val) => {entities = val;}
			None =>
				{
					let new_entities: Vec<Box<dyn Any>> = vec![];
					self.entities_map.insert(actual_id,new_entities);
					entities = self.entities_map.get_mut(&actual_id).unwrap();
				}
		}

		entities.push(entity);
	}
}