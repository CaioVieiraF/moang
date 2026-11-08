import { useEffect, useState } from 'react'
import { PostsContainer, Title } from '../Posts/styles'
import { TimelineEmpty, TimelineList } from './styles'
import { api } from '../../lib/axios'
import { Reply, ReplyCard } from '../../components/ReplyCard'

interface Timeline {
  totalItems: number
  orderedItems: Reply[]
}

export function Timeline() {
  const [timeline, setTimeline] = useState<Timeline>({
    totalItems: 0,
    orderedItems: [],
  })

  useEffect(() => {
    api.get('users/caio/timeline').then(response => setTimeline(response.data))
  }, [])

  return (
    <PostsContainer>
      <Title>
        <h2>Timeline</h2>
        <small>{timeline.totalItems} posts remotos</small>
      </Title>

      {timeline.totalItems === 0 ? (
        <TimelineEmpty>Nenhum post remoto recebido ainda.</TimelineEmpty>
      ) : (
        <TimelineList>
          {timeline.orderedItems.map(post => (
            <ReplyCard key={post.id} reply={post} />
          ))}
        </TimelineList>
      )}
    </PostsContainer>
  )
}
