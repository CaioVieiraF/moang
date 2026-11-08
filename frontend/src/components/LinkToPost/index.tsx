import { LinkContainer } from './styles'

interface LinkProps {
  title: string,
  postID: number
  createdAt: Date
  kind: string
}

export function LinkToPost({ title, postID, createdAt, kind }: LinkProps) {
  const date = new Intl.DateTimeFormat('pt-BR')
  const formatedDate = date.format(new Date(createdAt))

  return (
    <LinkContainer to={'/post/' + postID}>
      <h3>{title}</h3>
      <span className="meta">
        <span className="kind">{kind}</span>
        <small>{formatedDate}</small>
      </span>
    </LinkContainer>
  )
}
