import { useEffect, useMemo, useState } from 'react'
import {
  PostsList,
  StateMessage,
  TabButton,
  Tabs,
  Title,
} from './styles'
import { LinkToPost } from '../LinkToPost'
import { SearchPosts } from '../SearchPosts'
import { api } from '../../lib/axios'
import {
  OrderedCollectionPage,
  PostObject,
  userEndpoint,
} from '../../lib/blog'

type LoadState = 'loading' | 'ready' | 'error'
type PostTab = 'articles' | 'notes'

interface TabDefinition {
  id: PostTab
  label: string
  singular: string
  plural: string
  empty: string
  emptySearch: string
}

const TABS: TabDefinition[] = [
  {
    id: 'articles',
    label: 'Artigos',
    singular: 'artigo',
    plural: 'artigos',
    empty: 'Nenhum artigo ainda.',
    emptySearch: 'Nenhum artigo encontrado para esta busca.',
  },
  {
    id: 'notes',
    label: 'Notas',
    singular: 'nota',
    plural: 'notas',
    empty: 'Nenhuma nota ainda.',
    emptySearch: 'Nenhuma nota encontrada para esta busca.',
  },
]

function normalize(value: string) {
  return value
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
}

function isNote(post: PostObject) {
  return post.object.type === 'Note'
}

/** The API returns absolute links; axios only needs the path and query. */
function toApiPath(url: string) {
  const parsed = new URL(url)
  return `${parsed.pathname}${parsed.search}`
}

function countLabel(count: number, tab: TabDefinition) {
  const noun = count === 1
    ? tab.singular
    : tab.plural

  return `${count} ${noun}`
}

/** The publications feed: tabs, search and the list itself. */
export function PostsFeed() {
  const [posts, setPosts] = useState<OrderedCollectionPage>({
    totalItems: 0,
    orderedItems: [],
  })
  const [state, setState] = useState<LoadState>('loading')
  const [query, setQuery] = useState('')
  const [tab, setTab] = useState<PostTab>('articles')

  async function getPosts() {
    setState('loading')
    try {
      const allPosts: PostObject[] = []
      let path = userEndpoint('/outbox/posts')

      // The public feed follows every page: search and the tab counts must
      // cover all posts, not only the first page.
      while (path) {
        const response = await api.get<OrderedCollectionPage>(path)
        allPosts.push(...response.data.orderedItems)
        path = response.data.next
          ? toApiPath(response.data.next)
          : ''
      }

      setPosts({
        totalItems: allPosts.length,
        orderedItems: allPosts,
      })
      setState('ready')
    } catch {
      setState('error')
    }
  }

  useEffect(() => {
    getPosts()
  }, [])

  const activeTab = TABS.find(item => item.id === tab) ?? TABS[0]

  const tabPosts = useMemo(
    () => posts.orderedItems.filter(post => (
      tab === 'notes'
        ? isNote(post)
        : !isNote(post)
    )),
    [posts.orderedItems, tab],
  )

  const filtered = useMemo(() => {
    const term = normalize(query.trim())
    if (!term) return tabPosts

    return tabPosts.filter(post => {
      const searchable = [
        post.name,
        post.object.source?.content ?? '',
        post.object.content,
      ].join(' ')

      return normalize(searchable).includes(term)
    })
  }, [tabPosts, query])

  const isEmpty = state === 'ready' && filtered.length === 0
  const hasPosts = state === 'ready' && filtered.length > 0

  // The backend orders posts by created_at DESC (newest first), the same
  // order used by the RSS feed. No reverse() here.
  return (
    <>
      <Title>
        <Tabs role="tablist" aria-label="Tipo de publicação">
          {TABS.map(item => (
            <TabButton
              key={item.id}
              type="button"
              role="tab"
              aria-selected={tab === item.id}
              $active={tab === item.id}
              onClick={() => setTab(item.id)}
            >
              {item.label}
            </TabButton>
          ))}
        </Tabs>

        <small>
          {query.trim()
            ? `${filtered.length} de ${countLabel(tabPosts.length, activeTab)}`
            : countLabel(tabPosts.length, activeTab)}
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
            ? activeTab.emptySearch
            : activeTab.empty}
        </StateMessage>
      )}

      {hasPosts && (
        <PostsList>
          {filtered.map(post => {
            const postId = Number(post.id.split('/').pop())

            return (
              <LinkToPost
                key={post.id}
                postID={postId}
                title={post.name}
                createdAt={post.published}
              />
            )
          })}
        </PostsList>
      )}
    </>
  )
}
