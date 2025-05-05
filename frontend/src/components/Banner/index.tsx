import { Linkedin } from 'react-feather'
import { BannerContainer, BannerImg, BannerInfoArea, BannerSocial } from './styles'

export function Banner() {
  const linkedinLink = ''
  const profileImgPath = 'profile.png'
  return (
    <BannerContainer>
      <BannerImg src={profileImgPath} />
      <BannerInfoArea>
        <header>
          <h2>Caio Vieira Fernandes</h2>
          <small>Ciência da computação | Fullstack | Infraestrutura | Linux</small>
        </header>
        <p>
          Desenvolvo software desde os 10 anos
          de idade, e hoje, com 23 anos, já tenho
          mais de 4 anos de experiência no
          mercado de redes, 8 anos estudando e
          criando projetos pessoais e uma
          formação em Ciência da Computação
          pela Universidade Paulista.
        </p>
        <footer>
          <BannerSocial>
            <a href={linkedinLink}>
              <Linkedin size={24} />
            </a>
          </BannerSocial>
        </footer>
      </BannerInfoArea>
    </BannerContainer>
  )
}
