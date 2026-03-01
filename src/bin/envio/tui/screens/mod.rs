mod edit_envs_screen;
mod get_key_screen;
mod profile_form_screen;
mod select_screen;

pub use edit_envs_screen::EditEnvsScreen;
pub use get_key_screen::GetKeyScreen;
pub use profile_form_screen::{CreateProfileScreen, EditProfileScreen};
pub use select_screen::SelectScreen;

use envio::Profile;
use ratatui::{Frame, crossterm::event::KeyEvent};

use crate::error::AppResult;

pub enum Action {
    None,
    Exit,
    OpenProfile(String),
    NewProfile,
    EditProfile(String),
    Back,
}

pub enum ScreenEvent {
    ProfileDecrypted(Profile),
}

#[derive(Clone)]
pub enum ScreenId {
    Select,
    CreateProfile,
    EditProfile(String),
    GetKey(String),
    Edit(Box<Profile>),
}

impl PartialEq for ScreenId {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScreenId::Select, ScreenId::Select) => true,
            (ScreenId::CreateProfile, ScreenId::CreateProfile) => true,
            (ScreenId::EditProfile(a), ScreenId::EditProfile(b)) => a == b,
            (ScreenId::GetKey(a), ScreenId::GetKey(b)) => a == b,
            (ScreenId::Edit(a), ScreenId::Edit(b)) => a.metadata.name == b.metadata.name,
            _ => false,
        }
    }
}

impl ScreenId {
    pub fn create_screen(&self) -> AppResult<Box<dyn Screen>> {
        match self {
            ScreenId::Select => Ok(Box::new(SelectScreen::new()?)),
            ScreenId::CreateProfile => Ok(Box::new(CreateProfileScreen::new()?)),
            ScreenId::EditProfile(name) => Ok(Box::new(EditProfileScreen::new(name.clone())?)),
            ScreenId::GetKey(name) => Ok(Box::new(GetKeyScreen::new(name.clone()))),
            ScreenId::Edit(profile) => Ok(Box::new(EditEnvsScreen::new(*profile.clone())?)),
        }
    }
}

pub trait Screen {
    fn draw(&mut self, frame: &mut Frame);
    fn handle_key_event(&mut self, key: KeyEvent) -> AppResult<Action>;
    fn tick(&mut self) -> AppResult<Option<ScreenEvent>> {
        Ok(None)
    }

    fn id(&self) -> ScreenId;
}
