import { useContext } from 'react'
import { PostContainer } from './styles'
import { PostsContext } from '../../layouts/contexts/PostsContext'
import { Link } from 'react-router-dom'

export function Post() {
  const { title, body } = useContext(PostsContext)

  return (
    <PostContainer>
      <div className="titleArea">
        <h1>{title}</h1>
      </div>
      <article>{body}</article>
      <Link to="/">{'< Voltar'}</Link>
    </PostContainer>
  )
}
