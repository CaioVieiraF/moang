import { LinkContainer } from './styles'

interface LinkProps {
  title: string,
  postID: number
  createdAt: string
}

export function LinkToPost({ title, postID, createdAt }: LinkProps) {
  const date = new Intl.DateTimeFormat('pt-BR')
  const formatedDate = date.format(new Date(createdAt))

  return (
    <LinkContainer to={'/post/' + postID}>
      <h3>{title}</h3>
      <small>{formatedDate}</small>
    </LinkContainer>
  )
}
