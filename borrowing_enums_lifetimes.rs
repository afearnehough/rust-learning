
struct DrawCommand {
    draw_index: i32
}

enum Signal<'signal> {
    TickSignal,
    
    GenerateDrawCommands(&'signal mut Vec<DrawCommand>)
}

fn process_signal(signal: &mut Signal) {
    match signal {
        Signal::TickSignal => println!("tick"),
        Signal::GenerateDrawCommands(commands) => {
            println!("adding draw commands");
            commands.push(DrawCommand {
                draw_index: commands.len() as i32
            })
        }
    }
}

fn main() {
    println!("Hello, world!");
    
    let mut tick_signal = Signal::TickSignal;
    process_signal(&mut tick_signal);
    
    let mut draw_commands = Vec::new();
    
    process_signal(&mut Signal::GenerateDrawCommands(&mut draw_commands));
    process_signal(&mut Signal::GenerateDrawCommands(&mut draw_commands));
    process_signal(&mut Signal::GenerateDrawCommands(&mut draw_commands));
    process_signal(&mut Signal::GenerateDrawCommands(&mut draw_commands));
    
    for command in draw_commands {
        println!("Draw index: {}", command.draw_index);
    }
}
