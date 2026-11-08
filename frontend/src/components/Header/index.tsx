import { HeaderContainer, Logo } from './styles'
import { NavLink } from 'react-router-dom'
import { Rss } from 'react-feather'
import { FEED_URL } from '../../lib/blog'

export function Header() {
  return (
    <HeaderContainer>
      <Logo to="/">Moang</Logo>
      <nav>
        <a href={FEED_URL} title="Feed RSS" aria-label="Feed RSS">
          <Rss size={18} />
        </a>
        <NavLink to="/timeline" title="timeline">Timeline</NavLink>
        <NavLink to="/about" title="about">Sobre</NavLink>
      </nav>
    </HeaderContainer>
  )
}
