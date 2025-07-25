use anyhow::Result;
use yazi_actor::Ctx;
use yazi_boot::BOOT;
use yazi_macro::act;
use yazi_parser::{VoidOpt, mgr::CdSource};
use yazi_shared::{event::Data, url::Url};

use crate::app::App;

impl App {
	pub fn bootstrap(&mut self, _: VoidOpt) -> Result<Data> {
		for (i, file) in BOOT.files.iter().enumerate() {
			let tabs = &mut self.core.mgr.tabs;
			if tabs.len() <= i {
				tabs.push(Default::default());
			}

			let cx = &mut Ctx::active(&mut self.core);
			cx.tab = i;

			if file.is_empty() {
				act!(mgr:cd, cx, (Url::from(&BOOT.cwds[i]), CdSource::Tab))?;
			} else {
				act!(mgr:reveal, cx, (Url::from(BOOT.cwds[i].join(file)), CdSource::Tab))?;
			}
		}

		act!(render, self)
	}
}
