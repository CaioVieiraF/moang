# Backend

API e implementação ActivityPub do blog.

## Rodando localmente

```bash
cp .env.example .env
docker compose -f infra/compose.yaml up -d
cargo run
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
- O outbox contém atividades `Create` com objetos `Article`.
- Cada `Article` tem:
  - `content`: trecho em HTML com link para o post completo no frontend;
  - `source.content`: Markdown completo (o frontend usa esse campo);
  - `url`: URL do post no frontend.
- Posts públicos e não listados aparecem na coleção pública do outbox.
- Quando um Follow é aceito, os posts existentes (exceto `private`) são enviados
  ao novo seguidor como atividades `Create`. O Mastodon usa o `totalItems` do
  outbox como contagem de publicações, mas não importa os itens do outbox: sem
  esse backfill o perfil mostrava a contagem e nenhum post.
- Entregas usam HTTP Signatures (RSA-SHA256) e ficam em uma fila persistente no
  banco; o worker tenta novamente com backoff.
- O inbox verifica assinatura, `Digest`, deduplica atividades e aceita
  `Follow`, `Undo`, `Accept`, `Reject`, `Create`, `Update`, `Delete` e `Block`.

## Endpoints

### Descoberta

| Método | Caminho | Descrição |
| --- | --- | --- |
| GET | `/.well-known/webfinger?resource=acct:caio@moang.com.br` | Descobre o ator |
| GET | `/.well-known/nodeinfo` | Aponta para o NodeInfo 2.1 |
| GET | `/nodeinfo/2.1` | Estatísticas e protocolos da instância |

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
| GET | `/users/{username}/posts/{uuid}` | Objeto `Article` canônico |
| GET | `/users/{username}/posts/{uuid}/replies` | Respostas remotas ao post |
| GET | `/users/{username}/timeline` | Posts remotos recebidos, mais recentes primeiro |

`/timeline` e `/replies` são endpoints JSON da aplicação (não fazem parte do
ActivityPub). Eles servem para o frontend ler o que chegou pelo inbox.

### Inbox e ações autenticadas

| Método | Caminho | Descrição |
| --- | --- | --- |
| POST | `/users/{username}/inbox` | Inbox pessoal (ActivityPub) |
| POST | `/inbox` | Shared inbox (ActivityPub) |
| POST | `/users/{username}/outbox` | Cria post (sessão) |
| PUT | `/users/{username}/outbox/posts/{id}` | Edita post e federar `Update` (sessão) |
| POST | `/users/{username}/following` | Segue um ator remoto (sessão), corpo `{"actor":"https://..."}` |
| DELETE | `/users/{username}/following` | Deixa de seguir (sessão), mesmo corpo |
| PUT | `/users/{username}/profile` | Atualiza perfil e federa `Update` do ator (sessão) |

## Banco de dados

- `users`: dados do perfil (`display_name`, `summary`, `avatar_url`, `header_url`, `is_locked`, `is_discoverable`).
- `posts`: conteúdo local, `ap_id` (UUID canônico), `visibility` e `updated_at`.
- `actor_keys`: par de chaves RSA por usuário.
- `remote_actors`: cache de atores remotos e suas chaves públicas.
- `followers` / `following`: relações locais com atores remotos.
- `activities`: atividades enviadas e recebidas (deduplicação e histórico).
- `deliveries`: fila de entrega com tentativas e backoff.
- `remote_posts`: posts e respostas recebidos pelo inbox.

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
(ex.: `profile.png` e `banner.jpg`) para o nginx servir na raiz do domínio.

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
    "avatar_url": "https://moang.com.br/profile.png",
    "header_url": "https://moang.com.br/banner.jpg"
  }'
```

Campos omitidos não são alterados; envie `""` para remover uma imagem. A
atualização envia `Update` do ator para os seguidores, então instâncias remotas
atualizam nome, bio, avatar e banner. O Mastodon também atualiza o perfil
periodicamente sozinho.

## Deploy em produção

O frontend e o ActivityPub precisam do mesmo domínio para o handle
`@caio@moang.com.br`. Use o proxy de exemplo em `infra/nginx.conf.example`:
o SPA fica na raiz e apenas os caminhos de API/ActivityPub vão para o backend.

Checklist:

1. Configure `BASE_URL=https://moang.com.br` e `BASE_URL_NAME=moang.com.br`.
2. Faça o build do frontend com `VITE_API_BASEURL=https://moang.com.br`.
3. Rode as migrations (automático no startup) e mantenha o worker habilitado.
4. Libere HTTPS público para `/.well-known/webfinger`, `/users/*` e `/inbox`.
5. Confira `https://moang.com.br/.well-known/webfinger?resource=acct:caio@moang.com.br`.

## Testando

```bash
curl -s 'https://moang.com.br/.well-known/webfinger?resource=acct:caio@moang.com.br' | jq
curl -s https://moang.com.br/users/caio | jq
curl -s https://moang.com.br/users/caio/outbox/posts | jq
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
