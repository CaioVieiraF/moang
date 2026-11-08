import { useCallback, useEffect, useState } from 'react'
import { PostsContainer, Title } from '../Posts/styles'
import { TimelineEmpty, TimelineList } from './styles'
import { api } from '../../lib/axios'
import { userEndpoint } from '../../lib/blog'
import { FediversePost, FediversePostCard } from '../../components/FediversePostCard'

interface TimelineData {
  totalItems: number
  orderedItems: FediversePost[]
}

type LoadState = 'loading' | 'ready' | 'error'

export function Timeline() {
  const [timeline, setTimeline] = useState<TimelineData>({
    totalItems: 0,
    orderedItems: [],
  })
  const [state, setState] = useState<LoadState>('loading')

  const load = useCallback(async () => {
    setState('loading')
    try {
      const response = await api.get(userEndpoint('/timeline'))
      setTimeline(response.data)
      setState('ready')
    } catch {
      setState('error')
    }
  }, [])

  useEffect(() => {
    load()
  }, [load])

  const isEmpty = state === 'ready' && timeline.orderedItems.length === 0
  const hasPosts = state === 'ready' && timeline.orderedItems.length > 0

  return (
    <PostsContainer>
      <Title>
        <h2>Timeline</h2>
        <small>{timeline.totalItems} posts remotos</small>
      </Title>

      {state === 'loading' && <TimelineEmpty>Carregando timeline...</TimelineEmpty>}

      {state === 'error' && (
        <TimelineEmpty role="alert">
          Não foi possível carregar a timeline.{' '}
          <button type="button" onClick={load}>Tentar novamente</button>
        </TimelineEmpty>
      )}

      {isEmpty && <TimelineEmpty>Nenhum post remoto recebido ainda.</TimelineEmpty>}

      {hasPosts && (
        <TimelineList>
          {timeline.orderedItems.map(post => (
            <FediversePostCard key={post.id} post={post} />
          ))}
        </TimelineList>
      )}
    </PostsContainer>
  )
}
