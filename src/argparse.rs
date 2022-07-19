pub mod argparser {

    pub struct Config {
        pub input_filename: String,
        pub output_filename: String,
        pub verbose: bool,
    }

    fn print_help_message() {
        println!("Something something help");
        std::process::exit(0);
    }

    fn get_arg_index(args: &Vec<String>, arg: &String) -> Option<usize> {
        args.into_iter().position(|x| x == arg)
    }

    fn get_arg_value(args: &Vec<String>, arg: &String) -> String {
        let index = self::get_arg_index(args, arg).unwrap();
        let error_msg = &format!("Missing argument value for argument {}", arg);
        args.get(index + 1).expect(error_msg).clone()
    }

    pub fn parse_args(args: &Vec<String>) -> Config {
        let mut config = Config {
            input_filename: String::from(""),
            output_filename: String::from(""),
            verbose: false,
        };

        let arguments = vec![
            String::from("-i"),
            String::from("--input"),
            String::from("-o"),
            String::from("--output"),
            String::from("-v"),
            String::from("--verbose"),
            String::from("-h"),
            String::from("--help"),
        ];

        for arg in &arguments {
            match self::get_arg_index(args, &arg) {
                Some(index) => match args.get(index).unwrap() {
                    argument if argument == arguments.get(0).unwrap() => config.input_filename = self::get_arg_value(args, &arg),
                    argument if argument == arguments.get(1).unwrap() => config.input_filename = self::get_arg_value(args, &arg),
                    argument if argument == arguments.get(2).unwrap() => config.output_filename = self::get_arg_value(args, &arg),
                    argument if argument == arguments.get(3).unwrap() => config.output_filename = self::get_arg_value(args, &arg),
                    argument if argument == arguments.get(4).unwrap() => config.verbose = true,
                    argument if argument == arguments.get(5).unwrap() => config.verbose = true,
                    argument if argument == arguments.get(6).unwrap() => self::print_help_message(),
                    argument if argument == arguments.get(7).unwrap() => self::print_help_message(),
                    _ => (),
                },
                None => {
                    continue;
                }
            }
        }

        if config.input_filename.len() == 0 {
            panic!("Input filename is required")
        }

        if config.output_filename.len() == 0 {
            config.output_filename = format!("{}{}", config.input_filename, ".hfm");
        }

        config
    }
}
