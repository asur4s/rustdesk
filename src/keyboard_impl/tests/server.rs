#[cfg(test)]
mod server {
    use std::{
        io::{BufReader, Read},
        net::{TcpListener, TcpStream},
    };

    use hbb_common::{anyhow, env_logger, log};

    fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut http_request = Vec::new();
        let _size = buf_reader.read_to_end(&mut http_request)?;

        dbg!(_size);

        Ok(())
    }

    #[test]
    fn test_keyboard_simulater() -> anyhow::Result<()> {
        init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "info"));
        std::env::set_var("DISPLAY", ":0");

        let listener = TcpListener::bind("0.0.0.0:7878")?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stream: TcpStream = stream;
                    if let Err(err) = handle_connection(stream) {
                        log::error!("simulate err: {:?}", err);
                    }
                }

                Err(err) => {
                    log::error!("Faile to process stream: {:?}", err);
                    break;
                }
            }
        }
        Ok(())
    }
}
