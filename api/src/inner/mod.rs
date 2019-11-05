pub mod auto;
pub mod control;
pub mod drawable;
pub mod member;

pub mod container;
pub mod container_multi;
pub mod container_single;

pub mod has_image;
pub mod has_label;
pub mod has_layout;
pub mod has_native_id;
pub mod has_progress;
pub mod has_size;
pub mod has_visibility;

pub mod clickable;
pub mod closeable;

pub mod application;
//pub mod button;
pub mod tray;
pub mod window;

pub(crate) mod seal {
    pub trait Sealed {}
}
