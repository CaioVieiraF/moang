import { useEffect, useMemo, useState } from 'react'
import { PostsContainer, PostsList, StateMessage, Title } from './styles'
import { LinkToPost } from '../../components/LinkToPost'
import { api } from '../../lib/axios'
import { userEndpoint } from '../../lib/blog'
import { SearchPosts } from '../../components/SearchPosts'

export interface PostObject {
  id: string,
  name: string,
  object: {
    id: string
    type?: string
    content: string
    published?: string
    updated?: string
    to?: string[]
    cc?: string[]
    source?: {
      content: string
      mediaType: string
    }
    likes?: {
      totalItems: number
    }
    shares?: {
      totalItems: number
    }
    replies?: {
      totalItems: number
    }
  },
  published: Date
}

interface OrderedCollectionPage {
  totalItems: number,
  orderedItems: PostObject[]
}

type LoadState = 'loading' | 'ready' | 'error'

function normalize(value: string) {
  return value
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
}

export function Posts() {
  const [posts, setPosts] = useState<OrderedCollectionPage>({
    totalItems: 0,
    orderedItems: [],
  })
  const [state, setState] = useState<LoadState>('loading')
  const [query, setQuery] = useState('')

  async function getPosts() {
    setState('loading')
    try {
      const response = await api.get(userEndpoint('/outbox/posts'))
      setPosts(response.data)
      setState('ready')
    } catch {
      setState('error')
    }
  }

  useEffect(() => {
    getPosts()
  }, [])

  const filtered = useMemo(() => {
    const term = normalize(query.trim())
    if (!term) return posts.orderedItems

    return posts.orderedItems.filter(post => {
      const searchable = [
        post.name,
        post.object.source?.content ?? '',
        post.object.content,
      ].join(' ')

      return normalize(searchable).includes(term)
    })
  }, [posts.orderedItems, query])

  const isEmpty = state === 'ready' && filtered.length === 0
  const hasPosts = state === 'ready' && filtered.length > 0

  // The backend orders posts by created_at DESC (newest first), the same
  // order used by the RSS feed. No reverse() here.
  return (
    <PostsContainer>
      <Title>
        <h2>Publicações</h2>
        <small>
          {query.trim()
            ? `${filtered.length} de ${posts.totalItems} publicações`
            : `${posts.totalItems} publicações`}
        </small>
      </Title>

      {state !== 'error' && <SearchPosts onSearch={setQuery} />}

      {state === 'loading' && <StateMessage>Carregando publicações...</StateMessage>}

      {state === 'error' && (
        <StateMessage role="alert">
          Não foi possível carregar as publicações.{' '}
          <button type="button" onClick={getPosts}>Tentar novamente</button>
        </StateMessage>
      )}

      {isEmpty && (
        <StateMessage>
          {query.trim()
            ? 'Nenhuma publicação encontrada para esta busca.'
            : 'Nenhuma publicação ainda.'}
        </StateMessage>
      )}

      {hasPosts && (
        <PostsList>
          {filtered.map(post => {
            const postId = Number(post.id.split('/').pop())
            const kind = post.object.type === 'Note'
              ? 'Microblog'
              : 'Artigo'

            return (
              <LinkToPost
                key={post.id}
                postID={postId}
                title={post.name}
                createdAt={post.published}
                kind={kind}
              />
            )
          })}
        </PostsList>
      )}
    </PostsContainer>
  )
}
