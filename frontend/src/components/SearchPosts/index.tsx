import { Search } from 'react-feather'
import { SearchPostsContainer } from './styles'

export function SearchPosts() {
  return (
    <SearchPostsContainer>
      <input type="text" placeholder="Buscar um post..." />
      <button><Search size={20} /> Buscar</button>
    </SearchPostsContainer>
  )
}
