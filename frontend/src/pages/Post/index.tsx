import { useContext } from 'react'
import { PostContainer, PostTitle } from './styles'
import { PostsContext } from '../../layouts/contexts/PostsContext'
import { Link } from 'react-router-dom'

export function Post() {
  const { title, body } = useContext(PostsContext)

  return (
    <PostContainer>
      <PostTitle>
        <Link to="/">{'< Voltar'}</Link>
        <h1>{title}</h1>
      </PostTitle>
      <article>{body}</article>
    </PostContainer>
  )
}
