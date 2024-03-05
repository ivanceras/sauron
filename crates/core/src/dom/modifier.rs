/// These are collections of fields where we can modify the Cmd
/// such as logging measurement or should update the view
#[derive(Clone)]
pub struct Modifier {
    /// this instruct the program whether or not to update the view
    pub should_update_view: bool,
    /// tell the cmd to log the measurements of not.
    /// set only to true for a certain MSG where you want to measure the performance
    /// in the component update function. Otherwise, measurement calls
    /// for other non-trivial functions are also called causing on measurements.
    pub log_measurements: bool,
    /// Set the measurement name for this Cmd.
    /// This is used to distinguish measurements from other measurements in different parts of you
    /// application
    ///
    /// This measurment name will be copied
    /// into the [`Measurements`](crate::dom::Measurements) passed in
    /// [`Application::measurements`](crate::Application::measurements)
    pub measurement_name: String,
}

impl Default for Modifier {
    fn default() -> Self {
        Self {
            // every cmd will update the view by default
            should_update_view: true,
            // every cmd will not log measurement by default
            log_measurements: false,
            // empty string by default
            measurement_name: String::new(),
        }
    }
}

impl Modifier {
    /// coalesece the implicitly set values
    pub fn coalesce(&mut self, other: &Self) {
        if other.should_update_view {
            self.should_update_view = true;
        }
        if other.log_measurements {
            self.log_measurements = true;
        }
        if !other.measurement_name.is_empty() {
            self.measurement_name = other.measurement_name.to_string();
        }
    }
}
