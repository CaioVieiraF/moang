import { FooterContainer } from './styles'
import { GithubLogoIcon, LinkedinLogoIcon, MastodonLogoIcon } from '@phosphor-icons/react'

export function Footer() {
  return (
    <FooterContainer>
      <a rel="me" href="https://mastodon.social/@Smartstein"><MastodonLogoIcon /></a>
      <a href="https://github.com/CaioVieiraF"><GithubLogoIcon /></a>
      <a href="https://www.linkedin.com/in/caio-vieira-2738a3173/"><LinkedinLogoIcon /></a>
    </FooterContainer>
  )
}
