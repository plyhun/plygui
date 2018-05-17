#[macro_export]
macro_rules! impl_all_defaults {
	($typ: ty) => {
		impl_as_any!{$typ}
		impl_as_member!{$typ}
	}
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
	}
}

#[macro_export]
macro_rules! impl_as_member {
	($typ: ty) => {
		unsafe fn _as_member_mut(base: &mut ::plygui_api::development::MemberBase) -> &mut ::plygui_api::traits::UiMember {
			::plygui_api::utils::base_to_impl_mut::<$typ>(base)
		}
		unsafe fn _as_member(base: &::plygui_api::development::MemberBase) -> &::plygui_api::traits::UiMember {
			::plygui_api::utils::base_to_impl::<$typ>(base)
		}
	}
}
