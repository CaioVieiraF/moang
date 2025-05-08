import { useEffect, useState } from 'react'
import { HomeContainer, HomePostsContainer, Title } from './styles'
import { LinkToPost } from '../../components/LinkToPost'
import { api } from '../../lib/axios'
import { Banner } from '../../components/Banner'
import { SearchPosts } from '../../components/SearchPosts'

interface Post {
  id: number,
  title: string,
  body: string,
}

export function Home() {
  const [posts, setPosts] = useState<Post[]>([])

  async function getPosts() {
    const response = await api.get('posts')
    setPosts(response.data)
  }

  useEffect(() => {
    getPosts()
  }, [])
  return (
    <HomeContainer>
      <Banner />
      <HomePostsContainer>
        <Title>
          <h2>Publicações</h2>
          <small>{posts.length} publicações</small>
        </Title>
        <SearchPosts />
        <ul>
          {posts.map(post => <LinkToPost key={post.id} postID={post.id} title={post.title} content={post.body} />)}
        </ul>
      </HomePostsContainer>
    </HomeContainer>
  )
}
