use bevy::prelude::*;

pub fn close_app(mut event_writer: EventWriter<AppExit>){
    info!("Closing game");
    event_writer.write(AppExit::Success);
}
