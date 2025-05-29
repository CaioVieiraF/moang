import { useCallback, useEffect, useState } from 'react'
import { PostContainer, PostTitle, PostTitleBackLink } from './styles'
import { useParams } from 'react-router-dom'
import { api } from '../../lib/axios'
import { PostObject } from '../Posts'

export function Post() {
  const { id } = useParams()
  const [post, setPost] = useState<PostObject>({
    id: '',
    name: '',
    object: {
      content: '',
    },
    published: new Date(),
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
  const formatedDate = date.format(new Date(post.published))

  const getPost = useCallback(async () => {
    const response = await api.get('outbox/posts/' + id)
    setPost(response.data)
  }, [id])

  useEffect(() => {
    document.title = `Moang blog - ${post.id}`
    getPost()
  }, [getPost, post.id])

  return (
    <PostContainer>
      <PostTitle>
        <PostTitleBackLink to="/posts">{'< Voltar'}</PostTitleBackLink>
        <h1>{post.name}</h1>
        <small>postado em {formatedDate}</small>
      </PostTitle>
      <article>{post.object.content}</article>
    </PostContainer>
  )
}
