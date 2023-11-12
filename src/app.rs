pub enum CurrentSreen { // to keep track of what screen is being displayed
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing { // to keep track of which field is being currently edited
    Key,
    Value,
}

pub struct App {
    pub key_input: String,                  // the currently being edited json key
    pub value_input: String,                // the currently being edited json value
    pub pairs: HashMap<String, String>,     // representation of our key-value pairs with serde serialization support
    pub current_screen: CurrentSreen,       // current screen the user is looking at, will determine what is rendered
    pub currently_editing: CurrentlyEditing // the optional state containing which of the key or value pairs the user is editing
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentSreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
}