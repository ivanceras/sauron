/// Msg that needs to be executed in its component on the next update loop
pub struct Effects<MSG, PMSG> {
    /// Msg that will be executed in its own widget
    pub follow_ups: Vec<MSG>,
    /// PMSG that will be executed in the component that mounts it
    pub effects: Vec<PMSG>,
}

impl<MSG, PMSG> Effects<MSG, PMSG> {
    /// create a new effects with follow_ups and effects
    pub fn new(follow_ups: Vec<MSG>, effects: Vec<PMSG>) -> Self {
        Self {
            follow_ups,
            effects,
        }
    }
    /// create a follow up message, but no effects
    pub fn with_follow_ups(follow_ups: Vec<MSG>) -> Self {
        Self {
            follow_ups,
            effects: vec![],
        }
    }
    /// Create effects with no follow ups.
    pub fn with_effects(effects: Vec<PMSG>) -> Self {
        Self {
            follow_ups: vec![],
            effects,
        }
    }

    /// No effects, no follow ups
    pub fn none() -> Self {
        Self {
            follow_ups: vec![],
            effects: vec![],
        }
    }

    /// map the follow up messages of this Effect such that
    /// follow ups with type Vec<MSG> will become Vec<MSG2>
    pub fn map_msg<F, MSG2>(self, f: F) -> Effects<MSG2, PMSG>
    where
        F: Fn(MSG) -> MSG2 + 'static,
    {
        let Effects {
            follow_ups,
            effects,
        } = self;

        Effects {
            follow_ups: follow_ups.into_iter().map(f).collect(),
            effects,
        }
    }

    /// map the `follow_ups` with function `f` and merge it with `effects`
    /// to create a new effect with homogenous follow ups
    pub fn merge<F>(self, f: F) -> Effects<PMSG, ()>
    where
        F: Fn(MSG) -> PMSG + 'static,
    {
        let Effects {
            follow_ups,
            effects,
        } = self;

        Effects {
            follow_ups: effects
                .into_iter()
                .chain(follow_ups.into_iter().map(f))
                .collect(),
            effects: vec![],
        }
    }
}
