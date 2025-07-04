use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use iced::widget::{column, container, row, text, text_editor,horizontal_space};
fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{
    content : text_editor::Content 
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action) 
}


impl Application for Editor  {
    type Message = Message; 
    type Flags = (); // represent any data for an application to be initialized, used as an argument to self
    type Executor = executor::Default; //engine used to run async tasks in bg
    type Theme = Theme; // self explanatory
    fn new(_flags : Self::Flags) -> (Self, Command<Message>){  
        (
            Self {
            content: text_editor::Content::with(include_str!("main.rs")), 
        }, 
        Command::none(),         
    )
}
    fn title(&self) -> String { 
        String::from("editor cuhh ")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) =>{
                self.content.edit(action);
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
