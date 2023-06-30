use crate::dom::cmd::Modifier;

/// Effects is a convenient way to group Msg for component to execute subsequent updates based on certain conditions.
/// This can be used for doing animation and incremental changes to the view to provide an effect
/// of transition or animation.
///
/// Effects contains 2 types of Messages. The local messages which will be executed in its
/// own component on the next update loop. The other type is the external effects which are Messages
/// that are sent to the parent Component in response to an event that has been triggerred.
pub struct Effects<MSG, XMSG> {
    /// Messages that will be executed locally in the Component
    pub local: Vec<MSG>,
    /// effects that will be executed on the parent Component which instantiate
    /// this component
    pub external: Vec<XMSG>,
    pub(crate) modifier: Modifier,
}

impl<MSG, XMSG> Effects<MSG, XMSG> {
    /// create a new Effects with local and external expects respectively
    pub fn new(
        local: impl IntoIterator<Item = MSG>,
        external: impl IntoIterator<Item = XMSG>,
    ) -> Self {
        Self {
            local: local.into_iter().collect(),
            external: external.into_iter().collect(),
            modifier: Modifier::default(),
        }
    }

    /// split the local and external MSG of this effect
    pub fn unzip(self) -> (Vec<MSG>, Vec<XMSG>) {
        let Self {
            local,
            external,
            modifier: _,
        } = self;
        (local, external)
    }

    /// Create an Effects with  local messages that will be executed on the next update loop on this Component
    pub fn with_local(local: impl IntoIterator<Item = MSG>) -> Self {
        Self {
            local: local.into_iter().collect(),
            external: vec![],
            modifier: Modifier::default(),
        }
    }

    /// Create an Effects with extern messages that will be executed on the parent Component
    pub fn with_external(external: impl IntoIterator<Item = XMSG>) -> Self {
        Self {
            local: vec![],
            external: external.into_iter().collect(),
            modifier: Modifier::default(),
        }
    }

    /// Create and empty Effects
    pub fn none() -> Self {
        Self {
            local: vec![],
            external: vec![],
            modifier: Modifier::default(),
        }
    }

    /// Map the local messages of this Effects such that MSG will be transposed into
    /// MSG2 with the use of the mapping function `f`.
    ///
    /// The external messages stays the same.
    pub fn map_msg<F, MSG2>(self, f: F) -> Effects<MSG2, XMSG>
    where
        F: Fn(MSG) -> MSG2 + 'static,
    {
        let Effects {
            local,
            external,
            modifier,
        } = self;

        Effects {
            local: local.into_iter().map(f).collect(),
            external,
            modifier,
        }
    }

    /// Map the external messages of this Effects such that XMSG will be transposed into XMSG2
    /// with the use of the mapping function `f`
    pub fn map_external<F, XMSG2>(self, f: F) -> Effects<MSG, XMSG2>
    where
        F: Fn(XMSG) -> XMSG2 + 'static,
    {
        let Effects {
            local,
            external,
            modifier,
        } = self;
        Effects {
            local,
            external: external.into_iter().map(f).collect(),
            modifier,
        }
    }

    /// derives an Effects which contains only local effects by transforming the external messages
    /// and mapping them with function `f` such that they can be of the same type as local effects
    /// them merge them together into local effects.
    ///
    pub fn localize<F>(self, f: F) -> Effects<XMSG, ()>
    where
        F: Fn(MSG) -> XMSG + 'static,
    {
        let Effects {
            local,
            external,
            modifier,
        } = self;

        Effects {
            local: external
                .into_iter()
                .chain(local.into_iter().map(f))
                .collect(),
            external: vec![],
            modifier,
        }
    }

    /// Append this msgs to the local effects
    pub fn append_local(mut self, local: impl IntoIterator<Item = MSG>) -> Self {
        self.local.extend(local);
        self
    }

    /// Modify the Effect such that it will not do an update on the view when it is executed
    pub fn no_render(mut self) -> Self {
        self.modifier.should_update_view = false;
        self
    }

    /// Modify the Effect such that it will log measurement when it is executed
    pub fn measure(mut self) -> Self {
        self.modifier.log_measurements = true;
        self
    }

    /// Merge all the internal objects of this Vec of Effects to produce only one.
    pub fn merge_all(all_effects: Vec<Self>) -> Self {
        let mut local = vec![];
        let mut external = vec![];
        for effect in all_effects {
            local.extend(effect.local);
            external.extend(effect.external);
        }
        Effects::new(local, external)
    }

    /// Extern the local and external MSG of this Effect
    pub fn extend(
        mut self,
        local: impl IntoIterator<Item = MSG>,
        external: impl IntoIterator<Item = XMSG>,
    ) -> Self {
        self.local.extend(local);
        self.external.extend(external);
        self
    }
}
