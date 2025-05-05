import { useEffect, useState } from 'react'
import { HomeContainer } from './styles'
import { LinkToPost } from '../../components/LinkToPost'

interface Post {
  id: number,
  title: string,
  body: string,
}

export function Home() {
  const [posts, setPosts] = useState<Post[]>([])

  useEffect(() => {
    fetch('http://localhost:5000/api/posts').then(response => response.json()).then(data => {
      setPosts(data)
    })
  }, [])
  return (
    <HomeContainer>
      <ul>
        {posts.map(post => <LinkToPost key={post.id} postID={post.id} title={post.title} content={post.body} />)}
      </ul>
    </HomeContainer>
  )
}
