use iced::{Element,Sandbox, Settings};
use iced::widget::text;
fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor;
#[derive(Debug)]
enum Message{}


impl Sandbox for Editor  {
    type Message = Message; // these are user interactions or events that happen during the cours eof the thingy
                            // the messages that our application can ahndle or react to ex: user pressing a button is a message
                            // user typing is a message

    fn new() -> Self{  //initiliazing the state, basically telling the iced to this is how our state should be in
        Self           // when the state starts
    }

    fn title(&self) -> String { // title that will be displayed on the window of our application
        String::from("editor cuhh ")
    }

    fn update(&mut self, message: Self::Message) { //here we update the application state, with the change in messages 
        match message {}
    }

    fn view(&self) -> Element<'_, Message> { // widgets that comprise the usewr interface orr elements
        text("hello, iced!").into() // if a user interacts with these widgets, then it results in producing a message 
                                    // and the message is fed to the update above, it changes the state and gives back the view
    } //the .into() converts the text widget into a generic widget( Element widget) so what view does is u can returna any generic widget u want
    //view -> update -> view -> update, perpetual cycle of this 
}