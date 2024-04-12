#![deny(warnings)]
pub use app::App;
pub use error::Error;
use log::Level;
pub use restq::{ast::ddl::DataTypeDef, ColumnDef, DataType, DataValue};
use sauron::*;
use views::DataView;

#[macro_use]
extern crate log;

pub(crate) mod app;
pub(crate) mod assets;
mod error;
mod views;
pub(crate) mod widgets;

#[wasm_bindgen]
pub fn initialize(initial_state: &str) {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).expect("should work");
    trace!("Iniatialization start..!");

    trace!("initial state: {}", initial_state);
    trace!("mounting..");

    let data_view = create_data_view();
    let width = data_view.allocated_width;
    let height = data_view.allocated_height;
    let app = App::new(data_view, width, height);
    Program::mount_to_body(app);
}

fn create_data_view() -> DataView {
    let csv =
        "actor{*actor_id:s32,@first_name:text,last_name:text,last_update:utc,is_active:bool}\n\
        1,PENELOPE,GUINESS,2006-02-15 09:34:33,true\n\
        2,NICK,WAHLBERG,2006-02-15 09:34:33,false\n\
        3,ED,CHASE,2006-02-15 09:34:33,true\n\
        4,JENNIFER,DAVIS,2006-02-15 09:34:33,true\n\
        5,JOHNNY,LOLLOBRIGIDA,2006-02-15 09:34:33,true\n\
        6,BETTE,NICHOLSON,2006-02-15 09:34:33,true\n\
        7,GRACE,MOSTEL,2006-02-15 09:34:33,true\n\
        8,MATTHEW,JOHANSSON,2006-02-15 09:34:33,true\n\
        9,JOE,SWANK,2006-02-15 09:34:33,true\n\
        10,CHRISTIAN,GABLE,2006-02-15 09:34:33,true\n\
        11,ZERO,CAGE,2006-02-15 09:34:33,true\n\
        12,KARL,BERRY,2006-02-15 09:34:33,true\n\
        13,UMA,WOOD,2006-02-15 09:34:33,true\n\
        14,VIVIEN,BERGEN,2006-02-15 09:34:33,true\n\
        15,CUBA,OLIVIER,2006-02-15 09:34:33,true\n\
        16,FRED,COSTNER,2006-02-15 09:34:33,true\n\
        17,HELEN,VOIGHT,2006-02-15 09:34:33,true\n\
        18,DAN,TORN,2006-02-15 09:34:33,true\n\
        19,BOB,FAWCETT,2006-02-15 09:34:33,true\n\
        20,LUCILLE,TRACY,2006-02-15 09:34:33,true";

    trace!("csv data: {}", csv);
    let mut data_view = DataView::from_csv_data(csv.as_bytes().to_vec()).expect("must be parsed");
    let column_widths = [200, 200, 200, 500, 200];

    let total_width = column_widths.iter().fold(0, |acc, cw| acc + cw + 10);

    data_view.set_allocated_size(total_width, 600);
    data_view.set_column_widths(&column_widths);
    data_view.freeze_columns(vec![0]);
    data_view.freeze_rows(vec![(0, vec![0, 1, 2])]);
    data_view
}
