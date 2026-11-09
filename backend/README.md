# Backend

API e implementação ActivityPub do blog.

## Rodando localmente

```bash
cp .env.example .env
docker compose -f ../docker-compose.yaml up -d database
cargo run
```

Para subir tudo com Docker (banco, backend e nginx), use o `docker-compose.yaml`
da raiz do repositório. Ele lê `.env` da raiz (credenciais do banco e build do
frontend, ver `.env.example` da raiz) e `backend/.env` para o backend:

```bash
cp ../.env.example ../.env
docker compose -f ../docker-compose.yaml up -d --build
```

As migrations são executadas automaticamente na inicialização. O worker de
entregas também é iniciado automaticamente (`AP_WORKER_ENABLED=true`).

Em desenvolvimento, `BASE_URL` pode ser `http://localhost:5000` e
`BASE_URL_NAME=localhost:5000`. Em produção, use sempre HTTPS:

```env
BASE_URL=https://moang.com.br
BASE_URL_NAME=moang.com.br
FRONTEND_URL=https://moang.com.br
```

`BASE_URL` é usado para montar ids do ActivityPub (actor, objetos, atividades).
Mudar esse valor depois que o perfil já foi federado muda a identidade dos
objetos para os outros servidores.

## Como o blog é federado

- O ator é `GET /users/{username}` (`Person`).
- O WebFinger responde em `GET /.well-known/webfinger?resource=acct:caio@moang.com.br`.
- O outbox contém atividades `Create` com objetos `Article` ou `Note`,
  conforme o campo `kind` do post:
  - `article`: objeto `Article` com trecho em HTML e link para o post completo;
  - `microblog`: objeto `Note` com o conteúdo Markdown completo convertido para
    HTML, para o Mastodon renderizar o texto direto no perfil/timeline.
- Os dois formatos têm:
  - `source.content`: Markdown completo (o frontend usa esse campo);
  - `url`: URL do post no frontend;
  - `replies`, `likes` e `shares`: coleções com `totalItems` para o Mastodon
    mostrar as contagens de respostas, curtidas e reposts.
- Posts públicos e não listados aparecem na coleção pública do outbox.
- Quando um Follow é aceito, os posts existentes (exceto `private`) são enviados
  ao novo seguidor como atividades `Create`. O Mastodon usa o `totalItems` do
  outbox como contagem de publicações, mas não importa os itens do outbox: sem
  esse backfill o perfil mostrava a contagem e nenhum post.
- Entregas usam HTTP Signatures (RSA-SHA256) e ficam em uma fila persistente no
  banco; o worker tenta novamente com backoff.
- O inbox verifica assinatura, `Digest`, deduplica atividades e aceita
  `Follow`, `Undo`, `Accept`, `Reject`, `Create`, `Update`, `Delete`, `Block`,
  `Like` e `Announce`. Curtidas e reposts de posts locais são guardados em
  `remote_interactions`; `Undo` remove a interação correspondente. Ao salvar um
  `Update` que não traz as coleções, as contagens anteriores são mantidas, e a
  audiência (`to`/`cc`) é guardada para remontar a resposta como `Note` válido.

## Endpoints

### Descoberta

| Método | Caminho | Descrição |
| --- | --- | --- |
| GET | `/.well-known/webfinger?resource=acct:caio@moang.com.br` | Descobre o ator |
| GET | `/.well-known/host-meta` | WebFinger em XRD (software mais antigo) |
| GET | `/.well-known/host-meta.json` | WebFinger em JRD |
| GET | `/.well-known/nodeinfo` | Aponta para o NodeInfo 2.1 |
| GET | `/nodeinfo/2.1` | Estatísticas e protocolos da instância |
| GET | `/feed.xml` | Feed RSS 2.0 dos artigos públicos |

O WebFinger aceita `acct:usuario@dominio`, `usuario@dominio` e a própria URL
do ator, com domínio em qualquer caixa. O ator em `/users/{username}` responde
com `Person` para clientes ActivityPub e redireciona navegadores (`text/html`)
para a página "Sobre", como o Mastodon faz.

NodeInfo é um documento público padronizado que diz "o que esta instância é e
quais protocolos fala". Ele não é obrigatório para federar, mas ajuda
ferramentas e diretórios a descobrir que o blog fala ActivityPub.

### Ator e coleções

| Método | Caminho | Descrição |
| --- | --- | --- |
| GET | `/users/{username}` | Ator `Person` com chave pública |
| GET | `/users/{username}/outbox` | Coleção do outbox |
| GET | `/users/{username}/outbox/posts` | Página com atividades `Create` |
| GET | `/users/{username}/outbox/posts/{id}` | Atividade `Create` por id numérico |
| GET | `/users/{username}/followers` | Seguidores aceitos |
| GET | `/users/{username}/following` | Perfis seguidos aceitos |

### Objetos, respostas e timeline

| Método | Caminho | Descrição |
| --- | --- | --- |
| GET | `/users/{username}/posts/{uuid}` | Objeto `Article`/`Note` canônico |
| GET | `/users/{username}/posts/{uuid}/replies` | Respostas remotas ao post |
| GET | `/users/{username}/posts/{uuid}/likes` | Atores que curtiram o post |
| GET | `/users/{username}/posts/{uuid}/shares` | Atores que repostaram o post |
| GET | `/users/{username}/timeline` | Posts remotos recebidos, mais recentes primeiro |

`/likes`, `/shares` e `/replies` são coleções `OrderedCollection` com
`@context`. A coleção de respostas negocia pelo cabeçalho `Accept`:

- `application/activity+json` ou `application/ld+json`: devolve `Note`s AS2
  completos (`@context`, `attributedTo`, `inReplyTo`, `to`, `cc`, `content`,
  `published`, `url`, `summary`, `sensitive`, contagens), além de `orderedItems`
  e de uma página `first` embutida para consumidores como o Mastodon.
- `application/json` (o frontend): devolve o JSON compacto com `actor`,
  `avatarUrl` e as contagens que os cards exibem.

`/timeline` é um endpoint JSON da aplicação, mantido sem link público no
frontend para uma futura visualização autenticada.

### Inbox e ações autenticadas

| Método | Caminho | Descrição |
| --- | --- | --- |
| POST | `/users/{username}/inbox` | Inbox pessoal (ActivityPub) |
| POST | `/inbox` | Shared inbox (ActivityPub) |
| POST | `/users/{username}/outbox` | Cria post (sessão), com `kind`: `article` ou `microblog` |
| PUT | `/users/{username}/outbox/posts/{id}` | Atualização parcial do post e federa `Update` (sessão) |
| POST | `/users/{username}/following` | Segue um ator remoto (sessão), corpo `{"actor":"https://..."}` |
| DELETE | `/users/{username}/following` | Deixa de seguir (sessão), mesmo corpo |
| PUT | `/users/{username}/profile` | Atualiza perfil e federa `Update` do ator (sessão) |

Todas as ações autenticadas conferem não só a sessão, mas também se o recurso
pertence ao usuário logado (dono do post, do perfil, da conta ou da relação de
seguimento). A criação de posts usa `current_user_id` para definir o autor, e
falhas ao escrever o espelho Gemini são apenas registradas no log: o post é
salvo e federado mesmo sem `content/index.gmi`. Ao apagar um post, as respostas
e interações remotas ligadas ao objeto são removidas junto.

## Banco de dados

- `users`: dados do perfil (`display_name`, `summary`, `avatar_url`, `header_url`, `is_locked`, `is_discoverable`).
- `posts`: conteúdo local, `ap_id` (UUID canônico), `visibility`, `kind`
  (`article` ou `microblog`) e `updated_at`.
- `actor_keys`: par de chaves RSA por usuário.
- `remote_actors`: cache de atores remotos e suas chaves públicas.
- `followers` / `following`: relações locais com atores remotos.
- `activities`: atividades enviadas e recebidas (deduplicação e histórico).
- `deliveries`: fila de entrega com tentativas e backoff.
- `remote_posts`: posts e respostas recebidos pelo inbox, incluindo as
  contagens de curtidas, reposts e respostas informadas pelo servidor de origem
  e a audiência (`to_addresses`/`cc_addresses`) usada para republicar a nota.
- `remote_interactions`: curtidas (`like`) e reposts (`repost`) recebidos para
  posts locais, uma linha por ator/objeto/tipo.

## Visibilidade

| Valor | `is_public` | `to` | `cc` |
| --- | --- | --- | --- |
| `public` | true | `#Public` | seguidores |
| `unlisted` | true | seguidores | `#Public` |
| `followers` | false | seguidores | vazio |
| `private` | false | vazio | vazio, não federado |

## Foto de perfil e banner

O ator publica `avatar_url` como `icon` e `header_url` como `image`. As imagens
são URLs públicas; a forma mais simples é colocá-las em `frontend/public`
(ex.: `profile.jpg` e `banner.jpg`) para o nginx servir na raiz do domínio.

Depois de publicar os arquivos, atualize o perfil:

```bash
curl -c /tmp/cookies -s -X POST https://moang.com.br/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"caio@moang.com.br","password":"SUA_SENHA"}'

curl -b /tmp/cookies -s -X PUT https://moang.com.br/users/caio/profile \
  -H 'Content-Type: application/json' \
  -d '{
    "display_name": "Caio",
    "summary": "Bio que aparece no perfil do Mastodon",
    "avatar_url": "https://moang.com.br/profile.jpg",
    "header_url": "https://moang.com.br/banner.jpg"
  }'
```

Campos omitidos não são alterados; envie `""` para remover uma imagem. A
atualização envia `Update` do ator para os seguidores, então instâncias remotas
atualizam nome, bio, avatar e banner. O Mastodon também atualiza o perfil
periodicamente sozinho.

## Artigo ou microblog

`kind` escolhe como o post é federado:

| `kind` | Objeto ActivityPub | Comportamento no Mastodon |
| --- | --- | --- |
| `article` (padrão) | `Article` | título + link para o post no blog |
| `microblog` | `Note` | conteúdo completo renderizado no post |

Exemplo de post de microblog:

```bash
curl -b /tmp/cookies -s -X POST https://moang.com.br/users/caio/outbox \
  -H 'Content-Type: application/json' \
  -d '{
    "title": "Nota rápida",
    "body": "Uma nota curta federada no **Mastodon**.",
    "visibility": "public",
    "is_public": true,
    "kind": "microblog"
  }'
```

O `PUT` é uma atualização parcial: todos os campos (`title`, `body`,
`visibility`, `kind`) são opcionais e os omitidos mantêm o valor atual. Para
trocar só o formato:

```bash
curl -b /tmp/cookies -s -X PUT https://moang.com.br/users/caio/outbox/posts/123 \
  -H 'Content-Type: application/json' \
  -d '{"kind":"microblog"}'
```

Posts antigos continuam `article` por causa do default da migration.

## Feed RSS

`GET /feed.xml` gera um RSS 2.0 com os artigos públicos mais recentes (até 20),
datas em RFC 2822, `guid` permanente e descrição em texto limpo derivada do
Markdown. Notas (`kind = "microblog"`) ficam fora do feed. O XML é montado a
partir do banco a cada requisição, então novos artigos entram automaticamente.
Cada item também traz `<content:encoded>` com o HTML completo do post. O feed inclui `<generator>moang-blog</generator>`, `ETag` forte com
`If-None-Match` (responde `304 Not Modified`) e `Cache-Control: public,
max-age=300`. O header do site e o `<link rel="alternate">` do `index.html`
apontam para ele. No nginx de exemplo, `/feed.xml` precisa ser proxyado para o
backend (por causa do fallback do SPA).

## Frontend

O frontend lê `VITE_API_BASEURL` e `VITE_BLOG_USERNAME` (padrão `caio`). Todas
as chamadas passam por `userEndpoint()` em `src/lib/blog.ts`, então trocar o
username não exige editar página por página.

Rotas:

| Rota | Página |
| --- | --- |
| `/` | `Home`: banner de apresentação + publicações |
| `/posts` | Redireciona para `/` (compatibilidade com links antigos) |
| `/post/{id}` | `Post`: publicação individual e respostas |
| `/about` | `About` |

A Home usa o componente `PostsFeed`, que separa Artigos e Notas em abas e
filtra os posts já carregados por título e conteúdo, sem endpoint de busca
novo. A lista mostra apenas título e data. As contagens de
curtidas/reposts/respostas aparecem na página do post e nos cards de respostas
do fediverso.

## Deploy em produção

O frontend e o ActivityPub precisam do mesmo domínio para o handle
`@caio@moang.com.br`. Use o proxy de exemplo em `infra/nginx.conf.example`:
o SPA fica na raiz e apenas os caminhos de API/ActivityPub vão para o backend.

Checklist:

1. Configure `BASE_URL=https://moang.com.br` e `BASE_URL_NAME=moang.com.br`.
2. Faça o build do frontend com `VITE_API_BASEURL=https://moang.com.br` e
   `VITE_BLOG_USERNAME=caio`.
3. Rode as migrations (automático no startup) e mantenha o worker habilitado.
4. Libere HTTPS público para `/.well-known/*`, `/users/*`, `/inbox`,
   `/nodeinfo/*` e `/feed.xml`.
5. Confira `https://moang.com.br/.well-known/webfinger?resource=acct:caio@moang.com.br`.

## Testando

```bash
curl -s 'https://moang.com.br/.well-known/webfinger?resource=acct:caio@moang.com.br' | jq
curl -s https://moang.com.br/users/caio | jq
curl -s https://moang.com.br/users/caio/outbox/posts | jq
curl -s https://moang.com.br/feed.xml
curl -s -H 'Accept: application/activity+json' \
  https://moang.com.br/users/caio/posts/{uuid}/replies | jq

# Testes unitários (parsing do inbox, coleções, feed, descoberta)
cargo test --lib
```

No Mastodon, procure por `@caio@moang.com.br`. Para testar a entrega, siga o
perfil pelo Mastodon; o Follow chega no inbox, é aceito e o `Accept` é enviado.

## Limitações atuais

- Este backend implementa o lado servidor (S2S) do ActivityPub, não o lado
  cliente (C2S/OAuth).
- As chaves privadas ficam no banco em texto puro. Isso é aceitável para uma
  instância pessoal, mas o ideal futuro é usar um secret manager ou criptografia
  em repouso.
- `is_locked` ainda não tem uma tela de aprovação de seguidores. Quando o perfil
  não é travado, o Follow é aceito automaticamente.
