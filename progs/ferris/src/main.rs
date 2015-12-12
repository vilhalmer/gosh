/*
	Ferris - a simple http server wrapper built for gosh
	Author: Stephen Radachy <sjradach@mtu.edu>
	
	Referenced material:
	https://crates.io/crates/tiny_http/
	https://github.com/frewsxcv/tiny-http/blob/master/examples/serve-root.rs
	
	How to use:
	1. Run `cargo build` in <path-to-repo>/progs/ferris
	2. Run `export PATH=$PATH:<path-to-repo>/progs/ferris/target/debug`
	3. Run `cargo run` in <path-to-repo>
	4. Now you are running gosh with ferris accessible via the path environment variable!
	5. Run `ferris <parameters>`
	6. PROFIT
	
	There are 3 manipulatable parameters for ferris:
	ip (default: 0.0.0.0) - the ip address ferris binds to
	port (default: 8000) - the port ferris binds to
	root (default: cwd) - path to the directory ferris serves files from
	
	example commands:
	ferris
	ferris ip:192.168.1.1 root:/var/www/html
	ferris port:3000 root:/home/sjradach/website
	ferris ip:127.0.0.1 port:80 root:/var/www/website
	
	to test:
	open http://ip:port/file.txt from your favorite browser
	NOTE: if bound to 0.0.0.0, address should be http://localhost:port/file.txt
*/

use std::path::Path;
use std::fs;
use std::env;

extern crate ascii;
extern crate tiny_http;

fn main() {
    use ascii::AsciiCast;
	
	let ip;
	let port;
	let root;
	
	// grab parameters from gosh or set to default values
	// format!("{}",e); is ugly and used just to get rust
	// to not emit any warnings during compilation.
	// please ignore it.
	match env::var("ip") {
		Ok(val) => {ip = val},
		Err(e) => {ip = "0.0.0.0".to_string();format!("{}",e);},
	}
	match env::var("port") {
		Ok(val) => {port = val},
		Err(e) => {port = "8000".to_string();format!("{}",e);},
	}
	match env::var("root") {
		Ok(val) => {root = val},
		Err(e) => {root = format!("{}", env::current_dir().unwrap().display());format!("{}",e);},
	}
	
	// instantiate server!
	let config = format!("{}:{}", ip, port);
    let server = tiny_http::Server::http(&*config).unwrap();
    println!("Now listening on {}:{}", ip, port);

    loop {
		// get a request
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break
        };
		
		// log the trequest
        println!("{:?}", rq);

		// put together file to be served
        let url = format!("{}{}", root, rq.url().to_string());
        let path = Path::new(&url);
        let file = fs::File::open(&path);

        if file.is_ok() {
			// build response
            let response = tiny_http::Response::from_file(file.unwrap());
            let response = response.with_header(
                tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: get_content_type(&path).to_ascii().unwrap().to_ascii_string()
                }
            );
			
			// send response
            let _ = rq.respond(response);

        } else {
			// File not found!
            let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
            // send response
			let _ = rq.respond(rep);
        }
    }
}


// handle content types
fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
		"json" => "application/json; charset=utf8",
        _ => "text/plain; charset=utf8"
    }
}