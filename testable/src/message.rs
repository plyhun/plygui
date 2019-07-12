use crate::common::{self, *};

struct TestableMessageAction {
    title: String,
    id: i32,
    cb: callbacks::Action,
}
impl From<(String, callbacks::Action)> for TestableMessageAction {
    fn from(a: (String, callbacks::Action)) -> Self {
        TestableMessageAction {
            id: {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};

                let mut hasher = DefaultHasher::new();
                a.0.hash(&mut hasher);
                hasher.finish() as i32
            },
            title: a.0,
            cb: a.1,
        }
    }
}

#[repr(C)]
pub struct TestableMessage {
    id: common::InnerId,
    parent: Option<ids::Id>,
    label: String,
    text: String,
    severity: types::MessageSeverity,
    actions: Vec<TestableMessageAction>,
}

pub type Message = Member<TestableMessage>;

impl HasLabelInner for TestableMessage {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        Cow::Borrowed(self.label.as_ref())
    }
    fn set_label(&mut self, _base: &mut MemberBase, label: Cow<str>) {
        self.label = label.into();
    }
}

impl MessageInner for TestableMessage {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Box<Member<Self>> {
        let (label, text) = match content {
            types::TextContent::Plain(text) => (String::new(/* TODO app name here? */), text),
            types::TextContent::LabelDescription(label, description) => (label, description),
        };
        let a: Box<Message> = Box::new(Member::with_inner(
            TestableMessage {
                id: ptr::null_mut(),
                parent: parent.map(|p|p.id()),
                label: label,
                text: text,
                severity: severity,
                actions: actions.into_iter().map(|a| a.into()).collect(),
            },
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        a
    }
    fn start(mut self) -> Result<String, ()> {
        self.actions.iter().nth(0)/*find(|a| a.id == pressed)*/.map(|a| a.title.clone()).ok_or(())
    }
    fn severity(&self) -> types::MessageSeverity {
        self.severity
    }
}

impl HasNativeIdInner for TestableMessage {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.id.into()
    }
}

impl MemberInner for TestableMessage {}

default_impls_as!(Message);
