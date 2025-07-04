use std::sync::Arc;

use iced::font::load;
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use iced::widget::{column, container, row, text, text_editor,horizontal_space};
use std::{io, result};
use std::path::Path;

fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{
    content : text_editor::Content 
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, io::ErrorKind>)
}


impl Application for Editor  {
    type Message = Message; 
    type Flags = (); // represent any data for an application to be initialized, used as an argument to self
    type Executor = executor::Default; //engine used to run async tasks in bg
    type Theme = Theme; // self explanatory
    fn new(_flags : Self::Flags) -> (Self, Command<Message>){  
        (
            Self {
            content: text_editor::Content::new(), 
        }, 
        Command::perform(load_file(
            format!("{}/src/main.rs",
            env!("CARGO_MANIFEST_DIR")
        )), Message::FileOpened),         
    )// we are using ( , ) as this is js one thing we are returning
}
    fn title(&self) -> String { 
        String::from("editor cuhh ")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) =>{
                self.content.edit(action);
            }
        Message::FileOpened(result) => {
            if let Ok(content) = result {
                self.content = text_editor::Content::with(&content);                
            }
        }
        }
        Command::none()
    }


    fn view(&self) -> Element<'_, Message> { 
        let input = text_editor(&self.content).on_edit(Message::Edit);
        let position = {
            let (line,column) = self.content.cursor_position();

            text(format!("{}:{}", line+1 , column + 1))
        };
        let status_bar = row![horizontal_space(Length::Fill), position];
        container(column![input, status_bar].spacing(10))
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_file( path : impl AsRef<Path>) -> Result<Arc<String> , io::ErrorKind> {
    tokio::fs::read_to_string(path).await.map(Arc::new).map_err(|error| error.kind())
}