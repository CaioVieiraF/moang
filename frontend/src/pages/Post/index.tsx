import { useCallback, useEffect, useState } from 'react'
import { PostContainer, PostTitle, PostTitleBackLink } from './styles'
import { useParams } from 'react-router-dom'
import { api } from '../../lib/axios'

interface PostData {
  title: string,
  body: string,
}

export function Post() {
  const { id } = useParams()
  const [post, setPost] = useState<PostData>({
    title: '',
    body: '',
  })
  const date = new Intl.DateTimeFormat('pt-BR', {
    year: 'numeric',
    month: 'numeric',
    day: 'numeric',
    hour: 'numeric',
    minute: 'numeric',
    second: 'numeric',
    hour12: false,
  })
  const formatedDate = date.format(new Date())

  const getPost = useCallback(async () => {
    const response = await api.get('/posts/' + id)
    setPost(response.data)
  }, [id])

  useEffect(() => {
    getPost()
  }, [getPost])

  return (
    <PostContainer>
      <PostTitle>
        <PostTitleBackLink to="/posts">{'< Voltar'}</PostTitleBackLink>
        <h1>{post.title}</h1>
        <small>postado em {formatedDate}</small>
      </PostTitle>
      <article>{post.body}</article>
    </PostContainer>
  )
}
