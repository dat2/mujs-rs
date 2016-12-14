extern crate mujs_sys;
extern crate rustyline;

use mujs_sys::{JsContext, JsStateRef};

use rustyline::error::ReadlineError;
use rustyline::Editor;

extern "C" fn println_handler(state: JsStateRef) {
    let mut ctx = JsContext::shadow(state);

    let size = ctx.gettop();
    for i in 1..size {
        let jstr = ctx.tostring(i);
        print!("{}{}", if i > 1 { " " } else { "" }, jstr.to_str().unwrap());
    }
    println!();

    ctx.push_undefined();
}

extern "C" fn create_object(state: JsStateRef) {
    let mut ctx = JsContext::shadow(state);

    ctx.newobject();

    ctx.pushnumber(42.);
    ctx.setproperty(-2, "foo");

    ctx.pushboolean(true);
    ctx.setproperty(-2, "bar");
}

fn main() {
    let mut context = JsContext::new();
    context.register(println_handler, "println", 0);
    context.register(create_object, "create_object", 1);

    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);

                match line.as_ref() {
                    ".exit" => break,
                    _ => {}
                };
                context.run(&line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
