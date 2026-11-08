import { useEffect, useState } from 'react'
import { PostsContainer, Title } from '../Posts/styles'
import { api } from '../../lib/axios'

interface RemoteActor {
  id: string
  username: string
  name: string
  domain: string
  avatarUrl: string
}

interface RemotePost {
  id: string
  url?: string
  content: string
  summary?: string
  published: string
  sensitive: boolean
  inReplyTo?: string
  actor: RemoteActor
}

interface Timeline {
  totalItems: number
  orderedItems: RemotePost[]
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
      <ul>
        {timeline.orderedItems.map(post => (
          <li key={post.id}>
            <article>
              <header>
                <strong>{post.actor.name || `@${post.actor.username}`}</strong>{' '}
                <small>@{post.actor.username}@{post.actor.domain}</small>
              </header>
              {post.summary && <p>{post.summary}</p>}
              <div dangerouslySetInnerHTML={{ __html: post.content }} />
              {post.url && <a href={post.url}>Ver original</a>}
            </article>
          </li>
        ))}
      </ul>
    </PostsContainer>
  )
}
