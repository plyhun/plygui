#[macro_export]
macro_rules! default_pub_use {
    () => {
        pub use plygui_api::callbacks;
        pub use plygui_api::controls::*;
        pub use plygui_api::external;
        pub use plygui_api::ids::*;
        pub use plygui_api::layout;
        pub use plygui_api::types::*;
        pub use plygui_api::utils;

        pub mod imp {
            pub use crate::application::Application;
            pub use crate::button::Button;
            pub use crate::frame::Frame;
            pub use crate::image::Image;
            pub use crate::layout_linear::LinearLayout;
            pub use crate::message::Message;
            pub use crate::progress_bar::ProgressBar;
            pub use crate::splitted::Splitted;
            pub use crate::text::Text;
            pub use crate::tray::Tray;
            pub use crate::window::Window;
            pub use crate::list::List;
        }
    };
}

#[macro_export]
macro_rules! default_markup_register_members {
    () => {
        #[cfg(feature = "markup")]
        pub fn register_members(registry: &mut plygui_api::markup::MarkupRegistry) {
            use plygui_api::development::Spawnable;
            
            registry.register_member(plygui_api::markup::MEMBER_TYPE_BUTTON.into(), imp::Button::spawn).unwrap();
            registry.register_member(plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT.into(), imp::LinearLayout::spawn).unwrap();
            registry.register_member(plygui_api::markup::MEMBER_TYPE_FRAME.into(), imp::Frame::spawn).unwrap();
            registry.register_member(plygui_api::markup::MEMBER_TYPE_SPLITTED.into(), imp::Splitted::spawn).unwrap();
            registry.register_member(plygui_api::markup::MEMBER_TYPE_IMAGE.into(), imp::Image::spawn).unwrap();
            registry.register_member(plygui_api::markup::MEMBER_TYPE_TEXT.into(), imp::Text::spawn).unwrap();
            registry.register_member(plygui_api::markup::MEMBER_TYPE_PROGRESS_BAR.into(), imp::ProgressBar::spawn).unwrap();
            registry.register_member(plygui_api::markup::MEMBER_TYPE_LIST.into(), imp::List::spawn).unwrap();
        }
    };
}

#[macro_export]
macro_rules! default_impls_as {
    ($typ: ty) => {
        impl_as_any! {$typ}
        impl_as_member! {$typ}
    };
}

#[macro_export]
macro_rules! impl_as_any {
    ($typ: ty) => {
        unsafe fn _as_any_mut(base: &mut ::plygui_api::development::MemberBase) -> &mut ::std::any::Any {
            ::plygui_api::utils::base_to_impl_mut::<$typ>(base)
        }
        unsafe fn _as_any(base: &::plygui_api::development::MemberBase) -> &::std::any::Any {
            ::plygui_api::utils::base_to_impl::<$typ>(base)
        }
    };
}

#[macro_export]
macro_rules! impl_as_member {
    ($typ: ty) => {
        unsafe fn _as_member_mut(base: &mut ::plygui_api::development::MemberBase) -> &mut ::plygui_api::controls::Member {
            ::plygui_api::utils::base_to_impl_mut::<$typ>(base)
        }
        unsafe fn _as_member(base: &::plygui_api::development::MemberBase) -> &::plygui_api::controls::Member {
            ::plygui_api::utils::base_to_impl::<$typ>(base)
        }
    };
}
