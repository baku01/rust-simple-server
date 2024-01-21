# Mini Servidor HTTP em Rust 🦀

Este é um mini projeto de um servidor HTTP simples implementado em Rust, capaz de lidar com diferentes páginas. Ele usa a biblioteca padrão do Rust e é destinado apenas para fins educacionais ou experimentais.

## Pré-requisitos 🛠️

Certifique-se de ter o Rust instalado em sua máquina. Você pode instalá-lo a partir do site oficial: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Como Executar ▶️

1. **Clone o repositório:**

   ```bash
   git clone https://github.com/baku01/rust-simple-server.git
   ```

2. **Navegue até o diretório do projeto:**

   ```bash
   cd rust-simple-server
   ```

3. **Execute o servidor:**

   ```bash
   cargo run
   ```

4. **Abra um navegador e acesse:**
    - Página 1: `http://127.0.0.1:8080/page1`
    - Página 2: `http://127.0.0.1:8080/page2`

## Estrutura do Projeto 📁

- `src/` : Contém o código fonte do projeto.
    - `server/` : Contém os arquivos HTML das páginas servidas pelo servidor.

## Funcionamento do Código 🧑‍💻

### 1. Ouvinte Core (`ouvinte_core`)

O `ouvinte_core` é responsável por criar um ouvinte TCP na porta 8080 e aguardar por conexões. Quando uma conexão é estabelecida, ele chama a função `lidar_com_conexoes` para lidar com essa conexão. Aqui está o trecho de código:

```rust
pub fn ouvinte_core() {
    let ouvinte = TcpListener::bind("127.0.0.1:8080").expect("Falha ao vincular ao endereço");

    for stream in ouvinte.incoming() {
        let stream = stream.expect("Erro ao aceitar a conexão");
        lidar_com_conexoes(stream);
    }
}
```

- `TcpListener::bind("127.0.0.1:8080")`: Cria um ouvinte TCP na interface local (`127.0.0.1`) e na porta `8080`.
- `ouvinte.incoming()`: Retorna um iterador que aguarda por conexões entrantes.

### 2. Lidar com Conexões (`lidar_com_conexoes`)

A função `lidar_com_conexoes` é chamada para cada conexão recebida. Ela cria um leitor de buffer para a conexão TCP, lê a primeira linha da solicitação HTTP e decide qual página servir com base na URL. Aqui está o trecho de código:

```rust
fn lidar_com_conexoes(mut tcp_stream: TcpStream) {
    // Cria um leitor de buffer para a conexão TCP
    let leitor_de_buffer = BufReader::new(&mut tcp_stream);

    // Lê a primeira linha da solicitação HTTP
    let http_request = leitor_de_buffer.lines().next().unwrap().unwrap();

    // Lógica para determinar qual página servir
    let (status_de_resposta, conteudo_de_resposta) = if http_request.contains("GET /page1") {
        ("HTTP/1.1 200 OK", fs::read_to_string("src/server/page1.html").unwrap())
    } else if http_request.contains("GET /page2") {
        ("HTTP/1.1 200 OK", fs::read_to_string("src/server/page2.html").unwrap())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Page Not Found".to_string())
    };

    // Obtém o tamanho da resposta
    let tamanho_de_resposta = conteudo_de_resposta.len();

    // Gera a resposta HTTP
    let resposta = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_de_resposta, tamanho_de_resposta, conteudo_de_resposta);

    // Envia a resposta
    tcp_stream.write_all(resposta.as_bytes()).unwrap();
}
```

- `BufReader::new(&mut tcp_stream)`: Cria um leitor de buffer para a conexão TCP.
- `leitor_de_buffer.lines().next().unwrap().unwrap()`: Lê a primeira linha da solicitação HTTP.
- Lógica para determinar qual página servir com base na solicitação.
- Gera uma resposta HTTP com o status, tamanho e conteúdo da resposta.
- `tcp_stream.write_all(resposta.as_bytes()).unwrap()`: Envia a resposta de volta ao cliente.

## Contribuições 🤝

Contribuições são bem-vindas! Se encontrar algum problema ou tiver sugestões, sinta-se à vontade para abrir uma issue ou enviar um pull request.

