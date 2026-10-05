use super::{NebulaWorkspace, WorkspaceTab};
use gpui::{Context, IntoElement};

impl NebulaWorkspace {
    fn active_tab_wallpaper_path(&self) -> Option<std::path::PathBuf> {
        (!self.settings_open && self.tabs.get(self.active).is_some_and(WorkspaceTab::is_terminal))
            .then(|| self.meta(self.active).background_image.map(std::path::PathBuf::from))
            .flatten()
    }

    fn prepare_active_tab_wallpaper(
        &self,
        path: Option<&std::path::PathBuf>,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = path {
            crate::gpui_shell::wallpaper::ensure_tab_wallpaper(path.clone(), cx);
        }
    }

    pub(super) fn tab_wallpaper_layer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let path = self.active_tab_wallpaper_path();
        self.prepare_active_tab_wallpaper(path.as_ref(), cx);
        crate::gpui_shell::wallpaper::tab_card_layer(path, cx)
    }

    pub(super) fn window_wallpaper_layer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let path = self.active_tab_wallpaper_path();
        self.prepare_active_tab_wallpaper(path.as_ref(), cx);
        crate::gpui_shell::wallpaper::window_layer(path, cx)
    }
}
