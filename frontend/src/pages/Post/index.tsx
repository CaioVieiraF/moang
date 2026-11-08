import { useCallback, useEffect, useState } from 'react'
import {
  PostContainer,
  PostMeta,
  PostReplies,
  PostRepliesHeader,
  PostRepliesList,
  PostStats,
  PostTitle,
  PostTitleBackLink,
} from './styles'
import { useParams } from 'react-router-dom'
import { api } from '../../lib/axios'
import { PostObject } from '../Posts'
import { Reply, ReplyCard } from '../../components/ReplyCard'
import {
  ChatCircleIcon,
  HeartIcon,
  RepeatIcon,
} from '@phosphor-icons/react'
import Markdown from 'react-markdown'

interface Replies {
  totalItems: number
  orderedItems: Reply[]
}

const PUBLIC_COLLECTION = 'https://www.w3.org/ns/activitystreams#Public'

function visibilityLabel(to: string[] = [], cc: string[] = []) {
  if (to.includes(PUBLIC_COLLECTION)) return 'Público'
  if (cc.includes(PUBLIC_COLLECTION)) return 'Não listado'
  if (to.length > 0) return 'Seguidores'
  return 'Privado'
}

function readingTime(content: string) {
  const words = content.trim().split(/\s+/).filter(Boolean).length
  return Math.max(1, Math.ceil(words / 200))
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
  const kind = post.object.type === 'Note' ? 'Microblog' : 'Artigo'
  const visibility = visibilityLabel(post.object.to, post.object.cc)
  const content = post.object.source?.content ?? post.object.content
  const minutes = readingTime(content)

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

  const likes = post.object.likes?.totalItems ?? 0
  const reposts = post.object.shares?.totalItems ?? 0
  const replyCount = replies.totalItems || post.object.replies?.totalItems || 0

  return (
    <PostContainer>
      <PostTitle>
        <PostTitleBackLink to="/posts">{'< Voltar'}</PostTitleBackLink>
        <h1>{post.name}</h1>
        <PostMeta>
          <span className="kind">{kind}</span>
          <span>{visibility}</span>
          <span className="dot">·</span>
          <time dateTime={post.object.published ?? ''}>postado em {formatedDate}</time>
          {post.object.updated && post.object.updated !== post.object.published && (
            <>
              <span className="dot">·</span>
              <span className="edited" title={post.object.updated}>
                editado
              </span>
            </>
          )}
          {kind === 'Artigo' && (
            <>
              <span className="dot">·</span>
              <span>{minutes} min de leitura</span>
            </>
          )}
        </PostMeta>
      </PostTitle>

      <article>
        <Markdown>{content}</Markdown>
      </article>

      <PostStats>
        <span title={`${likes} curtidas`} aria-label={`${likes} curtidas`}>
          <HeartIcon size={18} weight="bold" />
          {likes}
        </span>
        <span title={`${reposts} reposts`} aria-label={`${reposts} reposts`}>
          <RepeatIcon size={18} weight="bold" />
          {reposts}
        </span>
        <span title={`${replyCount} respostas`} aria-label={`${replyCount} respostas`}>
          <ChatCircleIcon size={18} weight="bold" />
          {replyCount}
        </span>
      </PostStats>

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
