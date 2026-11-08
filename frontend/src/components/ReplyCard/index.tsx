import { useState } from 'react'
import { ArrowSquareOutIcon } from '@phosphor-icons/react'
import {
  ReplyAuthor,
  ReplyAvatar,
  ReplyAvatarFallback,
  ReplyBody,
  ReplyCardContainer,
  ReplyFooter,
  ReplyHeader,
  ReplySummary,
} from './styles'

export interface ReplyActor {
  id: string
  username: string
  name: string
  domain: string
  avatarUrl: string
}

export interface Reply {
  id: string
  url?: string
  content: string
  summary?: string
  published: string
  sensitive: boolean
  actor: ReplyActor
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

interface ReplyCardProps {
  reply: Reply
}

export function ReplyCard({ reply }: ReplyCardProps) {
  const [avatarFailed, setAvatarFailed] = useState(false)
  const { actor } = reply
  const displayName = actor.name?.trim() || `@${actor.username}`
  const showAvatar = actor.avatarUrl?.trim() !== '' && !avatarFailed

  return (
    <ReplyCardContainer>
      <ReplyHeader>
        {showAvatar ? (
          <ReplyAvatar
            src={actor.avatarUrl}
            alt=""
            loading="lazy"
            referrerPolicy="no-referrer"
            onError={() => setAvatarFailed(true)}
          />
        ) : (
          <ReplyAvatarFallback aria-hidden="true">
            {initials(actor.name, actor.username)}
          </ReplyAvatarFallback>
        )}

        <ReplyAuthor>
          <strong>{displayName}</strong>
          <span>
            <span className="handle">@{actor.username}@{actor.domain}</span>
            <span className="dot">·</span>
            <time dateTime={reply.published}>{formatDate(reply.published)}</time>
          </span>
        </ReplyAuthor>
      </ReplyHeader>

      {reply.summary && <ReplySummary>{reply.summary}</ReplySummary>}

      <ReplyBody dangerouslySetInnerHTML={{ __html: reply.content }} />

      {reply.url && (
        <ReplyFooter>
          <a href={reply.url} target="_blank" rel="noopener noreferrer">
            Ver original <ArrowSquareOutIcon size={16} weight="bold" />
          </a>
        </ReplyFooter>
      )}
    </ReplyCardContainer>
  )
}
