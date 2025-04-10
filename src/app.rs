use std::collections::HashMap;


pub enum CurrentScreen {
    Main, 
    TicketDashboard,
}


pub struct App {
    pub current_screen: CurrentScreen,
    pub tasks_map: HashMap<String, String>,
    pub display_cat: bool
}


impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            tasks_map: HashMap::new(),
            display_cat: false,
        }
    }

    pub fn load_fake_task(&mut self) {
        self.tasks_map.insert("Task 1".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 2".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 3".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 4".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 5".to_owned(), "This is a placeholder".to_owned());
    }


    pub fn show_cat(&mut self){
        if self.display_cat {
            self.display_cat = false
        } else {
            self.display_cat = true
        }
    }
}