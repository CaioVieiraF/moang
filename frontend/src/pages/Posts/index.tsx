import { useEffect, useState } from 'react'
import { PostsContainer, Title } from './styles'
import { LinkToPost } from '../../components/LinkToPost'
import { api } from '../../lib/axios'
import { SearchPosts } from '../../components/SearchPosts'

export interface PostObject {
  id: string,
  name: string,
  object: {
    content: string
  },
  published: Date
}

interface OrderedCollectionPage {
  totalItems: number,
  orderedItems: PostObject[]
}

export function Posts() {
  const [posts, setPosts] = useState<OrderedCollectionPage>({
    totalItems: 0,
    orderedItems: [],
  })

  async function getPosts() {
    const response = await api.get('users/caio/outbox')

    setPosts(response.data)
  }

  useEffect(() => {
    getPosts()
  }, [])

  return (
    <PostsContainer>
      <Title>
        <h2>Publicações</h2>
        <small>{posts.totalItems} publicações</small>
      </Title>
      <SearchPosts />
      <ul>
        {posts.orderedItems.map(post => {
          const path = new URL(post.id).pathname.split('/')
          const postId = parseInt(path[path.length - 1])

          return <LinkToPost key={post.id} postID={postId} title={post.name} createdAt={post.published} />
        }).reverse()}
      </ul>
    </PostsContainer>
  )
}
