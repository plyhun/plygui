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
