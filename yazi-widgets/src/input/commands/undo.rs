use yazi_macro::render;
use yazi_shared::event::CmdCow;

use crate::input::{Input, InputMode};

impl Input {
	pub fn undo(&mut self, _: CmdCow) {
		if !self.snaps.undo() {
			return;
		}

		self.r#move(0);
		if self.snap().mode == InputMode::Insert {
			self.escape(());
		}

		render!();
	}
}
