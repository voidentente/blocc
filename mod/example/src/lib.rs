use bevy::prelude::*;

pub mod client {
    use bevy::prelude::*;
    use crate::common_code;

    pub struct ExampleClientPlugin;

    impl Plugin for ExampleClientPlugin {
        fn build(&self, app: &mut App) {
            app.add_startup_system(|| info!("Hello World from Clientside!"));
            common_code(app);
        }
    }
}

pub mod server {
    use bevy::prelude::*;
    use crate::common_code;

    pub struct ExampleServerPlugin;

    impl Plugin for ExampleServerPlugin {
        fn build(&self, app: &mut App) {
            app.add_startup_system(|| info!("Hello World from Serverside!"));
            common_code(app);
        }
    }
}

fn common_code(app: &mut App) {
    app.add_startup_system(|| info!("Hello World!"));
}