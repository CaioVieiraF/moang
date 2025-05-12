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

  return (
    <LinkContainer onClick={handleSelectPost} to="/posts">
      <h3>#{postID} {title}</h3>
    </LinkContainer>
  )
}
