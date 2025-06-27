use iced::{Element,Sandbox, Settings};
use iced::widget::{text_editor,container};
fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor{
    content : text_editor::Content //used to store the internal state of the text editor like the user text buffer
                                   // cursor posn, slection
}
#[derive(Debug,Clone)]
enum Message{
    Edit(text_editor::Action)// so text_editor::action represents a user action in the txt editor like typing a char 
                             // pressing backspace/del, moving the cursor, selecting text, pasting from clipboard
                             // these messages are represented as this enum 
}


impl Sandbox for Editor  {
    type Message = Message; 

    fn new() -> Self{  
        Self {
            content: text_editor::Content::new(), //makes a new plain editor
        }           
    }

    fn title(&self) -> String { 
        String::from("editor cuhh ")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) =>{//mathches that if the message is message::edit(action) then do self.content.edit(action)
                self.content.edit(action);
            }
        }
    }


    fn view(&self) -> Element<'_, Message> { 
        let input = text_editor(&self.content).on_edit(Message::Edit);

        container(input).padding(10).into()//used to contain other widgets, add padding, center them etc etc
    }  
}
//come back to this later
