use sauron::{html::text, node, Application, Cmd, Node};

pub enum Msg {
    Increment,
    Decrement,
    Reset,
}

#[derive(Default)]
pub struct App {
    count: i32,
}

impl Application for App {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        node! {
          <main class="mx-auto mt-10 text-center">
            <span class="isolate inline-flex rounded-md shadow-sm">
              <button
                type="button"
                class="relative inline-flex items-center rounded-l-md bg-white px-2 py-2 text-gray-400 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                on_click=|_| { Msg::Decrement }
              >
                <span class="sr-only">Previous</span>
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                  <path
                    fill-rule="evenodd"
                    d="M12.79 5.23a.75.75 0 01-.02 1.06L8.832 10l3.938 3.71a.75.75 0 11-1.04 1.08l-4.5-4.25a.75.75 0 010-1.08l4.5-4.25a.75.75 0 011.06.02z"
                    clip-rule="evenodd"
                  />
                </svg>
              </button>
              <button
                class="relative -ml-px inline-flex items-center bg-white px-4 py-2 text-gray-400 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                on_click=|_|{ Msg::Reset } >{ text(self.count) }
              </button>
              <button
                type="button"
                class="relative -ml-px inline-flex items-center rounded-r-md bg-white px-2 py-2 text-gray-400 ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:z-10"
                on_click=|_| { Msg::Increment }
              >
                <span class="sr-only">Next</span>
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                  <path
                    fill-rule="evenodd"
                    d="M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                    clip-rule="evenodd"
                  />
                </svg>
              </button>
            </span>
          </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
            Msg::Reset => self.count = 0,
        }
        Cmd::none()
    }
}
