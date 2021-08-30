/// Effects are a way for component to execute subsequent updates based on certain conditions.
/// This can be used for doing animation and incremental changes to the view to provide an effect
/// of transition or animation.
///
/// Effects contains 2 types of Messages. The local messages which will be executed in its
/// own component on the next update loop. The other type is the external effects which are Messages
/// that are sent to the parent Component in response to an event that has been triggerred.
pub struct Effects<MSG, PMSG> {
    /// Messages that will be executed locally in the Component
    pub local: Vec<MSG>,
    /// effects that will be executed on the parent Component that instantiate
    /// this component
    pub external: Vec<PMSG>,
}

impl<MSG, PMSG> Effects<MSG, PMSG> {
    /// create a new Effects with local and external expects respectively
    pub fn new(local: Vec<MSG>, external: Vec<PMSG>) -> Self {
        Self { local, external }
    }

    /// Create an Effects with  local messages that will be executed on the next update loop on this Component
    pub fn with_local(local: Vec<MSG>) -> Self {
        Self {
            local,
            external: vec![],
        }
    }
    /// Create an Effects with extern messages that will be executed on the parent Component
    pub fn with_external(external: Vec<PMSG>) -> Self {
        Self {
            local: vec![],
            external,
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
    pub fn map_msg<F, MSG2>(self, f: F) -> Effects<MSG2, PMSG>
    where
        F: Fn(MSG) -> MSG2 + 'static,
    {
        let Effects { local, external } = self;

        Effects {
            local: local.into_iter().map(f).collect(),
            external,
        }
    }

    /// derives an Effects which contains only local effects by transforming the external messages
    /// and mapping them with function `f` such that they can be of the same type as local effects
    /// them merge them together into local effects.
    ///
    pub fn localize<F>(self, f: F) -> Effects<PMSG, ()>
    where
        F: Fn(MSG) -> PMSG + 'static,
    {
        let Effects { local, external } = self;

        Effects {
            local: external
                .into_iter()
                .chain(local.into_iter().map(f))
                .collect(),
            external: vec![],
        }
    }

    /// Append this msgs to the local effects
    pub fn append_local(mut self, local: Vec<MSG>) -> Self {
        self.local.extend(local);
        self
    }
}
