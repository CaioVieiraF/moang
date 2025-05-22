import { useEffect, useState } from 'react'
import { PostsContainer, Title } from './styles'
import { LinkToPost } from '../../components/LinkToPost'
import { api } from '../../lib/axios'
import { SearchPosts } from '../../components/SearchPosts'

interface Post {
  id: number,
  title: string,
  body: string,
  created_at: string
}

export function Posts() {
  const [posts, setPosts] = useState<Post[]>([])

  async function getPosts() {
    const response = await api.get('posts')
    setPosts(response.data.reverse())
  }

  useEffect(() => {
    getPosts()
  }, [])

  return (
    <PostsContainer>
      <Title>
        <h2>Publicações</h2>
        <small>{posts.length} publicações</small>
      </Title>
      <SearchPosts />
      <ul>
        {posts.map(post => <LinkToPost key={post.id} postID={post.id} title={post.title} createdAt={post.created_at} />)}
      </ul>
    </PostsContainer>
  )
}
