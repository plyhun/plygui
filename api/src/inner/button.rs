use super::auto::{HasLabel, HasLabelInner, Clickable, ClickableInner};
use super::control::{Control, ControlInner, AControl};
use super::member::{Member, MemberInner, AMember};

define! {
    Button: Control + Clickable + HasLabel {
        inner: {
            fn with_label(label: &str) -> Box<AMember<AControl<AButton<Self>>>>;
        }
    }
}

impl<T: ButtonInner> ControlInner for AButton<T> {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16) {
        
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container) {
        
    }

    fn parent(&self) -> Option<&dyn Member> {
        
    }
    fn parent_mut(&mut self) -> Option<&mut dyn Member> {
        
    }
    fn root(&self) -> Option<&dyn Member> {
        
    }
    fn root_mut(&mut self) -> Option<&mut dyn Member> {
        
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &crate::markup::Markup, registry: &mut crate::markup::MarkupRegistry) {
        
    }  
}

impl<T: ButtonInner> Button for AMember<AControl<AButton<T>>> {}

impl<T: ButtonInner> AMember<AControl<AButton<T>>> {
    #[inline]
    pub fn with_label(label: &str) -> Box<dyn Button> {
        T::with_label(label)
    }
}
