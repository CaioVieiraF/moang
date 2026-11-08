import { useCallback, useEffect, useState } from 'react'
import { PostContainer, PostReplies, PostTitle, PostTitleBackLink } from './styles'
import { useParams } from 'react-router-dom'
import { api } from '../../lib/axios'
import { PostObject } from '../Posts'
import Markdown from 'react-markdown'

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
  actor: RemoteActor
}

interface Replies {
  totalItems: number
  orderedItems: RemotePost[]
}

export function Post() {
  const { id } = useParams()
  const [post, setPost] = useState<PostObject>({
    id: '',
    name: '',
    object: {
      id: '',
      content: '',
    },
    published: new Date(),
  })
  const [replies, setReplies] = useState<Replies>({
    totalItems: 0,
    orderedItems: [],
  })
  const date = new Intl.DateTimeFormat('pt-BR', {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: 'numeric',
    minute: 'numeric',
    hour12: false,
  })
  const formatedDate = date.format(new Date(post.published))

  const getPost = useCallback(async () => {
    const response = await api.get('users/caio/outbox/posts/' + id)
    setPost(response.data)
  }, [id])

  useEffect(() => {
    document.title = `Moang blog - ${post.name}`
    getPost()
  }, [getPost, post.name])

  useEffect(() => {
    if (!post.object.id) return

    const apId = post.object.id.split('/').pop()
    api
      .get(`users/caio/posts/${apId}/replies`)
      .then(response => setReplies(response.data))
  }, [post.object.id])

  return (
    <PostContainer>
      <PostTitle>
        <PostTitleBackLink to="/posts">{'< Voltar'}</PostTitleBackLink>
        <h1>{post.name}</h1>
        <small>postado em {formatedDate}</small>
      </PostTitle>
      <article>
        <Markdown>{post.object.source?.content ?? post.object.content}</Markdown>
      </article>
      {replies.totalItems > 0 && (
        <PostReplies>
          <h2>Respostas ({replies.totalItems})</h2>
          {replies.orderedItems.map(reply => (
            <div key={reply.id}>
              <header>
                <strong>{reply.actor.name || `@${reply.actor.username}`}</strong>{' '}
                <small>@{reply.actor.username}@{reply.actor.domain}</small>
              </header>
              <div dangerouslySetInnerHTML={{ __html: reply.content }} />
              {reply.url && <a href={reply.url}>Ver original</a>}
            </div>
          ))}
        </PostReplies>
      )}
    </PostContainer>
  )
}
