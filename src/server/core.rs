use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};

pub fn ouvinte_core(){
    let ouvinte = TcpListener::bind("127.0.0.1:8080")
        .expect("Falha ao vincular ao endereço");

    for stream in ouvinte.incoming() {
        let stream = stream.expect("Erro ao aceitar a conexão");

        lidar_com_conexoes(stream);
    }
}

fn lidar_com_conexoes(mut tcp_stream: TcpStream){
    let leitor_de_buffer = BufReader::new(&mut tcp_stream);
    let http_request = leitor_de_buffer.lines().next().unwrap().unwrap();

    let (status_de_resposta, conteudo_de_resposta) = if http_request.contains("GET /page1") {
        ("HTTP/1.1 200 OK", fs::read_to_string("src/server/page1.html").unwrap())
    } else if http_request.contains("GET /page2") {
        ("HTTP/1.1 200 OK", fs::read_to_string("src/server/page2.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Page Not Found".to_string())
    };

    let tamanho_de_resposta = conteudo_de_resposta.len();

    let resposta = format!("{status_de_resposta}\r\nContent-Length: {tamanho_de_resposta}\r\n\r\n{conteudo_de_resposta}");

    tcp_stream.write_all(resposta.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);
}
