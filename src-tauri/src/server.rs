use std::{io::Write, net::TcpListener, path::PathBuf, thread};

use opencv::{core::Vector, imgcodecs};

use crate::cam::Camera;

pub fn run(resource_path: &PathBuf) {
    let listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(listener) => listener,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let mut cam = Camera::new(0, resource_path).unwrap();
    let mut buf = Vector::new();

    thread::spawn(move || loop {
        let (mut stream, _) = listener.accept().unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n"
        );

        stream.write_all(response.as_bytes()).unwrap();

        loop {
            cam.face_detect().unwrap();
            buf.clear();
            let _ = imgcodecs::imencode(".jpg", &cam.frame, &mut buf, &Vector::new()).unwrap();

            let image_data = format!(
                "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                buf.len()
            );

            stream.write_all(image_data.as_bytes()).unwrap();
            stream.write_all(buf.as_slice()).unwrap();
            stream.write_all(b"\r\n").unwrap();
            stream.flush().unwrap();
        }
    });
}
