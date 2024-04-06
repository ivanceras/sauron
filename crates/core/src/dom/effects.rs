use crate::dom::Cmd;
use std::future::ready;

/// Effects is a convenient way to group Msg for component to execute subsequent updates based on certain conditions.
/// This can be used for doing animation and incremental changes to the view to provide an effect
/// of transition or animation.
///
/// Effects contains 2 types of Messages. The local messages which will be executed in its
/// own component on the next update loop. The other type is the external effects which are Messages
/// that are sent to the parent Component in response to an event that has been triggerred.
pub struct Effects<MSG, XMSG> {
    /// Messages that will be executed locally in the Component
    pub local: Vec<Cmd<MSG>>,
    /// effects that will be executed on the parent Component which instantiate
    /// this component
    pub external: Vec<Cmd<XMSG>>,
}

impl<MSG, XMSG> Effects<MSG, XMSG>
where
    MSG: 'static,
{
    /// create a new Effects with local and external expects respectively
    pub fn new(
        local: impl IntoIterator<Item = MSG>,
        external: impl IntoIterator<Item = XMSG>,
    ) -> Self
    where
        XMSG: 'static,
    {
        Self {
            local: local.into_iter().map(|l| Cmd::once(ready(l))).collect(),
            external: external.into_iter().map(|x| Cmd::once(ready(x))).collect(),
        }
    }

    /// Create an Effects with  local messages that will be executed on the next update loop on this Component
    pub fn with_local(local: impl IntoIterator<Item = MSG>) -> Self {
        Self {
            local: local.into_iter().map(|l| Cmd::once(ready(l))).collect(),
            external: vec![],
        }
    }

    /// Create an Effects with extern messages that will be executed on the parent Component
    pub fn with_external(external: impl IntoIterator<Item = XMSG>) -> Self
    where
        XMSG: 'static,
    {
        Self {
            local: vec![],
            external: external.into_iter().map(|x| Cmd::once(ready(x))).collect(),
        }
    }

    /// Create and empty Effects
    pub fn none() -> Self {
        Self {
            local: vec![],
            external: vec![],
        }
    }

    /// Map the local messages of this Effects such that MSG will be transposed into
    /// MSG2 with the use of the mapping function `f`.
    ///
    /// The external messages stays the same.
    pub fn map_msg<F, MSG2>(self, f: F) -> Effects<MSG2, XMSG>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
    {
        let Effects { local, external } = self;
        Effects {
            local: local.into_iter().map(|l| l.map_msg(f.clone())).collect(),
            external,
        }
    }

    /// Map the external messages of this Effects such that XMSG will be transposed into XMSG2
    /// with the use of the mapping function `f`
    pub fn map_external<F, XMSG2>(self, f: F) -> Effects<MSG, XMSG2>
    where
        F: Fn(XMSG) -> XMSG2 + Clone + 'static,
        XMSG: 'static,
        XMSG2: 'static,
    {
        let Effects { local, external } = self;
        Effects {
            local,
            external: external.into_iter().map(|l| l.map_msg(f.clone())).collect(),
        }
    }

    /// derives an Effects which contains only local effects by transforming the external messages
    /// and mapping them with function `f` such that they can be of the same type as local effects
    /// them merge them together into local effects.
    ///
    pub fn localize<F, XMSG2>(self, f: F) -> Effects<XMSG, XMSG2>
    where
        F: Fn(MSG) -> XMSG + Clone + 'static,
        XMSG: 'static,
        XMSG2: 'static,
    {
        let Effects { local, external } = self;

        Effects {
            local: external
                .into_iter()
                .chain(local.into_iter().map(|x| x.map_msg(f.clone())))
                .collect(),
            external: vec![],
        }
    }

    /// Append this msgs to the local effects
    pub fn append_local(mut self, local: impl IntoIterator<Item = MSG>) -> Self {
        self.local
            .extend(local.into_iter().map(|l| Cmd::once(ready(l))));
        self
    }

    /// Merge all the internal objects of this Vec of Effects to produce only one.
    pub fn batch(all_effects: impl IntoIterator<Item = Self>) -> Self {
        let mut local = vec![];
        let mut external = vec![];
        for effect in all_effects {
            local.extend(effect.local);
            external.extend(effect.external);
        }
        Effects { local, external }
    }

    /// Extern the local and external MSG of this Effect
    pub fn extend(
        mut self,
        local: impl IntoIterator<Item = MSG>,
        external: impl IntoIterator<Item = XMSG>,
    ) -> Self
    where
        XMSG: 'static,
    {
        self.local
            .extend(local.into_iter().map(|l| Cmd::once(ready(l))));
        self.external
            .extend(external.into_iter().map(|x| Cmd::once(ready(x))));
        self
    }
}

impl<MSG, XMSG> From<Cmd<MSG>> for Effects<MSG, XMSG> {
    fn from(task: Cmd<MSG>) -> Effects<MSG, XMSG> {
        Effects {
            local: vec![task],
            external: vec![],
        }
    }
}

impl<MSG> Effects<MSG, MSG>
where
    MSG: 'static,
{
    /// merge external msg into local msg, if they are of the same type
    pub fn merge(self) -> Effects<MSG, ()> {
        let Effects { local, external } = self;

        Effects {
            local: local.into_iter().chain(external).collect(),
            external: vec![],
        }
    }
}
