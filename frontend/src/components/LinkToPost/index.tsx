import { ChatCircleIcon, HeartIcon, RepeatIcon } from '@phosphor-icons/react'
import { LinkContainer } from './styles'

interface LinkProps {
  title: string,
  postID: number
  createdAt: Date
  kind: string
  likes: number
  reposts: number
  replies: number
}

export function LinkToPost({
  title,
  postID,
  createdAt,
  kind,
  likes,
  reposts,
  replies,
}: LinkProps) {
  const date = new Intl.DateTimeFormat('pt-BR')
  const formatedDate = date.format(new Date(createdAt))

  return (
    <LinkContainer to={'/post/' + postID}>
      <div className="post">
        <h3>{title}</h3>
        <span className="meta">
          <span className="kind">{kind}</span>
          <small>{formatedDate}</small>
        </span>
      </div>

      <span className="stats">
        <span title={`${likes} curtidas`} aria-label={`${likes} curtidas`}>
          <HeartIcon size={16} weight="bold" /> {likes}
        </span>
        <span title={`${reposts} reposts`} aria-label={`${reposts} reposts`}>
          <RepeatIcon size={16} weight="bold" /> {reposts}
        </span>
        <span title={`${replies} respostas`} aria-label={`${replies} respostas`}>
          <ChatCircleIcon size={16} weight="bold" /> {replies}
        </span>
      </span>
    </LinkContainer>
  )
}
