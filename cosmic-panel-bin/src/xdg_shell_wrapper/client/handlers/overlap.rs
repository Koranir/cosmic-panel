use cctk::{
    cosmic_protocols::overlap_notify::v1::client::{
        zcosmic_overlap_notification_v1::ZcosmicOverlapNotificationV1,
        zcosmic_overlap_notify_v1::{self, ZcosmicOverlapNotifyV1},
    },
    wayland_client::{
        delegate_dispatch,
        globals::{BindError, GlobalList},
        Dispatch, QueueHandle,
    },
};
use sctk::globals::GlobalData;
use tracing::debug;
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::ZwlrLayerSurfaceV1;

use crate::xdg_shell_wrapper::shared_state::GlobalState;

#[derive(Debug)]
pub struct OverlapNotifyManager {
    manager: ZcosmicOverlapNotifyV1,
}
impl OverlapNotifyManager {
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<GlobalState>,
    ) -> Result<Self, BindError> {
        let manager = globals.bind(queue_handle, 1..=1, GlobalData)?;
        Ok(Self { manager })
    }

    pub fn register_layer_shell(
        &self,
        layer_shell: &ZwlrLayerSurfaceV1,
        qh: &QueueHandle<GlobalState>,
    ) -> ZcosmicOverlapNotificationV1 {
        self.manager.notify_on_overlap(layer_shell, qh, ())
    }
}

impl Dispatch<ZcosmicOverlapNotifyV1, GlobalData, GlobalState> for OverlapNotifyManager {
    fn event(
        _state: &mut GlobalState,
        _proxy: &ZcosmicOverlapNotifyV1,
        _event: zcosmic_overlap_notify_v1::Event,
        _data: &GlobalData,
        _conn: &cctk::wayland_client::Connection,
        _qhandle: &cctk::wayland_client::QueueHandle<GlobalState>,
    ) {
        // no events to handle
    }
}

impl Dispatch<ZcosmicOverlapNotificationV1, (), GlobalState> for OverlapNotifyManager {
    fn event(
        state: &mut GlobalState,
        proxy: &ZcosmicOverlapNotificationV1,
        event: <ZcosmicOverlapNotificationV1 as cctk::wayland_client::Proxy>::Event,
        data: &(),
        conn: &cctk::wayland_client::Connection,
        qhandle: &QueueHandle<GlobalState>,
    ) {
        debug!(?event);
        // match event {
        //     cctk::cosmic_protocols::overlap_notify::v1::client::zcosmic_overlap_notification_v1::Event::ToplevelEnter { toplevel, x, y, width, height } => todo!(),
        //     cctk::cosmic_protocols::overlap_notify::v1::client::zcosmic_overlap_notification_v1::Event::ToplevelLeave { toplevel } => todo!(),
        //     cctk::cosmic_protocols::overlap_notify::v1::client::zcosmic_overlap_notification_v1::Event::LayerEnter { identifier, exclusive, layer, x, y, width, height } => todo!(),
        //     cctk::cosmic_protocols::overlap_notify::v1::client::zcosmic_overlap_notification_v1::Event::LayerLeave { identifier } => todo!(),
        //     _ => todo!(),
        // }
    }
}

delegate_dispatch!(GlobalState: [ZcosmicOverlapNotifyV1: GlobalData] => OverlapNotifyManager);
delegate_dispatch!(GlobalState: [ZcosmicOverlapNotificationV1: ()] => OverlapNotifyManager);
