use std::collections::HashMap;

pub struct Menu {
    pub id: u32,
    pub title: String,
    pub items: Vec<MenuItem>,
    pub is_open: bool,
}

pub struct MenuItem {
    pub id: u32,
    pub label: String,
    pub is_selected: bool,
}

pub struct MenuState {
    pub menus: HashMap<u32, Menu>,
    pub active_menu_id: Option<u32>,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            menus: HashMap::new(),
            active_menu_id: None,
        }
    }

    pub fn add_menu(&mut self, id: u32, title: String) {
        let menu = Menu {
            id,
            title,
            items: Vec::new(),
            is_open: false,
        };
        self.menus.insert(id, menu);
    }

    pub fn add_menu_item(&mut self, menu_id: u32, item_id: u32, label: String) {
        if let Some(menu) = self.menus.get_mut(&menu_id) {
            let item = MenuItem {
                id: item_id,
                label,
                is_selected: false,
            };
            menu.items.push(item);
        }
    }

    pub fn open_menu(&mut self, menu_id: u32) {
        if let Some(menu) = self.menus.get_mut(&menu_id) {
            menu.is_open = true;
            self.active_menu_id = Some(menu_id);
        }
    }

    pub fn close_menu(&mut self, menu_id: u32) {
        if let Some(menu) = self.menus.get_mut(&menu_id) {
            menu.is_open = false;
        }
        if self.active_menu_id == Some(menu_id) {
            self.active_menu_id = None;
        }
    }
}