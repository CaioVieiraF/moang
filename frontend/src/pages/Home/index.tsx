import { Banner } from '../../components/Banner'
import { PostsFeed } from '../../components/PostsFeed'
import { HomeContainer } from './styles'

export function Home() {
  return (
    <HomeContainer>
      <Banner />
      <PostsFeed />
    </HomeContainer>
  )
}
