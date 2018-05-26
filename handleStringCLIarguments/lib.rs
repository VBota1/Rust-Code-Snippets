fn handle_arguments() -> Result<String,String> {
    match args.get(1) {
        Some(command) => {
            match command.as_str() {
                "expected" => {
                    Err ( log_stub (format!("Command \"expected\" is not implemented")) );
                }
            }
        },
        None => {
            Err( log_stub(format!("No command received. {}", recommend_help_stub())) );
        },
    };
}

fn log_stub (message: String) -> String {
    //TODO log message
    message
}

fn recommend_help_stub() -> String {
    format!("Call \"help\" for aditional information.")
}
