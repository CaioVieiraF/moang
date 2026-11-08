import { useState } from 'react'
import {
  ArrowSquareOutIcon,
  ChatCircleIcon,
  HeartIcon,
  RepeatIcon,
} from '@phosphor-icons/react'
import {
  CardAuthor,
  CardAvatar,
  CardAvatarFallback,
  CardBody,
  CardContainer,
  CardFooter,
  CardHeader,
  CardStats,
  CardSummary,
} from './styles'

export interface FediverseActor {
  id: string
  username: string
  name: string
  domain: string
  avatarUrl: string
}

/** A post received from the fediverse: a reply or a timeline entry. */
export interface FediversePost {
  id: string
  url?: string
  content: string
  summary?: string
  published: string
  updated?: string
  sensitive: boolean
  likes: number
  reposts: number
  replies: number
  actor: FediverseActor
}

const dateFormatter = new Intl.DateTimeFormat('pt-BR', {
  day: '2-digit',
  month: 'short',
  year: 'numeric',
  hour: '2-digit',
  minute: '2-digit',
})

function formatDate(value: string) {
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? '' : dateFormatter.format(date)
}

function initials(name: string, username: string) {
  const value = (name || username).trim()
  if (!value) return '?'

  const parts = value.split(/\s+/)
  if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase()

  return `${parts[0][0]}${parts[parts.length - 1][0]}`.toUpperCase()
}

interface FediversePostCardProps {
  post: FediversePost
}

export function FediversePostCard({ post }: FediversePostCardProps) {
  const [avatarFailed, setAvatarFailed] = useState(false)
  const { actor } = post
  const displayName = actor.name?.trim() || `@${actor.username}`
  const showAvatar = actor.avatarUrl?.trim() !== '' && !avatarFailed

  return (
    <CardContainer>
      <CardHeader>
        {showAvatar ? (
          <CardAvatar
            src={actor.avatarUrl}
            alt=""
            loading="lazy"
            referrerPolicy="no-referrer"
            onError={() => setAvatarFailed(true)}
          />
        ) : (
          <CardAvatarFallback aria-hidden="true">
            {initials(actor.name, actor.username)}
          </CardAvatarFallback>
        )}

        <CardAuthor>
          <strong>{displayName}</strong>
          <span>
            <span className="handle">@{actor.username}@{actor.domain}</span>
            <span className="dot">·</span>
            <time dateTime={post.published}>{formatDate(post.published)}</time>
            {post.updated && post.updated !== post.published && (
              <>
                <span className="dot">·</span>
                <span className="edited" title={formatDate(post.updated)}>
                  editado
                </span>
              </>
            )}
          </span>
        </CardAuthor>
      </CardHeader>

      {post.summary && <CardSummary>{post.summary}</CardSummary>}

      <CardBody dangerouslySetInnerHTML={{ __html: post.content }} />

      <CardFooter>
        <CardStats>
          <span title={`${post.likes} curtidas`} aria-label={`${post.likes} curtidas`}>
            <HeartIcon size={16} weight="bold" />
            {post.likes}
          </span>
          <span title={`${post.reposts} reposts`} aria-label={`${post.reposts} reposts`}>
            <RepeatIcon size={16} weight="bold" />
            {post.reposts}
          </span>
          <span title={`${post.replies} respostas`} aria-label={`${post.replies} respostas`}>
            <ChatCircleIcon size={16} weight="bold" />
            {post.replies}
          </span>
        </CardStats>

        {post.url && (
          <a href={post.url} target="_blank" rel="noopener noreferrer">
            Ver original <ArrowSquareOutIcon size={16} weight="bold" />
          </a>
        )}
      </CardFooter>
    </CardContainer>
  )
}
