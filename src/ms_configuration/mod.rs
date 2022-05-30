pub mod nodes;

pub struct MsUser
{
	login: String,
	password: String
}

impl MsUser
{
	pub fn new(login: String, password: String) -> Self
	{
		MsUser
		{
			login: login,
			password: password
		}
	}

	pub fn login(&self) -> &str
	{
		&self.login
	}

	pub fn password(&self) -> &str
	{
		&self.password
	}
}

pub struct MsConfiguration
{
	pub server_api_url: String,
	pub account_id: String,
	pub def_page_size: u32,
	pub users: Vec<MsUser>
}

impl MsConfiguration
{


	pub fn new(account_id: String, users: Vec<MsUser>) -> Self
	{
		MsConfiguration
		{
			def_page_size: 50,
			server_api_url: "https://online.moysklad.ru/api/remap/1.2".to_string(),
			account_id,
			users
		}
	}
}