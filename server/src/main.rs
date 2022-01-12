use std::sync::Arc;

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy::{app::ScheduleRunnerSettings, core, utils::Duration};
use bootleg_networking::*;

const MESSAGE_CHANNEL_ID: MessageChannelID = MessageChannelID::new(0);
const MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: MESSAGE_CHANNEL_ID.id,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 25600,
    packet_buffer_size: 25600,
};

fn main() {
    let mut app = App::new();
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .add_plugins(MinimalPlugins)
    .add_plugin(NetworkingPlugin)
    .add_startup_system(setup)
    .add_system(send)
    .add_system(receive);

    //Uncomment the line below!
    app.run();
}

fn setup(mut commands: Commands, tokio_rt: Res<Runtime>, task_pool: Res<IoTaskPool>) {
    // First we need to actually initiate the NetworkReource. In this case, it's a server
    // We could use the new_client function if wanted a client
    let mut net = NetworkResource::new_server(tokio_rt.clone(), task_pool.0.clone());

    // Next, we need tell the server to setup listening
    // The equivalent function for clients is connect
    // Listen on ports 9000 for TCP and 9001 for UDP, and 9003
    // The first address is the one that the connect() function needs to use, and the other two are for WebRTC
    // Finally, the last argument is the maximum size of each packet. That argument is only necessary for native builds
    let listen_config = ListenConfig {
        tcp_addr: "127.0.0.1:9000",
        udp_addr: "127.0.0.1:9001",
        naia_addr: "127.0.0.1:9003",
        webrtc_listen_addr: "127.0.0.1:9004",
        public_webrtc_listen_addr: "127.0.0.1:9004",
    };

    net.listen(listen_config, Some(2 * 2048));
    // If we were calling net.connect, the first argument we would either have 9000 or 9003 as the port, depending on whether we were a native client or a web client
    // The second argument is only necessary on native builds, and it's asking for the UDP server SocketAddr
    /* let connect_config = ConnectConfig {
     *      addr: "127.0.0.1:9000",
     *      udp_addr: Some("127.0.0.1:9001"),
     * };
     *
     * net.connect(connect_config, Some(2048));
     */

    // We need to register for the native tcp/udp server and for naia seperately
    // Native registration
    net.register_message_channel_native(MESSAGE_SETTINGS, &MESSAGE_CHANNEL_ID)
        .unwrap();
    // Naia registration
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder.register::<String>(MESSAGE_SETTINGS).unwrap();
    });

    // Finally, insert the network resource so it can be used by other systems
    commands.insert_resource(net);
}

// The following two functions are equivalent for both clients and servers, provided you've set up the NetworkResource properly

fn send(mut net: ResMut<NetworkResource>) {
    let message = String::from("Hello from server!");
    net.broadcast_message(&message, &MESSAGE_CHANNEL_ID)
        .unwrap();
}

fn receive(mut net: ResMut<NetworkResource>) {
    let messages = net.view_messages::<String>(&MESSAGE_CHANNEL_ID).unwrap();

    for (_handle, message) in messages.iter() {
        println!("Recieved: {}b", message.len());
    }
}
