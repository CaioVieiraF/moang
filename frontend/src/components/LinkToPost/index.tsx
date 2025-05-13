import { LinkContainer } from './styles'
import { useContext } from 'react'
import { PostsContext } from '../../layouts/contexts/PostsContext'

interface LinkProps {
  title: string,
  content: string,
  postID: number
}

export function LinkToPost({ title, content, postID }: LinkProps) {
  const { setPostTitle, setPostBody } = useContext(PostsContext)
  function handleSelectPost() {
    setPostTitle(title)
    setPostBody(content)
  }

  const date = new Intl.DateTimeFormat('pt-BR')
  const formatedDate = date.format(new Date())

  return (
    <LinkContainer onClick={handleSelectPost} to={'/post/' + postID}>
      <h3>{title}</h3>
      <small>{formatedDate}</small>
    </LinkContainer>
  )
}
