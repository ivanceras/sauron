use crate::dom::Cmd;
use crate::vdom::Node;
use mt_dom::TreePath;
use std::fmt;


///
pub struct Eval{
    expr: Box<dyn Fn() -> bool>,
    children: Vec<Eval>,
}

impl fmt::Debug for Eval{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}", (self.expr)())?;
        write!(f, ",[")?;
        for child in &self.children{
            child.fmt(f)?;
        }
        write!(f, "])")?;
        Ok(())
    }
}


impl Eval{

    /// new
    pub fn new(val: bool, children: impl IntoIterator<Item = Self> ) -> Self {
        Self{
            expr: Box::new(move||val),
            children: children.into_iter().collect()
        }
    }


    ///
    pub fn none() -> Self {
        Eval{
            expr: Box::new(||false),
            children: vec![],
        }
    }

    ///
    pub fn traverse(evals: &[Eval]) -> Vec<TreePath> {
        let root = TreePath::root();
        if evals.len() == 1 {
            Self::traverse_recursive(&evals[0], root)
        }else{
            Self::traverse_list(evals, root)
        }
    }

    ///
    fn traverse_list(evals: &[Eval], current: TreePath) -> Vec<TreePath> {
        let mut paths = vec![];
        for (i,eval) in evals.iter().enumerate(){
            let more_paths = eval.traverse_recursive(current.traverse(i));
            paths.extend(more_paths);
        }
        paths
    }


    fn traverse_recursive(&self, current: TreePath) -> Vec<TreePath>{
        let mut paths = vec![];
        if (self.expr)(){
            paths.push(current.clone());
        }
        let more_paths = Self::traverse_list(&self.children, current);
        paths.extend(more_paths);
        paths
    }
}

/// evaluate check
pub fn eval(val: bool, children: impl IntoIterator<Item = Eval>) -> Eval{
    Eval::new(val, children)
}

/// An Application is the root component of your program.
/// Everything that happens in your application is done here.
///
pub trait Application<MSG>
where
    MSG: 'static,
{
    ///  The application can implement this method where it can modify its initial state.
    ///  This method is called right after the program is mounted into the DOM.
    fn init(&mut self) -> Cmd<Self, MSG>
    where
        Self: Sized + Clone + 'static,
    {
        Cmd::none()
    }

    /// Update the component with a message.
    /// The update function returns a Cmd, which can be executed by the runtime.
    ///
    /// Called each time an action is triggered from the view
    fn update(&mut self, _msg: MSG) -> Cmd<Self, MSG>
    where
        Self: Sized + Clone + 'static;

    /// pre evaluate the expression to determine
    /// whether to diff the nodes
    fn pre_eval(&self, _other: &Self) -> Option<Vec<Eval>> {
        None
    }

    /// Returns a node on how the component is presented.
    fn view(&self) -> Node<MSG>;

    /// The css style for the application, will be mounted automatically by the program
    fn stylesheet() -> Vec<String> {
        vec![]
    }

    /// dynamic style of an application which will be reinjected when the application style changed
    fn style(&self) -> Vec<String> {
        vec![]
    }

    /// This is called after dispatching and updating the dom for the component
    /// This is for diagnostic and performance measurement purposes.
    ///
    /// Warning: DO NOT use for anything else other than the intended purpose
    fn measurements(&self, measurements: Measurements) -> Cmd<Self, MSG>
    where
        Self: Sized + Clone + 'static,
    {
        log::debug!("Measurements: {:#?}", measurements);
        Cmd::none().no_render()
    }
}

/// Contains the time it took for the last app update call for the component
/// TODO: Maybe rename to Diagnostics
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Measurements {
    /// The application can name this measurement to determine where this measurement is coming
    /// from.
    pub name: String,
    /// The number of DOM nodes in this Component
    pub node_count: usize,
    /// Time it took for the Component to build it's view
    pub build_view_took: f64,
    /// Total number of patches applied on this update loop
    pub total_patches: usize,
    /// Time it took for the patching the DOM.
    pub dom_update_took: f64,
    /// Total time it took for the component dispatch
    pub total_time: f64,
    /// The total count reference of the Program App
    pub strong_count: usize,
    /// The total weak count reference of the Program App
    pub weak_count: usize,
}
