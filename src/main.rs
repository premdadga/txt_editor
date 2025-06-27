use iced::{Theme, Element,Sandbox, Settings};
use iced::widget::{column, container, text, text_editor};
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


impl Sandbox for Editor  {
    type Message = Message; 

    fn new() -> Self{  
        Self {
            content: text_editor::Content::with(include_str!("main.rs")), 
        }           
    }

    fn title(&self) -> String { 
        String::from("editor cuhh ")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) =>{
                self.content.edit(action);
            }
        }
    }


    fn view(&self) -> Element<'_, Message> { 
        let input = text_editor(&self.content).on_edit(Message::Edit);
        let position = {
            let (line,column) = self.content.cursor_position();

            text(format!("{}:{}", line+1 , column + 1))
        };

        container(column![input, position]).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
