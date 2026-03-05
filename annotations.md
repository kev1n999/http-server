# Protocolo HTTP - Minhas anotações sobre o que eu aprendi/entendi sobre o protócolo HTTP(Foi eu quem escrevi, então pode haver erros/equívocos)

# O que é o protocolo HTTP?
> O HTTP(HyperText Transfer Protocol) é um protocolo para transmitir dados na web;
> Ele funciona em cima da pilha de protocólos TCP/IP, que é o principal protocolo usado na camada de transporte de dados, sendo responsável por transmitir os dados de forma segura e confiável.
> Assim como todo protocólo por definição, o HTTP estabelece um conjunto de regras específicas e bem rigídas sobre como os dados devem ser enviados
---

# Como ele funciona na prática
> O HTTP é baseado na comunicação de cliente x servidor, onde o servidor é o responsável por entregar recursos para o cliente de acordo como solicitado pelo mesmo;
---

## Cliente 
> O cliente http é geralmente utilizado/implementado por meio de um navegador, ele faz requisições/pedidos ao servidor específicando as regras como o recurso que queremos usar, qual operação queremos que o servidor faça, e quais dados queremos enviar para ele processar 
---
## Servidor 
> O Servidor é responsável por receber as requisições do cliente, como dados e recursos que são definidos na requisição; Ele recebe a solicitação do cliente, processa com base no que foi pedido, e retorna uma saída para o cliente
---

# Requisição HTTP 
> Uma requisição HTTP é basicamente um pedido do cliente/browser para o servidor, usado quando o cliente deseja solicitar algum recurso

# Resposta HTTP 
> Uma resposta HTTP acontece depois que uma requisição HTTP é feita pelo cliente; O servidor recebe a requisição(com as regras estabelecidas pelo protocólo), e a partir disso, ele envia uma resposta com base no pedido feito pelo cliente

## Exemplo de como funciona a estrutura de requisições http e respostas http(Basicamente como funciona a troca de informações e dados entre cliente e servidor neste protocólo)
<img src="https://mdn.github.io/shared-assets/images/diagrams/http/messages/http-message-anatomy.svg">

> - No exemplo da esquerda, é representado como funciona a estrutura de uma requisição http, nele podemos ver inicialmente a request line(linha de requisição), onde apresentamos o método HTTP, rota e versão do http; abaixo, o header(cabeçalho) da requisição, onde enviamos metadados como  a host, tipo de conteúdo, tamanho do conteúdo em bytes e entre outros; e após a linha invisível, o body(corpo) da requisição, como estamos usando o método POST no exemplo, precisamos enviar o conteúdo contendo os dados que queremos enviar ao servidor, respeitando os metadados do header, como no Content-Type definimos que ele será do tipo json, então enviamos um conteúdo em formato JSON no corpo da requisição
---
> - Já no exemplo da direita, é representado como funciona a estrutura de uma resposta http, dessa vez, na primeira linha, podemos ver a response line(linha de resposta), onde o servidor define a versão do protocólo, e o código de resposta(Indica como a requisição do cliente foi processada pelo servidor), logo abaixo(e não muito diferente da requisiçaõ http), temos o cabeçalho da resposta, contendo os metadados da mesma, a linha de separação e abaixo, o conteúdo que será enviado para o cliente como resposta, no caso, como definido no Content-Type, um HTML 
---

## Códigos de Resposta 
> Os códigos de resposta são basicamente números específicos que o protocólo HTTP define para representar o status final do processamento de uma requisição pelo cliente, cada número representa um estado diferente, veja mais abaixo
---
### 1xx — Informacional
- **100 Continue** — O cliente pode continuar enviando a requisição.
- **101 Switching Protocols** — O servidor aceitou mudar de protocolo.

### 2xx — Sucesso
- **200 OK** — Requisição processada com sucesso.
- **201 Created** — Recurso criado com sucesso.
- **204 No Content** — Sucesso, mas sem conteúdo para retornar.

### 3xx — Redirecionamento
- **301 Moved Permanently** — Recurso movido permanentemente.
- **302 Found** — Redirecionamento temporário.
- **304 Not Modified** — Recurso não foi modificado.

### 4xx — Erro do Cliente
- **400 Bad Request** — Requisição inválida.
- **401 Unauthorized** — Autenticação necessária.
- **403 Forbidden** — Acesso proibido.
- **404 Not Found** — Recurso não encontrado.

### 5xx — Erro do Servidor
- **500 Internal Server Error** — Erro interno do servidor.
- **502 Bad Gateway** — Gateway recebeu resposta inválida.
- **503 Service Unavailable** — Serviço indisponível.
- **504 Gateway Timeout** — Tempo de resposta excedido.
---

# Métodos HTTP
> Os métodos HTTP definem o tipo de operação que o cliente deseja que o servidor faça/ou ofereça
## Quais são esses métodos?
* GET: O método GET define uma solicitação de leitura, ele é o principal método utilizado na maior parte das vezes quando queremos pedir algo visual ao servidor, que na maioria dos casos é um HTML(Acompanhado de CSS e JavaScript muitas vezes), imagens e entre outros tipos de arquivos
* POST: O método POST define um tipo de solicitação onde o cliente deseja enviar dados para o servidor processar, realizar uma operação específica com esses dados(isso vai ser definido pela rota da requisição) e retornar uma resposta a partir disso
* PUT: Define uma soliticação para atualizar dados específicos que geralmente estão armazenados em um banco de dados
* PATCH: Também usado para atualizar dados, mas de forma parcial, ao invés de atualizar todos os campos como o método PUT faz, ele atualiza um conjunto parcial desses campos/dados 
* DELETE: Define uma solicitação onde você quer deletar um dado/informação específica que também está armazenado em um banco de dados, como por exemplo quando queremos deletar uma conta de usuário, o servidor recebe os dados da requisição, e faz uma query no banco de dados para deletar o dado que foi solicitado; mas atenção: 
---
