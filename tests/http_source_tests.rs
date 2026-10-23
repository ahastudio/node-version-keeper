use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

use node_version_keeper::http_source::{HttpReleaseSource, ReleaseSource};

// A server that takes one request, returns a fixed response, and closes. Returns the address and thread
fn serve_once(response: &'static str) -> (String, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("should open a port");
    let address = listener.local_addr().expect("should know the address");
    let handle = thread::spawn(move || answer_one_request(listener, response));
    (format!("http://{address}/"), handle)
}

fn answer_one_request(listener: TcpListener, response: &str) {
    let (mut stream, _) = listener.accept().expect("should accept a connection");
    read_request(&mut stream);
    stream.write_all(response.as_bytes()).expect("should write the response");
}

fn read_request(stream: &mut TcpStream) {
    let mut buffer = [0u8; 4096];
    let _request_length = stream.read(&mut buffer).expect("should read the request");
}

#[test]
fn fetches_the_body_of_a_successful_response() {
    let (url, handle) = serve_once("HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\nhello");
    assert_eq!(HttpReleaseSource.fetch_text(&url), Ok("hello".to_string()));
    handle.join().unwrap();
}

#[test]
fn a_not_found_response_is_an_error_naming_the_url() {
    let (url, handle) =
        serve_once("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    let error = HttpReleaseSource.fetch_text(&url).unwrap_err();
    assert!(error.to_string().contains(&url), "{error}");
    handle.join().unwrap();
}

#[test]
fn a_body_cut_short_is_an_error() {
    let (url, handle) =
        serve_once("HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nshort");
    assert!(HttpReleaseSource.fetch_text(&url).is_err());
    handle.join().unwrap();
}

#[test]
fn a_refused_connection_is_an_error() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    drop(listener);
    assert!(HttpReleaseSource.fetch_text(&url).is_err());
}
