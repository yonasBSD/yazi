use yazi_macro::render;
use yazi_proxy::ManagerProxy;
use yazi_shared::event::{Cmd, Data};

use crate::spot::Spot;

struct Opt {
	step: isize,
}

impl From<Cmd> for Opt {
	fn from(c: Cmd) -> Self { Self { step: c.first().and_then(Data::as_isize).unwrap_or(0) } }
}

impl Spot {
	#[yazi_codegen::command]
	pub fn arrow(&mut self, opt: Opt) {
		let Some(lock) = &mut self.lock else { return };

		let Some(old) = lock.selected() else {
			let new = self.skip.saturating_add_signed(opt.step);
			return ManagerProxy::spot(Some(new));
		};

		lock.select(Some(old.saturating_add_signed(opt.step)));
		let new = lock.selected().unwrap();

		self.skip = new;
		render!(new != old);
	}
}