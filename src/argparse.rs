pub mod argparser {

    pub struct Config {
        pub input_filename: String,
        pub output_filename: String,
        pub binary: bool,
        pub interactive: bool,
        pub verbose: bool,
    }

    fn print_help_message() {
        let help_msg = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "usage: ",
            "  rust_huffman_compression --interactive",
            "  rust_huffman_compression -i <input filename> [optional args]",
            "required",
            "  --interactive\t\tuse cli input",
            "  -i, --input\t\tuse file input",
            "optional args:",
            "  -o, --output\t\toutput filename",
            "  -b, --binary\t\tread input file as binary file",
            "\t\t\tthis option should be used when compressing large files",
            "\t\t\tor when compressing other filetypes than text files",
            "  -v, --verbose\t\tincrease verbosity",
            "  -h, --help\t\tprint help message and exit",
        );
        println!("{}", help_msg);
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
            binary: false,
            interactive: false,
            verbose: false,
        };

        let arguments = vec![
            String::from("-i"),
            String::from("--input"),
            String::from("-o"),
            String::from("--output"),
            String::from("-b"),
            String::from("--binary"),
            String::from("--interactive"),
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
                    argument if argument == arguments.get(4).unwrap() => config.binary = true,
                    argument if argument == arguments.get(5).unwrap() => config.binary = true,
                    argument if argument == arguments.get(6).unwrap() => config.interactive = true,
                    argument if argument == arguments.get(7).unwrap() => config.verbose = true,
                    argument if argument == arguments.get(8).unwrap() => config.verbose = true,
                    argument if argument == arguments.get(9).unwrap() => self::print_help_message(),
                    argument if argument == arguments.get(10).unwrap() => self::print_help_message(),
                    _ => (),
                },
                None => {
                    continue;
                }
            }
        }

        if config.interactive {
            return config
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
