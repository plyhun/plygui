mod auto;
mod member;
mod native_id;
mod control;
mod drawable;
mod container;
mod container_single;
mod container_multi;
mod has_layout;
mod has_size;
mod has_visibility;
mod has_label;
mod has_progress;
mod has_image;
mod clickable;
mod closeable;

//mod application;
//mod button;
mod tray;

pub(crate) mod seal {
    pub trait Sealed {}
}
