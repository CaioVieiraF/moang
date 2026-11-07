import { HeaderContainer, Logo } from './styles'
import { Link, NavLink } from 'react-router-dom'
import { Rss } from 'react-feather'

export function Header() {
  return (
    <HeaderContainer>
      <Logo to="/">Moang</Logo>
      <nav>
        <Link to="https://moang.com.br/feed.xml"><Rss size={18} /></Link>
        <NavLink to="/timeline" title="timeline">Timeline</NavLink>
        <NavLink to="/about" title="about">Sobre</NavLink>
      </nav>
    </HeaderContainer>
  )
}
