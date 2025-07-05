use std::sync::Arc;

use iced::font::load;
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use iced::widget::{button,column, container, row, text, text_editor,horizontal_space};
use std::{io, result};
use std::path::Path;
use rfd::FileHandle;

fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{
    content : text_editor::Content,
    error: Option<Error>
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action),
    Open,
    FileOpened(Result<Arc<String>, Error>)
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
            error :None,
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
                Command::none()
            }
            Message::Open => {
                Command::perform(pick_file(), Message::FileOpened)
            }
            Message::FileOpened(Ok(content)) => {
                self.content = text_editor::Content::with(&content);                
                Command::none()
            }
            Message::FileOpened(Err((error))) => {
                self.error = Some(error) ;
                Command::none()
            }
        }
    }
    


    fn view(&self) -> Element<'_, Message> { 
        let controls = row![button("Open").on_press(Message::Open)];

        let input = text_editor(&self.content).on_edit(Message::Edit);
        let position = {
            let (line,column) = self.content.cursor_position();

            text(format!("{}:{}", line+1 , column + 1))
        };
        let status_bar = row![horizontal_space(Length::Fill), position];
        container(column![controls, input, status_bar].spacing(10))
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn pick_file() -> Result<Arc<String>, Error>{
    let handle = rfd::AsyncFileDialog::new().set_title("choose a text file...").pick_file().await.ok_or(Error::DialogClosed)?;
    load_file(handle.path()).await
}

async fn load_file( path : impl AsRef<Path>) -> Result<Arc<String> , Error> {
    tokio::fs::read_to_string(path)
    .await
    .map(Arc::new)
    .map_err(|error| error.kind())
    .map_err(Error::IO)
}

#[derive(Debug,Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind)
}