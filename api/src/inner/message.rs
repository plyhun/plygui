use crate::{callbacks, types};

use super::auto::HasInner;
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member, MemberInner};

define! {
    Message: Member + HasLabel {
        outer: {
            fn severity(&self) -> types::MessageSeverity;
            fn start(self: Box<Self>) -> Result<String, ()>;
        },
        inner: {
            fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn Member>) -> Box<dyn Message>;
            fn severity(&self) -> types::MessageSeverity;
            fn start(self) -> Result<String, ()>;
        }
    }
}

pub trait NewMessage {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn Member>) -> Box<dyn Message>;
    fn with_content(content: types::TextContent, severity: types::MessageSeverity, parent: Option<&dyn Member>) -> Box<dyn Message> {
        Self::with_actions(content, severity, vec![], parent)
    }
    fn start_with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn Member>) -> Result<String, ()> {
        Self::with_actions(content, severity, actions, parent).start()
    }
}

impl<II: MessageInner, T: HasInner<I = II> + 'static> MessageInner for T {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn Member>) -> Box<dyn Message> {
        <<Self as HasInner>::I as MessageInner>::with_actions(content, severity, actions, parent)
    }
    fn severity(&self) -> types::MessageSeverity {
        self.inner().severity()
    }
    fn start(self) -> Result<String, ()> {
        self.into_inner().start()
    }
}

impl<T: MessageInner> Message for AMember<AMessage<T>> {
    #[inline]
    fn severity(&self) -> types::MessageSeverity {
        self.inner.inner.severity()
    }
    #[inline]
    fn start(self: Box<Self>) -> Result<String, ()> {
        self.inner.start()
    }

    #[inline]
    fn as_message(&self) -> &dyn Message {
        self
    }
    #[inline]
    fn as_message_mut(&mut self) -> &mut dyn Message {
        self
    }
    #[inline]
    fn into_message(self: Box<Self>) -> Box<dyn Message> {
        self
    }
}

impl<T: MessageInner> NewMessage for AMember<AMessage<T>> {
    #[inline]
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn Member>) -> Box<dyn Message> {
        T::with_actions(content, severity, actions, parent)
    }
}
