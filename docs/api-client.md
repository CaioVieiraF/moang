# Contrato da API para o app de escrita

Este documento descreve a API que um app de escrita separado consome para
login, publicação, perfil, seguidores e timeline. O frontend deste repositório é
somente de leitura; ele não usa nada daqui além dos endpoints públicos de
leitura.

O backend implementa ActivityPub **S2S** (servidor para servidor), não C2S. O
app de escrita é um cliente próprio sobre esta API, não um cliente C2S/OAuth
genérico.

- Produção: `https://moang.com.br`
- Desenvolvimento: `http://localhost:5000` ou `http://127.0.0.1:5000`
- Formato: JSON (`Content-Type: application/json`) nas rotas de aplicação

## Autenticação

A sessão é um cookie gerenciado pelo `actix-session`, criado por `POST /login`.
Todas as requisições autenticadas precisam enviar esse cookie:

```js
// axios
api.defaults.withCredentials = true

// fetch
fetch(url, { credentials: 'include' })
```

Não há token no corpo da resposta nem endpoint de logout. As rotas autenticadas
conferem a sessão **e** se o recurso pertence ao usuário logado.

### Origens e CORS

O CORS libera `https://moang.com.br`, `https://dev.moang.com.br`,
`http://localhost:*` e `http://127.0.0.1:*`, com credenciais. Uma origem nova
precisa ser adicionada em `backend/src/main.rs`.

O cookie é host-only e `SameSite=Lax`. Por isso, o caminho mais simples é
hospedar o app em um subdomínio do mesmo site, por exemplo
`escrever.moang.com.br`: ele continua "same-site" com a API e o cookie funciona
sem mudanças. Em um domínio de outro site seria preciso `SameSite=None; Secure`,
proteção contra CSRF, ou trocar para bearer token (que ainda não existe).

## Erros

| Status | Significado |
| --- | --- |
| `400` | Corpo inválido (`visibility`/`kind` desconhecido, URL inválida) |
| `401` | Sem sessão válida |
| `403` | Sessão válida, mas o recurso é de outra conta |
| `404` | Recurso não encontrado (ou post privado sem sessão) |
| `500` | Erro interno; o corpo costuma ser texto simples |

Os corpos de erro são vazios ou texto simples, não JSON estruturado.

## Login

### `POST /login`

```json
{ "email": "caio@moang.com.br", "password": "SUA_SENHA" }
```

| Status | Resposta |
| --- | --- |
| `200` | Corpo vazio e cookie de sessão definido |
| `204` | Já existe sessão ativa; nada muda |
| `401` | Senha incorreta |
| `404` | E-mail não cadastrado |
| `500` | Erro interno |

A sessão expira em 1 dia.

## Posts

### `POST /users/{username}/outbox`

Cria um post. Exige sessão e que `username` seja o ator logado.

```json
{
  "title": "Nota rápida",
  "body": "Texto em **Markdown**.",
  "is_public": true,
  "visibility": "public",
  "kind": "microblog"
}
```

| Campo | Obrigatório | Valores |
| --- | --- | --- |
| `title` | sim | texto |
| `body` | sim | Markdown completo |
| `is_public` | sim | boolean |
| `visibility` | não | `public`, `unlisted`, `followers`, `private` |
| `kind` | não | `article` (padrão) ou `microblog` |

Sem `visibility`, o valor é `public` quando `is_public` é `true` e `private`
caso contrário.

Resposta: `201 Created` com o post criado, incluindo o `id` usado nas rotas de
edição e exclusão:

```json
{
  "id": 123,
  "title": "Nota rápida",
  "body": "Texto em **Markdown**.",
  "is_public": true,
  "slug": "nota-rapida",
  "created_at": "2026-11-09T12:00:00+00:00",
  "author": "4f0c...",
  "ap_id": "9b1e...",
  "updated_at": "2026-11-09T12:00:00+00:00",
  "visibility": "public",
  "kind": "microblog"
}
```

O `slug` é gerado a partir do título (`slugify`) e é único. Títulos repetidos
falham com `500` no insert.

### `PUT /users/{username}/outbox/posts/{id}`

Atualização parcial: campos omitidos mantêm o valor atual.

```json
{ "body": "Texto revisado.", "visibility": "unlisted" }
```

Resposta: `200` com o post no mesmo formato da criação, com datas em ISO 8601.

Trocar o `title` **não** recalcula o `slug`.

### `GET /users/{username}/outbox/posts`

Lista os posts públicos e não listados (`is_public = true`) como
`OrderedCollectionPage` de atividades `Create`. Não exige sessão.

- O Markdown completo está em `orderedItems[].object.source.content`.
- Posts `private` e `followers` **não aparecem** nesta lista. Para editar um
  rascunho, guarde o `id` devolvido na criação.
- Não há paginação: devolve todos.

### `GET /users/{username}/outbox/posts/{id}`

Devolve a atividade `Create` de um post pelo id numérico. Post `private` exige
sessão; sem ela, responde `404`.

### `PUT /{post_id}/make_public`

Torna o post público (`visibility = "public"`) e federa `Create` ou `Update`.
Exige sessão e ownership. Resposta: `202` sem corpo.

### `DELETE /{post_id}`

Apaga o post, federa `Delete` e remove respostas e interações recebidas. Exige
sessão e ownership. Resposta: `200` sem corpo.

As duas rotas acima ficam na raiz (`/{post_id}`), não sob `/users/...`; é uma
inconsistência conhecida da API.

## Perfil

### `GET /users/{username}`

Com `Accept: application/activity+json` (ou `ld+json`), devolve o ator `Person`:

```json
{
  "id": "https://moang.com.br/users/caio",
  "type": "Person",
  "preferredUsername": "caio",
  "name": "Nome exibido",
  "summary": "Bio",
  "url": "https://moang.com.br/about",
  "icon": { "type": "Image", "url": "https://moang.com.br/profile.jpg" },
  "image": { "type": "Image", "url": "https://moang.com.br/banner.jpg" },
  "manuallyApprovesFollowers": false,
  "discoverable": true,
  "indexable": true,
  "inbox": "...",
  "outbox": "...",
  "followers": "...",
  "following": "...",
  "endpoints": { "sharedInbox": "..." },
  "publicKey": { ... }
}
```

Navegadores (`Accept: text/html`) são redirecionados para `/about`.

### `PUT /users/{username}/profile`

Atualização parcial do perfil. Omitir um campo mantém o valor; enviar `""`
remove a imagem.

```json
{
  "display_name": "Caio",
  "summary": "Bio que aparece no Mastodon",
  "avatar_url": "https://moang.com.br/profile.jpg",
  "header_url": "https://moang.com.br/banner.jpg"
}
```

Validação: `display_name` até 64 caracteres, `summary` até 2000, URLs `http`/`https`
até 2048. Resposta: `200` com `name`, `display_name`, `summary`, `avatar_url` e
`header_url`. Federa `Update` do ator.

## Seguindo atores remotos

### `POST /users/{username}/following`

```json
{ "actor": "https://mastodon.social/users/ana" }
```

Busca o ator remoto, grava a relação como pendente e envia `Follow`.
Idempotente se a relação já existe. Resposta: `202` sem corpo.

### `DELETE /users/{username}/following`

Mesmo corpo. Remove a relação e envia `Undo` do `Follow` correspondente.
Resposta: `204` sem corpo.

## Pedidos de seguidores (perfil travado)

Quando `is_locked` é `true`, um `Follow` recebido fica pendente até ser aprovado.

### `GET /users/{username}/follow-requests`

```json
[
  {
    "id": "https://mastodon.social/users/ana",
    "username": "ana",
    "name": "Ana",
    "domain": "mastodon.social",
    "avatarUrl": "https://..."
  }
]
```

### `POST /users/{username}/follow-requests/accept`

```json
{ "actor": "https://mastodon.social/users/ana" }
```

Marca como aceito, envia `Accept` e faz o backfill dos posts existentes.
Resposta: `204`.

### `POST /users/{username}/follow-requests/reject`

Mesmo corpo. Remove o seguidor e envia `Reject`. Resposta: `204`.

## Timeline

### `GET /users/{username}/timeline`

Exige sessão e que a conta seja a do usuário logado. Devolve os posts remotos
de atores seguidos e as respostas a posts locais, mais recentes primeiro, até
100 itens.

```json
{
  "totalItems": 1,
  "orderedItems": [
    {
      "id": "https://mastodon.social/users/ana/statuses/1",
      "url": "https://mastodon.social/@ana/1",
      "content": "<p>HTML</p>",
      "summary": null,
      "published": "2026-11-09T12:00:00+00:00",
      "updated": null,
      "sensitive": false,
      "likes": 3,
      "reposts": 1,
      "replies": 2,
      "inReplyTo": null,
      "actor": {
        "id": "https://mastodon.social/users/ana",
        "username": "ana",
        "name": "Ana",
        "domain": "mastodon.social",
        "avatarUrl": "https://..."
      }
    }
  ]
}
```

## Respostas de um post

### `GET /users/{username}/posts/{ap_id}/replies`

`{ap_id}` é o UUID canônico do objeto (`object.id` termina com ele).

- `Accept: application/json` (app): JSON compacto com `totalItems` e
  `orderedItems` no mesmo formato de `RemotePostView` da timeline.
- `Accept: application/activity+json`: `OrderedCollection` de `Note`s AS2.

## Feed

### `GET /feed.xml`

RSS 2.0 público dos artigos (`kind = "article"`), até 20 itens. Não exige sessão.

## Lacunas conhecidas (antes de construir o app)

1. **Sem logout.** Não há como encerrar a sessão pelo servidor.
2. **Sem `GET /me`.** O app não tem como descobrir o usuário logado; hoje só
   comparando o e-mail do login com o ator.
3. **Sem upload nem anexos.** Posts não suportam mídia (`attachment` é sempre
   vazio). Avatares e banner são apenas URLs externas.
4. **Sem interações de saída.** Não há endpoint para curtir, repostar ou
   responder a um post remoto; o backend só ingere essas atividades.
5. **Sem paginação** no outbox e na timeline (limite de 100 na timeline).
6. **Sem bearer token.** Apenas cookie de sessão.

## Exemplo mínimo

```js
const api = 'https://moang.com.br'

await fetch(`${api}/login`, {
  method: 'POST',
  credentials: 'include',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ email, password }),
})

const response = await fetch(`${api}/users/caio/timeline`, {
  credentials: 'include',
})
const timeline = await response.json()
```
