use std::collections::HashMap;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{Client, Error};

pub enum CurrentScreen {
    Main, 
    TicketDashboard,
}


pub struct App {
    pub current_screen: CurrentScreen,
    pub tasks_map: HashMap<String, String>,
    pub display_cat: bool,
    pub table_names: HashMap<String, String>,
}


impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            tasks_map: HashMap::new(),
            display_cat: false,
            table_names: HashMap::new(),
        }
    }

    pub fn load_fake_task(&mut self) {
        self.tasks_map.insert("Task 1".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 2".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 3".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 4".to_owned(), "This is a placeholder".to_owned());
        self.tasks_map.insert("Task 5".to_owned(), "This is a placeholder".to_owned());
    }

    pub async fn get_table_names(&mut self) -> Result<(), Error> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        let client = Client::new(&config);
        let resp = client.list_tables().send().await?;

        let names = resp.table_names();

        for name in names {
            self.table_names.insert("Table".to_owned(), name.to_string());
        }

        Ok(())
    }

    pub fn show_cat(&mut self){
        if self.display_cat {
            self.display_cat = false
        } else {
            self.display_cat = true
        }
    }
}