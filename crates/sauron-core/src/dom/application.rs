use crate::dom::Cmd;
use crate::vdom::Node;
use mt_dom::TreePath;


pub struct Expr<V>{
    expr: V,
    children: Vec<Expr<V>>,
}

impl<V> Expr<V> where V:PartialEq{

    fn traverse_recursive(&self, old: &Self, current: TreePath) -> Vec<TreePath> {
        let mut paths = vec![];
        if self.expr == old.expr {
            paths.push(current.clone());
        }
        for (i, (child, old_child)) in self.children.iter().zip(old.children.iter()).enumerate(){
            let more_paths = child.traverse_recursive(old_child, current.traverse(i));
            paths.extend(more_paths);
        }
        paths
    }
}

pub enum Eval<V>{
    Expr(Expr<V>),
    List(Vec<Eval<V>>),
}

#[allow(unused)]
impl<V> Eval<V> where V:PartialEq{
    fn traverse(&self, old:&Self) -> Vec<TreePath>{
       self.traverse_recursive(old, TreePath::root())
    }
    fn traverse_recursive(&self, old: &Self, current: TreePath) -> Vec<TreePath> {
        match self{
            Self::Expr(expr) => {
                let Self::Expr(old_expr) = old else{unreachable!()};
                expr.traverse_recursive(old_expr, current)
            }
            Self::List(evals) => {
                let mut paths = vec![];
                let Self::List(old_evals) = old else{unreachable!()};
                for (i, (eval, old_eval)) in evals.iter().zip(old_evals.iter()).enumerate(){
                    let more_paths = eval.traverse_recursive(old_eval, current.traverse(i));
                    paths.extend(more_paths);
                }
                paths
            }
        }
    }
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
        Self: Sized + 'static,
    {
        Cmd::none()
    }

    /// Update the component with a message.
    /// The update function returns a Cmd, which can be executed by the runtime.
    ///
    /// Called each time an action is triggered from the view
    fn update(&mut self, _msg: MSG) -> Cmd<Self, MSG>
    where
        Self: Sized + 'static;

    ///
    fn pre_eval<V>(&self) -> Eval<V> {
        todo!()
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
        Self: Sized + 'static,
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
