import { useCallback, useEffect, useState } from 'react'
import {
  PostContainer,
  PostReplies,
  PostRepliesHeader,
  PostRepliesList,
  PostTitle,
  PostTitleBackLink,
} from './styles'
import { useParams } from 'react-router-dom'
import { api } from '../../lib/axios'
import { PostObject } from '../Posts'
import { Reply, ReplyCard } from '../../components/ReplyCard'
import Markdown from 'react-markdown'

interface Replies {
  totalItems: number
  orderedItems: Reply[]
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
          <PostRepliesHeader>
            <h2>Respostas</h2>
            <small>
              {replies.totalItems} {replies.totalItems === 1 ? 'resposta' : 'respostas'}
            </small>
          </PostRepliesHeader>
          <PostRepliesList>
            {replies.orderedItems.map(reply => (
              <ReplyCard key={reply.id} reply={reply} />
            ))}
          </PostRepliesList>
        </PostReplies>
      )}
    </PostContainer>
  )
}
