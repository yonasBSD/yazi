use std::collections::HashMap;

use tracing::error;
use yazi_macro::render;
use yazi_shared::{event::CmdCow, fs::Url};

use crate::{manager::{LINKED, Manager}, tasks::Tasks};

pub struct Opt {
	updates: HashMap<String, String>,
}

impl TryFrom<CmdCow> for Opt {
	type Error = ();

	fn try_from(mut c: CmdCow) -> Result<Self, Self::Error> {
		Ok(Self { updates: c.try_take("updates").ok_or(())?.into_dict_string() })
	}
}

impl Manager {
	pub fn update_mimes(&mut self, opt: impl TryInto<Opt>, tasks: &Tasks) {
		let Ok(opt) = opt.try_into() else {
			return error!("invalid arguments for update_mimes");
		};

		let linked = LINKED.read();
		let updates = opt
			.updates
			.into_iter()
			.map(|(url, mime)| (Url::from(url), mime))
			.filter(|(url, mime)| self.mimetype.get(url) != Some(mime))
			.fold(HashMap::new(), |mut map, (u, m)| {
				for u in linked.from_file(&u) {
					map.insert(u, m.clone());
				}
				map.insert(u, m);
				map
			});

		drop(linked);
		if updates.is_empty() {
			return;
		}

		let affected: Vec<_> = self
			.current()
			.paginate(self.current().page)
			.iter()
			.filter(|&f| updates.contains_key(&f.url))
			.cloned()
			.collect();

		let repeek = self.hovered().is_some_and(|f| updates.contains_key(&f.url));
		self.mimetype.extend(updates);

		if repeek {
			self.peek(false);
		}
		tasks.prework_affected(&affected, &self.mimetype);
		render!();
	}
}
