use std::os::raw::c_void;

use super::bridge::*;

/// This should only ever be created once
#[derive(Debug)]
pub struct MenuBar<T: Copy + std::fmt::Debug> {
    ptr: *const c_void,
    menus: Vec<Menu<T>>,
}

#[derive(Debug)]
pub struct Menu<T: Copy + std::fmt::Debug> {
    ptr: *const c_void,
    /// Not needed for top level popup menu
    name: Option<String>,
    items: Vec<MenuItem<T>>,
    help: Option<String>,
}

#[derive(Debug)]
pub enum MenuItem<T: Copy + std::fmt::Debug> {
    Submenu(Menu<T>),
    Entry(MenuEntry<T>),
    Separator,
}

#[derive(Debug)]
pub struct MenuEntry<T: Copy + std::fmt::Debug> {
    id: T,
    event_id: i32,
    name: String,
    help: Option<String>,
}

impl<T: Copy + std::fmt::Debug> MenuEntry<T> {
    pub fn new(id: T, name: String) -> Self {
        Self {
            id,
            event_id: 0,
            name,
            help: None,
        }
    }

    pub fn help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
}

impl<T: PartialEq + Copy + std::fmt::Debug> Menu<T> {
    pub fn enable_item_by_id(&mut self, id: T) {
        for (i, item) in self.items.iter_mut().enumerate() {
            match item {
                MenuItem::Entry(e) => {
                    if e.id == id {
                        self.enable_item(i);
                        return;
                    }
                }
                MenuItem::Submenu(s) => s.enable_item_by_id(id),
                _ => (),
            }
        }
    }

    pub fn disable_item_by_id(&mut self, id: T) {
        for (i, item) in self.items.iter_mut().enumerate() {
            match item {
                MenuItem::Entry(e) => {
                    if e.id == id {
                        self.disable_item(i);
                        return;
                    }
                }
                MenuItem::Submenu(s) => s.disable_item_by_id(id),
                _ => (),
            }
        }
    }
}

impl<T: Copy + std::fmt::Debug> Menu<T> {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name,
            ptr: create_menu(),
            items: vec![],
            help: None,
        }
    }

    pub fn help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }

    pub fn push_separator(mut self) -> Self {
        self.insert(self.items.len(), MenuItem::Separator);
        self
    }

    pub fn push_entry(mut self, entry: MenuEntry<T>) -> Self {
        self.insert(self.items.len(), MenuItem::Entry(entry));
        self
    }

    pub fn push_submenu(mut self, submenu: Menu<T>) -> Self {
        self.insert(self.items.len(), MenuItem::Submenu(submenu));
        self
    }

    pub fn get_entry_from_event_id(&self, event_id: i32) -> Option<T> {
        for item in self.items.iter() {
            match item {
                MenuItem::Entry(e) => {
                    if e.event_id == event_id {
                        return Some(e.id);
                    }
                }
                MenuItem::Submenu(s) => {
                    let r = s.get_entry_from_event_id(event_id);
                    if r.is_some() {
                        return r;
                    }
                }
                _ => (),
            }
        }
        None
    }

    pub fn enable_item(&mut self, i: usize) {
        enable_menu_item(self.ptr, i, true);
    }

    pub fn disable_item(&mut self, i: usize) {
        enable_menu_item(self.ptr, i, false);
    }

    pub fn insert(&mut self, i: usize, mut entry: MenuItem<T>) {
        match &mut entry {
            MenuItem::Submenu(sub) => insert_submenu(
                self.ptr,
                i,
                sub.ptr,
                sub.name
                    .as_ref()
                    .expect("Submenus must have names")
                    .as_str(),
                sub.help.as_ref().map(|s| s.as_str()),
            ),
            MenuItem::Entry(entry) => {
                let id = insert_to_menu(
                    self.ptr,
                    i,
                    &entry.name,
                    entry.help.as_ref().map(|s| s.as_str()),
                );
                entry.event_id = id;
            }
            MenuItem::Separator => insert_separator_to_menu(self.ptr, i),
        }
        self.items.insert(i, entry);
    }

    pub fn remove(&mut self, i: usize) {
        remove_from_menu(self.ptr, i);
        self.items.remove(i);
    }

    pub fn popup(&self) {
        set_status_menu(self.ptr);
    }
}

impl<T: Copy + std::fmt::Debug> Drop for Menu<T> {
    fn drop(&mut self) {
        delete_menu(self.ptr)
    }
}

impl<T: Copy + std::fmt::Debug> MenuBar<T> {
    pub fn new() -> Self {
        Self {
            ptr: create_menu_bar(),
            menus: vec![],
        }
    }

    pub fn show(&self) {
        set_menu_bar(self.ptr)
    }

    pub fn append(&mut self, menu: Menu<T>) {
        insert_to_menu_bar(
            self.ptr,
            menu.ptr,
            self.menus.len(),
            menu.name.as_ref().map_or("Default", |s| s.as_str()),
        );
        self.menus.push(menu);
    }

    pub fn insert(&mut self, menu: Menu<T>, i: usize) {
        insert_to_menu_bar(
            self.ptr,
            menu.ptr,
            i,
            menu.name.as_ref().map_or("Default", |s| s.as_str()),
        );
        self.menus.insert(i, menu);
    }

    pub fn remove(&mut self, i: usize) {
        remove_from_menu_bar(self.ptr, i);
        self.menus.remove(i);
    }

    pub fn clear(&mut self) {
        for i in (0..self.menus.len()).rev() {
            self.remove(i);
        }
        self.menus.clear();
    }

    pub fn get_entry_from_event_id(&self, event_id: i32) -> Option<T> {
        for menu in self.menus.iter() {
            let r = menu.get_entry_from_event_id(event_id);
            if r.is_some() {
                return r;
            }
        }
        None
    }
}

impl<T: Copy + std::fmt::Debug> Drop for MenuBar<T> {
    fn drop(&mut self) {
        self.clear();
        delete_menu_bar(self.ptr)
    }
}
